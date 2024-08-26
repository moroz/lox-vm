use std::process::exit;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Return,
}

pub struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: vec![] }
    }

    pub fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {name} ==");

        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        let instruction = self.code.get(offset).unwrap();

        match FromPrimitive::from_u8(*instruction) {
            Some(OpCode::Return) => {
                return self.simple_instruction("OP_RETURN", offset);
            }
            None => {
                println!("Unknown opcode {instruction}");
                return offset + 1;
            }
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        return offset + 1;
    }
}
