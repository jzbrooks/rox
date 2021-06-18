mod bytecode;
mod vm;

use bytecode::Chunk;
use bytecode::OpCode;
use std::env;
use std::fs;
use std::io;
use std::process;
use vm::VM;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    let mut vm = VM::new();

    if argc == 1 {
        repl(vm);
    } else if argc == 2 {
        run_file(vm);
    } else {
        eprintln!("Usage: rox [path]");
        process::exit(74);
    }
    // let chunk = Chunk::new(
    //     vec!(
    //         OpCode::Constant(1),
    //         OpCode::Constant(0),
    //         OpCode::Divide,
    //         OpCode::Negate,
    //         OpCode::Return,
    //     ),
    //     vec!(100.0, 5.0),
    //     vec!(123, 123, 123, 123, 123)
    // );
    // let mut vm = VM::new();
    // vm.interpret(chunk);
}

fn repl(vm: VM) {
    loop {
        print!("> ");

        if let Some(line) = io::stdin().read_line() { 
           // vm.interpret(line);
        } else {
            println!();
            break;
        }

        // vm.interpret(line);
    }
}

fn run_file(vm: VM, path: &str) {
    let source = fs::read_to_string(path)?;
    let result = vm.interpret(source);

    match result {
        InterpretResult::CompileError => process::exit(65);
        InterpretResult::RuntimeError => process::exit(70);
    }
}
