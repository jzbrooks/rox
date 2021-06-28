mod bytecode;
mod compiler;
mod scanner;
mod vm;

use std::{env, fs, io, process};
use vm::{InterpretError, VM};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    let vm = VM::new();

    if argc == 1 {
        repl(vm);
    } else if argc == 2 {
        run_file(vm, &argv[1]);
    } else {
        eprintln!("Usage: rox [path]");
        process::exit(74);
    }
}

fn repl(mut vm: VM) {
    loop {
        print!("> ");

        let mut line = String::new();
        if let io::Result::Ok(_) = io::stdin().read_line(&mut line) {
            let result = vm.interpret_source(&line);

            if let Ok(value) = result {
                println!(">>> {}", value);
            }
        } else {
            break;
        }
    }
}

fn run_file(mut vm: VM, path: &str) {
    let source = fs::read_to_string(path).unwrap();
    let result = vm.interpret_source(&source);

    match result {
        Err(InterpretError::Compile) => process::exit(65),
        Err(InterpretError::Runtime) => process::exit(70),
        Ok(value) => {
            println!(">>> {}", value);
        }
    }
}
