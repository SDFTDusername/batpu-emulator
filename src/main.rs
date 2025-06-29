use batpu_assembler::assembler::Assembler;
use batpu_assembler::assembler_config::AssemblerConfig;
use batpu_assembly::binary_to_instructions;
use crate::machine::Machine;

mod machine;
mod stack;
mod screen;
mod character_display;
mod number_display;

fn main() {
    let mut assembler = Assembler::new(AssemblerConfig::default());
    assembler.parse("ldi r1 5\nldi r2 10\nstr r1 r2 -1").unwrap();
    
    let machine_code = assembler.assemble().unwrap();
    
    let instructions = binary_to_instructions(&machine_code).unwrap();
    
    let mut machine = Machine::new(instructions);
    machine.reset_and_start();
    
    while machine.running() {
        println!("Tick");
        machine.tick();
    }
    
    println!("Registers:");
    for (i, &register) in machine.registers().iter().enumerate() {
        println!("r{}: {}", i, register);
    }

    println!("Memory:");
    for (i, &memory) in machine.memory().iter().enumerate() {
        println!("{}: {}", i, memory);
    }
}
