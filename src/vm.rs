use crate::bytecode::{Chunk, OpCode, Value};
use crate::compiler::Compiler;

pub struct VM {
    ip: usize,
    stack: Vec<Value>,
    pub output: Option<Value>,
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM {
            ip: 0,
            stack: Vec::new(),
            output: None,
        }
    }

    pub fn interpret_source(&mut self, source: &str) -> InterpretResult {
        let mut compiler = Compiler::new(source);
        let chunk = compiler.compile();
        self.run(&chunk)
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        // Reset the instruction pointer for each run
        self.ip = 0;

        println!("{:?}", self.stack);
        chunk.disassemble("test");

        macro_rules! binop {
			($op:tt) => {
				{
                    if !self.peek(0).is_float() || !self.peek(1).is_float() {
                        self.runtime_error(chunk, "Operands must be a number");
                        return InterpretResult::RuntimeError;
                    }
					let b = self.stack.pop().unwrap().as_float();
					let a = self.stack.pop().unwrap().as_float();
                    self.stack.push(Value::Float(a $op b));
				};
			}
		}

        loop {
            let op: OpCode = unsafe { std::mem::transmute(chunk.code[self.ip]) };
            self.ip += 1;

            match op {
                OpCode::Add => binop!(+),
                OpCode::Subtract => binop!(-),
                OpCode::Multiply => binop!(*),
                OpCode::Divide => binop!(/),
                OpCode::Negate => {
                    let previous = self.stack.pop().unwrap();

                    if !previous.is_float() {
                        self.runtime_error(chunk, &*format!("Cannot negate {}", previous));
                        return InterpretResult::RuntimeError;
                    }

                    let number = previous.as_float();
                    self.stack.push(Value::Float(-number));
                }
                OpCode::Constant => {
                    let constant_index = chunk.code[self.ip] as usize;
                    self.ip += 1;
                    self.stack.push(chunk.constants[constant_index].clone());
                }
                OpCode::Not => {
                    let value = self.stack.pop().unwrap().is_falsey();
                    self.stack.push(Value::Bool(value));
                }
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(Value::Bool(true)),
                OpCode::False => self.stack.push(Value::Bool(false)),
                OpCode::Return => {
                    self.output = self.stack.pop();
                    println!("{}", self.output.as_ref().unwrap().clone());
                    return InterpretResult::Ok;
                }
            }
        }
    }

    fn runtime_error(&self, chunk: &Chunk, message: &str) {
        let ip = self.ip;
        let line = chunk.lines[ip];
        eprintln!("[line {0}] {1}", line, message);
    }

    fn peek(&self, offset: usize) -> &Value {
        let size = self.stack.len();
        &self.stack[size - offset - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytecode::{Chunk, OpCode};

    #[test]
    fn division() {
        let chunk = Chunk::new(
            vec![
                OpCode::Constant as u8,
                0,
                OpCode::Constant as u8,
                1,
                OpCode::Divide as u8,
                OpCode::Return as u8,
            ],
            vec![Value::Float(100.0), Value::Float(5.0)],
            vec![123, 123, 123, 123, 123, 123, 123],
        );
        let mut vm = VM::new();
        vm.run(&chunk);

        assert_eq!(vm.output, Some(Value::Float(20.0)));
    }

    #[test]
    fn negation() {
        let chunk = Chunk::new(
            vec![
                OpCode::Constant as u8,
                0,
                OpCode::Negate as u8,
                OpCode::Return as u8,
            ],
            vec![Value::Float(100.0)],
            vec![123, 123, 123, 123],
        );
        let mut vm = VM::new();
        vm.run(&chunk);

        assert_eq!(vm.output, Some(Value::Float(-100.0)));
    }

    #[test]
    fn invalid_negation() {
        let chunk = Chunk::new(
            vec![
                OpCode::Constant as u8,
                0,
                OpCode::Negate as u8,
                OpCode::Return as u8,
            ],
            vec![Value::Bool(true)],
            vec![123, 123, 123, 123],
        );
        let mut vm = VM::new();
        let result = vm.run(&chunk);

        assert_eq!(result, InterpretResult::RuntimeError);
    }

    #[test]
    fn not() {
        let chunk = Chunk::new(
            vec![OpCode::True as u8, OpCode::Not as u8, OpCode::Return as u8],
            vec![],
            vec![123, 123, 123],
        );
        let mut vm = VM::new();
        vm.run(&chunk);

        assert_eq!(vm.output, Some(Value::Bool(false)));
    }
}
