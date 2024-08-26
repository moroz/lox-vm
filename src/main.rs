mod chunk;

fn main() {
    let mut chunk = chunk::Chunk::new();
    chunk.write_chunk(chunk::OpCode::Return as u8);
    chunk.disassemble_chunk("test chunk");
}
