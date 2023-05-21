mod opcode;
mod memory;
mod register;
mod port;
mod cpu;

use cpu::*;

fn main() {
    let mut cpu = CPU::new("test.bin");
    for i in 0..100 {
        let (opcode, operand) = cpu.fetch();
        cpu.execute(&opcode, operand);
        println!("{:<2} {:>4b}", i, cpu.port.output);
    }
}
