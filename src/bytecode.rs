#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Float(f64),
    Bool(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

pub mod op_code {
    pub const ADD: u8 = 0;
    pub const SUBTRACT: u8 = 1;
    pub const MULTIPLY: u8 = 2;
    pub const DIVIDE: u8 = 3;
    pub const CONSTANT: u8 = 4;
    pub const NEGATE: u8 = 5;
    pub const RETURN: u8 = 6;
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<u32>, // todo: run-length encoding?
}

impl Chunk {
    pub fn new(code: Vec<u8>, constants: Vec<Value>, lines: Vec<u32>) -> Chunk {
        Chunk {
            code,
            constants,
            lines,
        }
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, description: &str) {
        println!("=== {} ===", description);
        let mut offset: usize = 0;
        while offset < self.code.len() {
            let byte = self.code[offset];
            offset = self.disassemble_instruction(byte, offset);
        }
    }

    fn disassemble_instruction(&self, byte: u8, offset: usize) -> usize {
        print!("{offset:>0width$} ", offset = offset, width = 4);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{line:>width$} ", line = self.lines[offset], width = 4);
        }

        match byte {
            op_code::ADD => self.simple_instruction("OP_ADD", offset),
            op_code::SUBTRACT => self.simple_instruction("OP_SUBTRACT", offset),
            op_code::MULTIPLY => self.simple_instruction("OP_MULTIPLY", offset),
            op_code::DIVIDE => self.simple_instruction("OP_DIVIDE", offset),
            op_code::NEGATE => self.simple_instruction("OP_NEGATE", offset),
            op_code::RETURN => self.simple_instruction("OP_RETURN", offset),
            op_code::CONSTANT => self.constant_instruction("OP_CONSTANT", offset),
            _ => panic!("Unsupported opcode!"),
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        println!(
            "{0} {index:>0width$} '{1}'",
            name,
            self.constants[self.code[offset + 1] as usize],
            index = offset + 1,
            width = 4,
        );

        offset + 2
    }
}
