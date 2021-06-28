use crate::bytecode::{op_code, Chunk, Value};
use crate::scanner::{Scanner, Token, TokenKind};
use precedence::Precedence;

pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Option<Token<'a>>,
    chunk: Option<Chunk>,
    had_error: bool,
    panic_mode: bool,
}

mod precedence {
    pub type Precedence = u8;

    pub const NONE: Precedence = 0;
    pub const ASSIGNMENT: Precedence = 1;
    pub const OR: Precedence = 2;
    pub const AND: Precedence = 3;
    pub const EQUALITY: Precedence = 4;
    pub const COMPARISON: Precedence = 5;
    pub const TERM: Precedence = 6;
    pub const FACTOR: Precedence = 7;
    pub const UNARY: Precedence = 8;
    pub const CALL: Precedence = 9;
    pub const PRIMARY: Precedence = 10;
}

pub struct ParseRule<'a> {
    prefix: Option<fn(compiler: &mut Compiler<'a>)>,
    infix: Option<fn(compiler: &mut Compiler<'a>)>,
    precedence: Precedence,
}

pub trait ParseRuled<'a> {
    fn get_parse_rule(&self) -> ParseRule<'a>;
}

impl<'a> ParseRuled<'a> for TokenKind {
    fn get_parse_rule(&self) -> ParseRule<'a> {
        match *self {
            TokenKind::LeftParen => ParseRule {
                prefix: Some(Compiler::binary),
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::RightParen => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::LeftBrace => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::RightBrace => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Comma => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Dot => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Minus => ParseRule {
                prefix: Some(Compiler::unary),
                infix: Some(Compiler::binary),
                precedence: precedence::TERM,
            },
            TokenKind::Plus => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::TERM,
            },
            TokenKind::Semicolon => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Slash => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::FACTOR,
            },
            TokenKind::Star => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::FACTOR,
            },
            TokenKind::Bang => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::NONE,
            },
            TokenKind::BangEqual => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Equal => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::EqualEqual => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Greater => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::GreaterEqual => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Less => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::LessEqual => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Identifier => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::String => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Number => ParseRule {
                prefix: Some(Compiler::number),
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::And => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Class => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Else => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::False => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::For => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Fun => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::If => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Nil => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Or => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Print => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Return => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Super => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::This => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::True => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Var => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::While => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::Error => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
            TokenKind::End => ParseRule {
                prefix: None,
                infix: None,
                precedence: precedence::NONE,
            },
        }
    }
}

macro_rules! error_at {
    ($c:expr,$t:expr,$m:expr) => {
        if $c.panic_mode {
            return;
        }

        $c.panic_mode = true;
        eprint!("[line {}] Error", $t.line);

        match $t.kind {
            TokenKind::End => eprint!(" at the end."),
            _ => eprint!(" at {} ", &($t.lexeme)),
        }

        eprintln!(": {}", $m);
        $c.had_error = true;
    };
}

impl<'a> Compiler<'a> {
    pub fn new(source: &str) -> Compiler {
        let mut scanner = Scanner::new(source);
        let current = scanner.next();

        Compiler {
            scanner,
            current,
            previous: None,
            chunk: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn compile(&mut self) -> Chunk {
        self.chunk = Some(Chunk::new(Vec::new(), Vec::new(), Vec::new()));
        self.expression();
        self.consume(TokenKind::End, "Expected the end of an expression.");
        self.end();
        if !self.had_error {
            self.chunk.as_ref().unwrap().clone()
        } else {
            panic!("this is bad")
        }
    }

    fn advance(&mut self) {
        self.previous = Some(self.current.clone());

        loop {
            self.current = self.scanner.next();
            if self.current.kind == TokenKind::Error {
                let lexeme = self.current.lexeme.clone();
                self.error_at_current(&*lexeme);
            }
            break;
        }
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        if self.current.kind == kind {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn error(&mut self, message: &str) {
        let previous = self.previous.as_ref().unwrap();
        error_at!(self, previous, message);
    }

    fn error_at_current(&mut self, message: &str) {
        let current = &self.current;
        error_at!(self, current, message);
    }

    fn emit(&mut self, byte: u8) {
        let line = self.previous.as_ref().unwrap().line;
        self.chunk.as_mut().unwrap().write(byte, line);
    }

    pub fn end(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit(op_code::RETURN);
    }

    fn emit_two(&mut self, first: u8, second: u8) {
        self.emit(first);
        self.emit(second);
    }

    fn emit_constant(&mut self, value: Value) {
        let index = self.make_constant(value);
        self.emit_two(op_code::CONSTANT, index);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant_index = self.chunk.as_mut().unwrap().write_constant(value);
        if constant_index > 255 {
            self.error("Too many constants in one chunk.");
            return 0;
        }

        constant_index as u8
    }

    fn expression(&mut self) {
        self.parse_precedence(precedence::ASSIGNMENT);
    }

    fn number(&mut self) {
        let number = self.previous.as_ref().unwrap().lexeme.parse().unwrap();
        let value = Value::Float(number);
        self.emit_constant(value);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "Expected ')' after expression");
    }

    fn unary(&mut self) {
        let operator_kind = self.previous.as_ref().unwrap().kind;

        self.parse_precedence(precedence::UNARY);

        match operator_kind {
            TokenKind::Minus => self.emit(op_code::NEGATE),
            _ => self.error(&*format!("Unexpected unary operator: {:?}", operator_kind)),
        }
    }

    fn binary(&mut self) {
        let operator_kind = self.previous.as_ref().unwrap().kind;
        let rule = operator_kind.get_parse_rule();
        self.parse_precedence(rule.precedence + 1);

        match operator_kind {
            TokenKind::Plus => self.emit(op_code::ADD),
            TokenKind::Minus => self.emit(op_code::SUBTRACT),
            TokenKind::Star => self.emit(op_code::MULTIPLY),
            TokenKind::Slash => self.emit(op_code::DIVIDE),
            _ => self.error(&*format!("Unexpected binary operator: {:?}", operator_kind)),
        }
    }

    fn literal(&mut self) {
        match self.previous.as_ref().unwrap().kind {
            TokenKind::True => self.emit(op_code::TRUE),
            TokenKind::False => self.emit(op_code::FALSE),
            TokenKind::Nil => self.emit(op_code::NIL),
            _ => unreachable!(
                "Literal not handled {}",
                self.previous.as_ref().unwrap().lexeme
            ),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let prefix_rule = self.previous.as_ref().unwrap().kind.get_parse_rule().prefix;
        if let Some(rule) = prefix_rule {
            rule(self);
        } else {
            self.error("Expected an expression.");
            return;
        }

        while precedence <= self.current.kind.get_parse_rule().precedence {
            self.advance();
            let previous_token = self.previous.as_ref().unwrap();
            let parse_rule = previous_token.kind.get_parse_rule();
            let infix_rule = parse_rule.infix.unwrap();
            infix_rule(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unary_expression() {
        let mut compiler = Compiler::new("-1");
        let chunk = compiler.compile();

        assert_eq!(chunk.constants[0], Value::Float(1.0));
        assert_eq!(chunk.code[0], op_code::CONSTANT);
        assert_eq!(chunk.code[1], 0);
        assert_eq!(chunk.code[2], op_code::NEGATE);
        assert_eq!(chunk.code[3], op_code::RETURN);
    }

    #[test]
    fn binary_expression() {
        let mut compiler = Compiler::new("1 + 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.constants[0], Value::Float(1.0));
        assert_eq!(chunk.constants[1], Value::Float(2.0));
        assert_eq!(chunk.code[0], op_code::CONSTANT);
        assert_eq!(chunk.code[1], 0);
        assert_eq!(chunk.code[2], op_code::CONSTANT);
        assert_eq!(chunk.code[3], 1);
        assert_eq!(chunk.code[4], op_code::ADD);
        assert_eq!(chunk.code[5], op_code::RETURN);
    }

    #[test]
    fn literal_true() {
        let mut compiler = Compiler::new("true");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[0], op_code::TRUE);
        assert_eq!(chunk.code[1], op_code::RETURN);
    }

    #[test]
    fn literal_false() {
        let mut compiler = Compiler::new("false");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[0], op_code::FALSE);
        assert_eq!(chunk.code[1], op_code::RETURN);
    }

    #[test]
    fn literal_nil() {
        let mut compiler = Compiler::new("nil");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[0], op_code::NIL);
        assert_eq!(chunk.code[1], op_code::RETURN);
    }
}
