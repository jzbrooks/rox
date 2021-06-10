mod bytecode;

use bytecode::Chunk;
use bytecode::OpCode;

fn main() {
    let chunk = Chunk::new(
        vec!(
            OpCode::Constant(0),
            OpCode::Return,
        ),
        vec!(5.0),
    );
    chunk.disassemble("test");
}
