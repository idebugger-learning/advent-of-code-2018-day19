use crate::cpu::CPU;

mod cpu;
mod parser;

fn main() {
    let input = include_str!("./data/input.txt");
    let (leftover, (directives, instructions)) = parser::parse(input).unwrap();
    assert!(leftover.is_empty());

    let mut cpu = CPU::new();
    cpu.load_program(directives.ip_register, instructions);

    while cpu.tick() {
        let registers = cpu.get_registers();
        print!("ip={:2} [{:8}, {:8}, {:8}, {:8}, {:8}, {:8}]\r", cpu.get_ip(), registers[0], registers[1], registers[2], registers[3], registers[4], registers[5]);
    }
    println!();

    println!("Registers after execution: {:?}", cpu.get_registers());
    println!("Sum of divisors of 860: {}", sum_of_divisors(860));
    println!("Sum of divisors of 10551260: {}", sum_of_divisors(10551260));
}

pub fn sum_of_divisors(number: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=number {
        if number % i == 0 {
            sum += i;
        }
    }
    sum
}