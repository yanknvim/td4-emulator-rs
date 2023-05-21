use std::fs::File;
use std::io::{self, BufRead};

pub struct Memory {
    pub memory: Vec<u8>,
}
impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: vec![0; 16]
        }
    }
    
    pub fn load_new(path: &str) -> Memory {
        let mut memory = Memory::new();
        memory.load(path);
        memory
    }

    pub fn load(&mut self, path: &str) {
        let bin_file = File::open(path).unwrap();
        let lines = io::BufReader::new(bin_file).lines();
        for (addr, line) in lines.enumerate() {
            if let Ok(bin_str) = line {
                let bin = u8::from_str_radix(&bin_str, 2).unwrap();
                self.memory[addr] = bin;
            }
        }
    }
}
