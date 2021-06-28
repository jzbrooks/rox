#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Float(f64),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn is_float(&self) -> bool {
        if let Value::Float(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_bool(&self) -> bool {
        if let Value::Bool(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn as_float(&self) -> f64 {
        if let Value::Float(f) = *self {
            f
        } else {
            panic!("Value ({}) is not a float", *self);
        }
    }

    pub fn as_bool(&self) -> bool {
        if let Value::Bool(b) = *self {
            b
        } else {
            panic!("Value ({}) is not a bool", *self);
        }
    }

    pub fn is_nil(&self) -> bool {
        if let Value::Nil = *self {
            true
        } else {
            false
        }
    }

    pub fn is_falsey(&self) -> bool {
        match *self {
            Value::Nil | Value::Bool(false) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[repr(u8)]
pub enum OpCode {
    Add,
    Subtract,
    Multiply,
    Divide,
    Constant,
    Negate,
    Nil,
    True,
    False,
    Not,
    Equal,
    Greater,
    Less,
    Return,
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

        let op: OpCode = unsafe { std::mem::transmute(byte) };

        match op {
            OpCode::Add => self.simple_instruction("OP_ADD", offset),
            OpCode::Subtract => self.simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => self.simple_instruction("OP_MULTIPLY", offset),
            OpCode::Divide => self.simple_instruction("OP_DIVIDE", offset),
            OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::Negate => self.simple_instruction("OP_NEGATE", offset),
            OpCode::Nil => self.simple_instruction("OP_NIL", offset),
            OpCode::Not => self.simple_instruction("OP_NOT", offset),
            OpCode::True => self.simple_instruction("OP_TRUE", offset),
            OpCode::False => self.simple_instruction("OP_FALSE", offset),
            OpCode::Return => self.simple_instruction("OP_RETURN", offset),
            OpCode::Equal => self.simple_instruction("OP_EQUAL", offset),
            OpCode::Greater => self.simple_instruction("OP_GREATER", offset),
            OpCode::Less => self.simple_instruction("OP_LESS", offset),
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
