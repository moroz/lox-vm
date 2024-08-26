use crate::value;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Return,
}

pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: Vec<value::Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            lines: vec![],
            constants: vec![],
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: value::Value) -> u8 {
        self.constants.push(value);
        return (self.constants.len() - 1) as u8;
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
        if offset > 0 && self.lines.get(offset) == self.lines.get(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines.get(offset).unwrap());
        }

        let instruction = self.code.get(offset).unwrap();

        match FromPrimitive::from_u8(*instruction) {
            Some(OpCode::Return) => {
                return self.simple_instruction("OP_RETURN", offset);
            }
            Some(OpCode::Constant) => {
                return self.constant_instruction("OP_CONSTANT", offset);
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

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code.get(offset + 1).unwrap();
        let value = self.constants.get(*constant as usize).unwrap();
        println!("{:<16} {:4} '{value}'", name, constant);
        return offset + 2;
    }
}
