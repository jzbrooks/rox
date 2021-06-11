use crate::Chunk;

#[derive(Debug)]
pub struct VM {
	chunk: Option<Chunk>,
	ip: usize,
}

#[derive(Debug)]
pub enum InterpretResult {
	Ok,
	CompileError,
	RuntimeError,
}

impl VM {
	pub fn new() -> VM {
		VM { chunk: None, ip: 0 }
	}

	pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
		self.chunk = Some(chunk);
		self.ip = 0;
		self.run()
	}

	fn run(&self) -> InterpretResult {
		InterpretResult::Ok
	}
}