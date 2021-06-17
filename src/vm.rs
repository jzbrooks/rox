use crate::Chunk;
use crate::OpCode;
use crate::bytecode::Value;

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
		VM { chunk: None, ip: 0, stack: Vec::<Value>::new() }
	}

	pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
		self.chunk = Some(chunk);
		self.ip = 0;
		self.run()
	}

	fn run(&mut self) -> InterpretResult {
		let chunk = &self.chunk.as_ref().unwrap();
		println!("{:?}", self.stack);
		chunk.disassemble("test");

		loop {
			let op = &chunk.code[self.ip];
			self.ip += 1;

			match op {
				OpCode::Add => {
					let a = self.stack.pop().unwrap();
					let b = self.stack.pop().unwrap();
					self.stack.push(a + b);
				}
				OpCode::Subtract => {
					let a = self.stack.pop().unwrap();
					let b = self.stack.pop().unwrap();
					self.stack.push(a - b);
				},
				OpCode::Multiply => {
					let a = self.stack.pop().unwrap();
					let b = self.stack.pop().unwrap();
					self.stack.push(a * b);
				},
				OpCode::Divide => {
					let a = self.stack.pop().unwrap();
					let b = self.stack.pop().unwrap();
					self.stack.push(a / b);
				},
				OpCode::Negate => {
					let negation = -self.stack.pop().unwrap();
					self.stack.push(negation);		
				}
				OpCode::Constant(value) => {
					self.stack.push(chunk.constants[*value]);
				},
				OpCode::Return => {
					println!("{:?}", self.stack.pop().unwrap());
					return InterpretResult::Ok;
				},
			}
		}
	}
}