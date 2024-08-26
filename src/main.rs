mod chunk;
mod value;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::Constant as u8);
    chunk.write_chunk(constant);
    chunk.write_chunk(OpCode::Return as u8);
    chunk.disassemble_chunk("test chunk");
}
