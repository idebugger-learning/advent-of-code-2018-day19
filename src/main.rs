use crate::cpu::CPU;

mod cpu;
mod parser;

fn main() {
    let input = include_str!("./data/input.txt");
    let (leftover, (directives, instructions)) = parser::parse(input).unwrap();
    assert!(leftover.is_empty());

    let mut cpu = CPU::new();
    cpu.load_program(directives.ip_register, instructions);

    while cpu.tick() {}

    println!("Registers after execution: {:?}", cpu.get_registers());
}
