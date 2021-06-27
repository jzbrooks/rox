use crate::bytecode::Value;
use crate::compiler::Compiler;
use crate::op_code;
use crate::Chunk;

#[derive(Debug)]
pub struct VM {
    ip: usize,
    stack: Vec<Value>,
    pub output: Option<Value>,
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
					let b = self.stack.pop().unwrap();
					let a = self.stack.pop().unwrap();

					if let (Value::Float(u), Value::Float(t)) = (b, a) {
                        self.stack.push(Value::Float(t $op u));
                    } else {
                        panic!("This shouldn't happen.");
                    }
				};
			}
		}

        loop {
            let op = chunk.code[self.ip];
            self.ip += 1;

            match op {
                op_code::ADD => binop!(+),
                op_code::SUBTRACT => binop!(-),
                op_code::MULTIPLY => binop!(*),
                op_code::DIVIDE => binop!(/),
                op_code::NEGATE => {
                    let previous = self.stack.pop().unwrap();
                    if let Value::Float(n) = previous {
                        self.stack.push(Value::Float(-n));
                    } else {
                        panic!("Cannot negate {}", previous);
                    }
                }
                op_code::CONSTANT => {
                    let constant_index = chunk.code[self.ip] as usize;
                    self.ip += 1;
                    self.stack.push(chunk.constants[constant_index].clone());
                }
                op_code::RETURN => {
                    self.output = self.stack.pop();
                    println!("{}", self.output.as_ref().unwrap().clone());
                    return InterpretResult::Ok;
                }
                _ => panic!("Unsupported opcode!"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::op_code;
    use crate::Chunk;

    #[test]
    fn division() {
        let chunk = Chunk::new(
            vec![
                op_code::CONSTANT,
                0,
                op_code::CONSTANT,
                1,
                op_code::DIVIDE,
                op_code::RETURN,
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
            vec![op_code::CONSTANT, 0, op_code::NEGATE, op_code::RETURN],
            vec![Value::Float(100.0)],
            vec![123, 123, 123, 123],
        );
        let mut vm = VM::new();
        vm.run(&chunk);

        assert_eq!(vm.output, Some(Value::Float(-100.0)));
    }
}
