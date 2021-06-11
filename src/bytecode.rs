pub type Value = f64;

#[derive(Debug)]
pub enum OpCode {
	Constant(usize),
	Return,
}

#[derive(Debug)]
pub struct Chunk {
	code: Vec<OpCode>,
	constants: Vec<Value>,
	lines: Vec<u32>, // todo: run-length encoding?
}

impl Chunk {
	pub fn new(code: Vec<OpCode>, constants: Vec<Value>, lines: Vec<u32>) -> Chunk {
		Chunk { code, constants, lines }
	}

	pub fn disassemble(&self, description: &str) {
	    println!("=== {} ===", description);
	    for (offset, op) in self.code.iter().enumerate() {
	        self.disassemble_instruction(op, offset);
	    }
	}


	fn disassemble_instruction(&self, op: &OpCode, offset: usize) {
	    print!("{offset:>0width$} ", offset = offset, width = 4);

	    if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
			print!("   | ");
		} else {
		    print!("{line:>width$} ", line = self.lines[offset], width = 4);
		}

	    match op {
	        OpCode::Return => println!("OP_RETURN"),
	        OpCode::Constant(_) => self.constant_instruction(op),
	    }
	}

	fn constant_instruction(&self, op: &OpCode) {
		if let OpCode::Constant(index) = op {
			println!("OP_CONSTANT {index:>0width$} '{0}'", self.constants[*index], width = 4, index = index)
		}
	}
}
