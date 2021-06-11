mod bytecode;
mod vm;

use bytecode::Chunk;
use bytecode::OpCode;
use vm::VM;

fn main() {
    let chunk = Chunk::new(
        vec!(
            OpCode::Constant(0),
            OpCode::Return,
        ),
        vec!(5.0),
        vec!(123, 123)
    );
    chunk.disassemble("test");
    let mut vm = VM::new();
    println!("Interpret result: {:?}", vm.interpret(chunk));
}
