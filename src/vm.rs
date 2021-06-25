use crate::bytecode::Value;
use crate::compiler::Compiler;
use crate::op_code;
use crate::Chunk;

#[derive(Debug)]
pub struct VM {
    chunk: Option<Chunk>,
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
            chunk: None,
            ip: 0,
            stack: Vec::<Value>::new(),
            output: None,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    pub fn interpret_source(&mut self, source: &str) -> InterpretResult {
        // let chunk = compile(source);
        // self.interpret(chunk.unwrap())
        InterpretResult::Ok
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
                    let negation = -self.stack.pop().unwrap();
                    self.stack.push(negation);
                }
                op_code::CONSTANT => {
                    let constant_index = chunk.code[self.ip] as usize;
                    self.ip += 1;
                    self.stack.push(chunk.constants[constant_index]);
                }
                op_code::RETURN => {
                    self.output = self.stack.pop();
                    println!("{:?}", self.output.unwrap());
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
                1,
                op_code::CONSTANT,
                0,
                op_code::DIVIDE,
                op_code::RETURN,
            ],
            vec![100.0, 5.0],
            vec![123, 123, 123, 123, 123, 123, 123],
        );
        let mut vm = VM::new();
        vm.interpret(chunk);

        assert_eq!(vm.output, Some(20.0));
    }

    #[test]
    fn negation() {
        let chunk = Chunk::new(
            vec![op_code::CONSTANT, 0, op_code::NEGATE, op_code::RETURN],
            vec![100.0],
            vec![123, 123, 123, 123],
        );
        let mut vm = VM::new();
        vm.interpret(chunk);

        assert_eq!(vm.output, Some(-100.0));
    }
}
