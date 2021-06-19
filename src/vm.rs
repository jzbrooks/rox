use crate::bytecode::Value;
use crate::scanner::Scanner;
use crate::Chunk;
use crate::OpCode;

#[derive(Debug)]
pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,
    stack: Vec<Value>,
}

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: None,
            ip: 0,
            stack: Vec::<Value>::new(),
        }
    }

    fn compile(source: &str) -> Option<Chunk> {
        let mut scanner = Scanner::new(source);
        let line = -1;
        loop {
            let token = scanner.next();
        }
        None
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    pub fn interpret_source(&mut self, source: &str) -> InterpretResult {
        let chunk = VM::compile(source);
        self.interpret(chunk.unwrap())
    }

    fn run(&mut self) -> InterpretResult {
        let chunk = &self.chunk.as_ref().unwrap();
        println!("{:?}", self.stack);
        chunk.disassemble("test");

        macro_rules! binop {
			($op:tt) => {
				{
					let a = self.stack.pop().unwrap();
					let b = self.stack.pop().unwrap();
					let c = a $op b;
					self.stack.push(c);
				}
			}
		}

        loop {
            let op = &chunk.code[self.ip];
            self.ip += 1;

            match op {
                OpCode::Add => binop!(+),
                OpCode::Subtract => binop!(-),
                OpCode::Multiply => binop!(*),
                OpCode::Divide => binop!(/),
                OpCode::Negate => {
                    let negation = -self.stack.pop().unwrap();
                    self.stack.push(negation);
                }
                OpCode::Constant(value) => {
                    self.stack.push(chunk.constants[*value]);
                }
                OpCode::Return => {
                    println!("{:?}", self.stack.pop().unwrap());
                    return InterpretResult::Ok;
                }
            }
        }
    }
}
