mod bytecode;
mod scanner;
mod vm;

use bytecode::Chunk;
use bytecode::OpCode;
use scanner::Scanner;
use scanner::TokenKind;
use std::env;
use std::fs;
use std::io;
use std::process;
use vm::InterpretResult;
use vm::VM;

fn main() {
    let mut scanner = Scanner::new("{}  ; .!=");
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    let mut current = scanner.next();
    println!("{:?}", scanner.next());
    while current.kind != TokenKind::End {
        current = scanner.next();
        println!("{:?}", current);
    }

    let mut vm = VM::new();

    if argc == 1 {
        repl(vm);
    } else if argc == 2 {
        run_file(vm, &argv[1]);
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

fn repl(mut vm: VM) {
    loop {
        print!("> ");

        let mut line = String::new();
        if let io::Result::Ok(_) = io::stdin().read_line(&mut line) {
            vm.interpret_source(&line);
        } else {
            break;
        }
    }
}

fn run_file(mut vm: VM, path: &str) {
    let source = fs::read_to_string(path).unwrap();
    let result = vm.interpret_source(&source);

    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        InterpretResult::Ok => {}
    }
}
