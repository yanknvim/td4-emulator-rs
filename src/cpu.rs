use std::process::exit;

use crate::memory::*;
use crate::opcode::*;
use crate::port::*;
use crate::register::*;

pub struct CPU {
    pub memory: Memory,
    pub register: Register,
    pub port: Port,
}

impl CPU {
    pub fn new(path: &str) -> Self {
        CPU {
            memory: Memory::load_new(path),
            register: Register::new(),
            port: Port::new(),
        }
    }

    pub fn fetch(&self) -> (Opcode, u8) {
        let pc = self.register.pc;
        let operation = self.memory.memory[pc as usize];

        let opcode = num_traits::FromPrimitive::from_u8(operation >> 4).unwrap();
        let operand = operation & 0b1111;
        (opcode, operand)
    }

    pub fn execute(&mut self, opcode: &Opcode, operand: u8) {
        match opcode {
            Opcode::AddA => {
                let tmp = self.register.register_a + operand;
                self.register.carry = (0b10000 & tmp) != 0;
                self.register.register_a =  0b1111 & tmp;
            }
            Opcode::AddB => {
                let tmp = self.register.register_b + operand;
                self.register.carry = (0b10000 & tmp) != 0;
                self.register.register_b =  0b1111 & tmp;
            }
            Opcode::MovA => {
                self.register.register_a = operand;
                self.register.carry = false;
            }
            Opcode::MovB => {
                self.register.register_b = operand;
                self.register.carry = false;
            }
            Opcode::MovAB => {
                self.register.register_a = self.register.register_b;
                self.register.carry = false;
            }
            Opcode::MovBA => {
                self.register.register_b = self.register.register_a;
                self.register.carry = false;
            }
            Opcode::Jmp => {
                self.register.pc = operand;
                self.register.carry = false;
                return;
            }
            Opcode::Jnc => {
                if self.register.carry {
                    self.register.pc = operand;
                    self.register.carry = false;
                    return;
                } 
            }
            Opcode::InA => {
                self.register.register_a = self.port.input;
                self.register.carry = false;
            }
            Opcode::InB => {
                self.register.register_b = self.port.input;
                self.register.carry = false;
            }
            Opcode::OutB => {
                self.port.output = self.register.register_b;
                self.register.carry = false;
            }
            Opcode::OutIm => {
                self.port.output = operand;
                self.register.carry = false;
            }
            Opcode::Brk => {
                exit(0);
            }
        }
        self.register.pc = (self.register.pc + 1) & 0b1111;
    }

}
