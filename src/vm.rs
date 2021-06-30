use crate::bytecode::{Chunk, OpCode, Value};
use crate::compiler::Compiler;

pub struct VM {
    ip: usize,
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum InterpretError {
    Compile,
    Runtime,
}

impl std::fmt::Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InterpretError::Compile => write!(f, "Compile"),
            InterpretError::Runtime => write!(f, "Runtime"),
        }
    }
}

impl std::error::Error for InterpretError {}

impl VM {
    pub fn new() -> VM {
        VM {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret_source(&mut self, source: &str) -> Result<Value, InterpretError> {
        let mut compiler = Compiler::new(source);
        let chunk = compiler.compile();
        self.run(&chunk)
    }

    fn run(&mut self, chunk: &Chunk) -> Result<Value, InterpretError> {
        // Reset the instruction pointer for each run
        self.ip = 0;

        println!("{:?}", self.stack);
        chunk.disassemble("test");

        macro_rules! binop_float {
			($op:tt) => {
				{
                    if !self.peek(0).is_float() || !self.peek(1).is_float() {
                        self.runtime_error(chunk, "Operands must be a number");
                        return Err(InterpretError::Runtime);
                    }
					let b = self.stack.pop().unwrap().as_float();
					let a = self.stack.pop().unwrap().as_float();
                    self.stack.push(Value::Float(a $op b));
				};
			}
		}

        macro_rules! binop_bool {
            ($op:tt) => {
                {
                    if !self.peek(0).is_float() || !self.peek(1).is_float() {
                        self.runtime_error(chunk, "Operands must be a number");
                        return Err(InterpretError::Runtime);
                    }
                    let b = self.stack.pop().unwrap().as_float();
                    let a = self.stack.pop().unwrap().as_float();
                    self.stack.push(Value::Bool(a $op b));
                };
            }
        }

        loop {
            let op: OpCode = unsafe { std::mem::transmute(chunk.code[self.ip]) };
            self.ip += 1;

            match op {
                OpCode::Add => {
                    if self.peek(0).is_float() || self.peek(1).is_float() {
                        let b = self.stack.pop().unwrap().as_float();
                        let a = self.stack.pop().unwrap().as_float();
                        self.stack.push(Value::Float(a + b));
                    } else if self.peek(0).is_str() || self.peek(1).is_str() {
                        let b = self.stack.pop();
                        let a = self.stack.pop();
                        self.stack.push(Value::Str(
                            a.unwrap().as_str().to_owned() + b.unwrap().as_str(),
                        ));
                    } else {
                        self.runtime_error(chunk, "Operands must be two numbers or two strings.");
                        return Err(InterpretError::Runtime);
                    }
                }
                OpCode::Subtract => binop_float!(-),
                OpCode::Multiply => binop_float!(*),
                OpCode::Divide => binop_float!(/),
                OpCode::Negate => {
                    let previous = self.stack.pop().unwrap();

                    if !previous.is_float() {
                        self.runtime_error(chunk, &*format!("Cannot negate {}", previous));
                        return Err(InterpretError::Runtime);
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
                OpCode::Equal => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(Value::Bool(b == a));
                }
                OpCode::Less => binop_bool!(<),
                OpCode::Greater => binop_bool!(>),
                OpCode::Return => {
                    let result = self.stack.pop().as_ref().unwrap().clone();
                    return Ok(result);
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
    fn division() -> Result<(), InterpretError> {
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
        let result = vm.run(&chunk)?;

        assert_eq!(result, Value::Float(20.0));

        Ok(())
    }

    #[test]
    fn negation() -> Result<(), InterpretError> {
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
        let result = vm.run(&chunk)?;

        assert_eq!(result, Value::Float(-100.0));

        Ok(())
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

        assert_eq!(result, Err(InterpretError::Runtime));
    }

    #[test]
    fn not() -> Result<(), InterpretError> {
        let chunk = Chunk::new(
            vec![OpCode::True as u8, OpCode::Not as u8, OpCode::Return as u8],
            vec![],
            vec![123, 123, 123],
        );
        let mut vm = VM::new();
        let result = vm.run(&chunk)?;

        assert_eq!(result, Value::Bool(false));

        Ok(())
    }

    #[test]
    fn less() -> Result<(), InterpretError> {
        let chunk = Chunk::new(
            vec![
                OpCode::Constant as u8,
                0,
                OpCode::Constant as u8,
                1,
                OpCode::Less as u8,
                OpCode::Return as u8,
            ],
            vec![Value::Float(-1.0), Value::Float(1.0)],
            vec![123, 123, 123, 123, 123, 123],
        );
        let mut vm = VM::new();
        let result = vm.run(&chunk)?;

        assert_eq!(result, Value::Bool(true));

        Ok(())
    }

    #[test]
    fn greater() -> Result<(), InterpretError> {
        let chunk = Chunk::new(
            vec![
                OpCode::Constant as u8,
                0,
                OpCode::Constant as u8,
                1,
                OpCode::Greater as u8,
                OpCode::Return as u8,
            ],
            vec![Value::Float(-1.0), Value::Float(1.0)],
            vec![123, 123, 123, 123, 123, 123],
        );
        let mut vm = VM::new();
        let result = vm.run(&chunk)?;

        assert_eq!(result, Value::Bool(false));

        Ok(())
    }

    #[test]
    fn equals() -> Result<(), InterpretError> {
        let chunk = Chunk::new(
            vec![
                OpCode::Constant as u8,
                0,
                OpCode::Constant as u8,
                1,
                OpCode::Equal as u8,
                OpCode::Return as u8,
            ],
            vec![Value::Float(1.0), Value::Float(1.0)],
            vec![123, 123, 123, 123, 123, 123],
        );
        let mut vm = VM::new();
        let result = vm.run(&chunk)?;

        assert_eq!(result, Value::Bool(true));

        Ok(())
    }

    #[test]
    fn concatenation() -> Result<(), InterpretError> {
        let chunk = Chunk::new(
            vec![
                OpCode::Constant as u8,
                0,
                OpCode::Constant as u8,
                1,
                OpCode::Add as u8,
                OpCode::Return as u8,
            ],
            vec![
                Value::Str(String::from("hello, ")),
                Value::Str(String::from("world!")),
            ],
            vec![123, 123, 123, 123, 123, 123],
        );
        let mut vm = VM::new();
        let result = vm.run(&chunk)?;

        assert_eq!(result, Value::Str(String::from("hello, world!")));

        Ok(())
    }
}
