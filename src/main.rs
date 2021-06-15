mod bytecode;
mod vm;

use bytecode::Chunk;
use bytecode::OpCode;
use vm::VM;

fn main() {
    let chunk = Chunk::new(
        vec!(
            OpCode::Constant(0),
            OpCode::Negate,
            OpCode::Return,
        ),
        vec!(5.0),
        vec!(123, 123, 123)
    );
    let mut vm = VM::new();
    vm.interpret(chunk);
}
