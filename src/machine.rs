use crate::components::character_display::CharacterDisplay;
use crate::components::controller::Controller;
use crate::components::number_display::NumberDisplay;
use crate::components::screen::Screen;
use crate::components::stack::Stack;
use batpu_assembly::components::address;
use batpu_assembly::components::condition::Condition;
use batpu_assembly::components::immediate;
use batpu_assembly::components::location::Location;
use batpu_assembly::components::register::Register;
use batpu_assembly::instruction::Instruction;
use batpu_assembly::InstructionVec;
use rand::rngs::ThreadRng;
use rand::{rng, Rng};

pub const CHARACTERS: &[char] = &[' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '.', '!', '?'];

pub const PORTS: usize = 16;
pub const REGISTER_COUNT: usize = 16;
pub const MEMORY_SIZE: usize = 256;
pub const USABLE_MEMORY_SIZE: usize = MEMORY_SIZE - PORTS;
pub const PORTS_ADDRESS: usize = MEMORY_SIZE - PORTS;

pub type Word = u8;

pub struct Machine {
    rng: ThreadRng,

    program_counter: u32,
    halt: bool,

    registers: [Word; REGISTER_COUNT],
    memory: [Word; USABLE_MEMORY_SIZE],
    stack: Stack,
    
    registers_updated: bool,
    memory_updated: bool,
    
    zero_flag: bool,
    carry_flag: bool,
    
    flags_updated: bool,

    screen: Screen,
    character_display: CharacterDisplay,
    number_display: NumberDisplay,
    controller: Controller,

    instructions: InstructionVec
}

impl Machine {
    pub fn new() -> Self {
        Self {
            rng: rng(),

            program_counter: 0,
            halt: false,

            registers: [0; REGISTER_COUNT],
            memory: [0; USABLE_MEMORY_SIZE],
            stack: Stack::new(16),
            
            registers_updated: true,
            memory_updated: true,
            
            zero_flag: false,
            carry_flag: false,
            
            flags_updated: true,

            screen: Screen::new(32, 32),
            character_display: CharacterDisplay::new(10),
            number_display: NumberDisplay::new(),
            controller: Controller::new(),

            instructions: Vec::new()
        }
    }
    
    pub fn reset(&mut self) {
        self.registers.fill(0);
        self.memory.fill(0);
        self.stack.clear();
        
        self.registers_updated = true;
        self.memory_updated = true;
        
        self.zero_flag = false;
        self.carry_flag = false;
        
        self.flags_updated = true;

        self.screen.clear();
        self.character_display.clear();
        self.number_display.clear();
        self.controller.clear();
        
        self.program_counter = 0;
    }
    
    pub fn set_instructions(&mut self, instructions: InstructionVec) {
        self.instructions = instructions;
    }

    pub fn tick(&mut self) {
        if self.program_counter >= self.instructions.len() as u32 {
            self.program_counter = (self.program_counter + 1).rem_euclid(address::MAX_POSSIBLE_COUNT);
            return;
        }

        let instruction = self.instructions[self.program_counter as usize].clone();
        self.run_instruction(&instruction);
    }
    
    fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::NoOperation => {},
            Instruction::Halt => {
                self.halt = true;
                self.program_counter = 0;
                return;
            },
            Instruction::Addition(a, b, c) => {
                let (result, borrow) = self.reg(&a).overflowing_add(self.reg(&b));

                self.set_carry_flag(borrow);
                self.set_zero_flag(result == 0);

                self.set_reg(
                    &c,
                    result
                );
            },
            Instruction::Subtraction(a, b, c) => {
                let (result, borrow) = self.reg(&a).overflowing_sub(self.reg(&b));

                self.set_carry_flag(!borrow);
                self.set_zero_flag(result == 0);

                self.set_reg(
                    &c,
                    result
                );
            },
            Instruction::BitwiseNOR(a, b, c) => {
                let result = !(self.reg(&a) | self.reg(&b));

                self.set_zero_flag(result == 0);
                self.set_carry_flag(false);

                self.set_reg(
                    &c,
                    result
                );
            },
            Instruction::BitwiseAND(a, b, c) => {
                let result = self.reg(&a) & self.reg(&b);

                self.set_zero_flag(result == 0);
                self.set_carry_flag(false);

                self.set_reg(
                    &c,
                    result
                );
            },
            Instruction::BitwiseXOR(a, b, c) => {
                let result = self.reg(&a) ^ self.reg(&b);

                self.set_zero_flag(result == 0);
                self.set_carry_flag(false);

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
                    immediate.immediate() as u8
                );
            },
            Instruction::AddImmediate(a, immediate) => {
                let (result, borrow) = self.reg(&a).overflowing_add(immediate.immediate() as Word);

                self.set_carry_flag(borrow);
                self.set_zero_flag(result == 0);

                self.set_reg(
                    &a,
                    result
                );
            },
            Instruction::Jump(location) => {
                match location {
                    Location::Address(address) => {
                        self.program_counter = address.address();
                        return;
                    },
                    Location::Offset(_) => {
                        panic!("Attempted to jump to an offset");
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
                            self.program_counter = address.address();
                            return;
                        },
                        Location::Offset(_) => {
                            panic!("Attempted to jump to an offset");
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
                        self.stack.push((self.program_counter + 1).rem_euclid(address::MAX_POSSIBLE_COUNT));
                        self.program_counter = address.address();
                        return;
                    },
                    Location::Offset(_) => {
                        panic!("Attempted to jump to an offset");
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
                let mem = self.mem(self.reg(&a) as i32 + offset.offset());

                self.set_reg(
                    &b,
                    mem
                );
            },
            Instruction::MemoryStore(a, b, offset) => {
                self.set_mem(
                    self.reg(&a) as i32 + offset.offset(),
                    self.reg(&b)
                );
            }
        }

        self.program_counter += 1;
    }

    pub fn program_counter(&self) -> u32 {
        self.program_counter
    }
    
    pub fn set_program_counter(&mut self, program_counter: u32) {
        self.program_counter = program_counter
    }
    
    pub fn halt(&self) -> bool {
        self.halt
    }
    
    pub fn set_halt(&mut self, halt: bool) {
        self.halt = halt;
    }
    
    pub fn zero_flag(&self) -> bool {
        self.zero_flag
    }
    
    pub fn set_zero_flag(&mut self, zero_flag: bool) {
        if zero_flag != self.zero_flag {
            self.flags_updated = true;
        }
        
        self.zero_flag = zero_flag;
    }
    
    pub fn carry_flag(&self) -> bool {
        self.carry_flag
    }
    
    pub fn set_carry_flag(&mut self, carry_flag: bool) {
        if carry_flag != self.carry_flag {
            self.flags_updated = true;
        }
        
        self.carry_flag = carry_flag;
    }

    pub fn registers(&self) -> &[Word] {
        &self.registers
    }
    
    pub fn registers_mut(&mut self) -> &mut [Word] {
        &mut self.registers
    }

    pub fn memory(&self) -> &[Word] {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut [Word] {
        &mut self.memory
    }
    
    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }
    
    pub fn screen(&self) -> &Screen {
        &self.screen
    }
    
    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }

    pub fn character_display(&self) -> &CharacterDisplay {
        &self.character_display
    }

    pub fn character_display_mut(&mut self) -> &mut CharacterDisplay {
        &mut self.character_display
    }

    pub fn number_display(&self) -> &NumberDisplay {
        &self.number_display
    }

    pub fn number_display_mut(&mut self) -> &mut NumberDisplay {
        &mut self.number_display
    }
    
    pub fn controller(&self) -> &Controller {
        &self.controller
    }
    
    pub fn controller_mut(&mut self) -> &mut Controller {
        &mut self.controller
    }

    pub fn registers_updated(&self) -> bool {
        self.registers_updated
    }

    pub fn disable_registers_updated(&mut self) {
        self.registers_updated = false;
    }

    pub fn memory_updated(&self) -> bool {
        self.memory_updated
    }

    pub fn disable_memory_updated(&mut self) {
        self.memory_updated = false;
    }

    pub fn flags_updated(&self) -> bool {
        self.flags_updated
    }

    pub fn disable_flags_updated(&mut self) {
        self.flags_updated = false;
    }

    fn reg(&self, register: &Register) -> Word {
        let register = register.register();
        self.registers[register as usize]
    }

    fn set_reg(&mut self, register: &Register, value: Word) {
        let register = register.register();
        if register == 0 {
            return;
        }
        
        self.registers[register as usize] = value;
        self.registers_updated = true;
    }

    fn mem(&mut self, address: i32) -> Word {
        let address = address.rem_euclid(immediate::MAX_POSSIBLE_COUNT as i32) as usize;
        
        if address >= PORTS_ADDRESS {
            return match address - PORTS_ADDRESS {
                0  => 0,
                1  => 0,
                2  => 0,
                3  => 0,
                4  => if self.screen.pix() { 1 } else { 0 },
                5  => 0,
                6  => 0,
                7  => 0,
                8  => 0,
                9  => 0,
                10 => 0,
                11 => 0,
                12 => 0,
                13 => 0,
                14 => self.rng.random(),
                15 => self.controller.binary(),
                _ => panic!("I/O address {} not implemented", address)
            }
        }
        
        self.memory[address]
    }

    fn set_mem(&mut self, address: i32, value: Word) {
        let address = address.rem_euclid(immediate::MAX_POSSIBLE_COUNT as i32) as usize;
        
        if address >= PORTS_ADDRESS {
            match address - PORTS_ADDRESS {
                0  => self.screen.x = value as isize,
                1  => self.screen.y = value as isize,
                2  => self.screen.set_pix(true),
                3  => self.screen.set_pix(false),
                4  => (),
                5  => self.screen.push_buffer(),
                6  => self.screen.clear_buffer(),
                7  => { self.character_display.push(CHARACTERS.get(value as usize)); },
                8  => self.character_display.push_buffer(),
                9  => self.character_display.clear_buffer(),
                10 => self.number_display.set_value(value),
                11 => self.number_display.clear(),
                12 => self.number_display.signed = true,
                13 => self.number_display.signed = false,
                14 => (),
                15 => (),
                _ => panic!("I/O address {} not implemented", address)
            }

            return;
        }

        self.memory[address] = value;
        self.memory_updated = true;
    }
}