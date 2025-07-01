use crate::components::character_display::CharacterDisplay;
use crate::components::number_display::NumberDisplay;
use crate::components::screen::Screen;
use crate::stack::Stack;
use batpu_assembly::components::condition::Condition;
use batpu_assembly::components::location::Location;
use batpu_assembly::components::register::Register;
use batpu_assembly::instruction::Instruction;
use batpu_assembly::InstructionVec;
use rand::rngs::ThreadRng;
use rand::{rng, Rng};

const CHARACTERS: &[char] = &[' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '.', '!', '?'];

pub struct Machine {
    rng: ThreadRng,

    program_counter: usize,
    running: bool,

    registers: [u8; 16],
    memory: [u8; 240], // Memory Size (256) - Port Count (16) = 240
    stack: Stack,
    
    zero_flag: bool,
    carry_flag: bool,

    screen: Screen,
    character_display: CharacterDisplay,
    number_display: NumberDisplay,

    instructions: InstructionVec
}

impl Machine {
    pub fn new(instructions: InstructionVec) -> Self {
        Self {
            rng: rng(),

            program_counter: 0,
            running: false,

            registers: [0; 16],
            memory: [0; 240],
            stack: Stack::new(16),
            
            zero_flag: false,
            carry_flag: false,

            screen: Screen::new(32, 32),
            character_display: CharacterDisplay::new(10),
            number_display: NumberDisplay::new(),

            instructions
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }
    
    pub fn start(&mut self) {
        self.registers.fill(0);
        self.memory.fill(0);
        self.stack.clear();
        
        self.zero_flag = false;
        self.carry_flag = false;

        self.screen.clear();
        self.character_display.clear();
        self.number_display.clear();
        
        self.program_counter = 0;
        self.running = true;
    }
    

    pub fn tick(&mut self) {
        if !self.running {
            return;
        }
        
        if self.program_counter >= self.instructions.len() {
            self.running = false;
            return;
        }

        let instruction = self.instructions[self.program_counter].clone();

        match instruction {
            Instruction::Halt => {
                self.running = false;
                return;
            },
            Instruction::Addition(a, b, c) => {
                let result = self.reg(&a) as u16 + self.reg(&b) as u16;
                self.carry_flag = result > 255;
                
                let result_byte = result as u8;
                self.zero_flag = result == 0;
                
                self.set_reg(
                    &c,
                    result_byte
                );
            },
            Instruction::Subtraction(a, b, c) => {
                let result = self.reg(&a) as i16 - self.reg(&b) as i16;
                self.carry_flag = result >= 0;

                let result_byte = result as u8;
                self.zero_flag = result == 0;

                self.set_reg(
                    &c,
                    result_byte
                );
            },
            Instruction::BitwiseNOR(a, b, c) => {
                let result = !(self.reg(&a) | self.reg(&b));
                
                self.zero_flag = result == 0;
                self.carry_flag = false;
                
                self.set_reg(
                    &c,
                    result
                );
            },
            Instruction::BitwiseAND(a, b, c) => {
                let result = self.reg(&a) & self.reg(&b);

                self.zero_flag = result == 0;
                self.carry_flag = false;

                self.set_reg(
                    &c,
                    result
                );
            },
            Instruction::BitwiseXOR(a, b, c) => {
                let result = self.reg(&a) ^ self.reg(&b);

                self.zero_flag = result == 0;
                self.carry_flag = false;

                self.set_reg(
                    &c,
                    result
                );
            },
            Instruction::RightShift(a, c) => {
                self.set_reg(
                    &c,
                    self.reg(&a) >> 1
                );
            },
            Instruction::LoadImmediate(a, immediate) => {
                self.set_reg(
                    &a,
                    immediate.immediate()
                );
            },
            Instruction::AddImmediate(a, immediate) => {
                let result = self.reg(&a) as usize + immediate.immediate() as usize;
                self.carry_flag = result > 255;
                
                let result_byte = result as u8;
                self.zero_flag = result == 0;
                
                self.set_reg(
                    &a,
                    result_byte
                );
            },
            Instruction::Jump(location) => {
                match location {
                    Location::Address(address) => {
                        self.program_counter = address.address() as usize;
                        return;
                    },
                    Location::Label(_) => {
                        panic!("Attempted to jump to a label");
                    }
                }
            },
            Instruction::Branch(condition, location) => {
                let condition_met = match condition {
                    Condition::Zero     => self.zero_flag,
                    Condition::NotZero  => !self.zero_flag,
                    Condition::Carry    => self.carry_flag,
                    Condition::NotCarry => !self.carry_flag
                };
                
                if condition_met {
                    match location {
                        Location::Address(address) => {
                            self.program_counter = address.address() as usize;
                            return;
                        },
                        Location::Label(_) => {
                            panic!("Attempted to jump to a label");
                        }
                    }
                }
            }
            Instruction::Call(location) => {
                match location {
                    Location::Address(address) => {
                        self.stack.push(self.program_counter + 1);
                        self.program_counter = address.address() as usize;
                        return;
                    },
                    Location::Label(_) => {
                        panic!("Attempted to jump to a label");
                    }
                }
            },
            Instruction::Return => {
                let result = self.stack.pop();
                match result {
                    Some(address) => {
                        self.program_counter = address;
                    },
                    None => {}
                }
                return;
            },
            Instruction::MemoryLoad(a, b, offset) => {
                let mem = self.mem(self.reg(&a) as isize + offset.offset() as isize);

                self.set_reg(
                    &b,
                    mem
                );
            },
            Instruction::MemoryStore(a, b, offset) => {
                self.set_mem(
                    self.reg(&a) as isize + offset.offset() as isize,
                    self.reg(&b)
                );
            },
            _ => {
                panic!("Unknown instruction \"{:?}\"", instruction)
            }
        }

        self.program_counter += 1;
    }
    
    pub fn registers(&self) -> &[u8] {
        &self.registers
    }

    pub fn program_counter(&self) -> usize {
        self.program_counter
    }

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }
    
    pub fn zero_flag(&self) -> bool {
        self.zero_flag
    }
    
    pub fn carry_flag(&self) -> bool {
        self.carry_flag
    }
    
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn character_display(&self) -> &CharacterDisplay {
        &self.character_display
    }

    pub fn number_display(&self) -> &NumberDisplay {
        &self.number_display
    }

    fn reg(&self, register: &Register) -> u8 {
        let register = register.register();
        if register == 0 {
            return 0;
        }
        
        self.registers[register as usize]
    }

    fn set_reg(&mut self, register: &Register, value: u8) {
        let register = register.register();
        if register == 0 {
            return;
        }
        
        self.registers[register as usize] = value;
    }

    fn mem(&mut self, address: isize) -> u8 {
        let address = address.rem_euclid(256) as usize;
        
        if address >= 240 {
            return match address {
                240 => 0,
                241 => 0,
                242 => 0,
                243 => 0,
                244 => if self.screen.pix() { 1 } else { 0 },
                245 => 0,
                246 => 0,
                247 => 0,
                248 => 0,
                249 => 0,
                250 => 0,
                251 => 0,
                252 => 0,
                253 => 0,
                254 => self.rng.random(),
                255 => 0,
                _ => panic!("I/O address {} not implemented", address)
            }
        }
        
        self.memory[address]
    }

    fn set_mem(&mut self, address: isize, value: u8) {
        let address = address.rem_euclid(256) as usize;
        
        if address >= 240 {
            match address {
                240 => self.screen.x = value as isize,
                241 => self.screen.y = value as isize,
                242 => self.screen.set_pix(true),
                243 => self.screen.set_pix(false),
                244 => (),
                245 => self.screen.push_buffer(),
                246 => self.screen.clear_buffer(),
                247 => { self.character_display.push(CHARACTERS.get(value as usize)); },
                248 => self.character_display.push_buffer(),
                249 => self.character_display.clear_buffer(),
                250 => self.number_display.set_value(value),
                251 => self.number_display.clear(),
                252 => self.number_display.signed = true,
                253 => self.number_display.signed = false,
                254 => (),
                255 => (),
                _ => panic!("I/O address {} not implemented", address)
            }

            return;
        }

        self.memory[address] = value;
    }
}