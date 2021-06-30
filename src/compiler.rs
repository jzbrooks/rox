use crate::bytecode::{Chunk, OpCode, Value};
use crate::scanner::{Scanner, Token, TokenKind};

pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Option<Token<'a>>,
    chunk: Option<Chunk>,
    had_error: bool,
    panic_mode: bool,
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[repr(u8)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

struct ParseRule<'a> {
    prefix: Option<fn(compiler: &mut Compiler<'a>)>,
    infix: Option<fn(compiler: &mut Compiler<'a>)>,
    precedence: Precedence,
}

trait ParseRuled<'a> {
    fn get_parse_rule(&self) -> ParseRule<'a>;
}

impl<'a> ParseRuled<'a> for TokenKind {
    fn get_parse_rule(&self) -> ParseRule<'a> {
        match *self {
            TokenKind::LeftParen => ParseRule {
                prefix: Some(Compiler::grouping),
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::RightParen => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::LeftBrace => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::RightBrace => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Comma => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Dot => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Minus => ParseRule {
                prefix: Some(Compiler::unary),
                infix: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
            TokenKind::Plus => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
            TokenKind::Semicolon => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Slash => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
            TokenKind::Star => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
            TokenKind::Bang => ParseRule {
                prefix: Some(Compiler::unary),
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::BangEqual => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Equality,
            },
            TokenKind::Equal => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::EqualEqual => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Equality,
            },
            TokenKind::Greater => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
            TokenKind::GreaterEqual => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
            TokenKind::Less => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
            TokenKind::LessEqual => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
            TokenKind::Identifier => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::String => ParseRule {
                prefix: Some(Compiler::string),
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Number => ParseRule {
                prefix: Some(Compiler::number),
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::And => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Class => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Else => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::False => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::For => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Fun => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::If => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Nil => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Or => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Print => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Return => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Super => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::This => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::True => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Var => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::While => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::Error => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenKind::End => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
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

    fn emit_op(&mut self, op: OpCode) {
        let byte = op as u8;
        self.emit_byte(byte);
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.as_ref().unwrap().line;
        self.chunk.as_mut().unwrap().write(byte, line);
    }

    pub fn end(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_op(OpCode::Return);
    }

    fn emit_two(&mut self, op: OpCode, data: u8) {
        self.emit_op(op);
        self.emit_byte(data);
    }

    fn emit_constant(&mut self, value: Value) {
        let index = self.make_constant(value);
        self.emit_two(OpCode::Constant, index);
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
        self.parse_precedence(Precedence::Assignment);
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

        self.parse_precedence(Precedence::Unary);

        match operator_kind {
            TokenKind::Bang => self.emit_op(OpCode::Not),
            TokenKind::Minus => self.emit_op(OpCode::Negate),
            _ => self.error(&*format!("Unexpected unary operator: {:?}", operator_kind)),
        }
    }

    fn binary(&mut self) {
        let operator_kind = self.previous.as_ref().unwrap().kind;
        let rule = operator_kind.get_parse_rule();
        let next_precedence: Precedence =
            unsafe { std::mem::transmute((rule.precedence as u8) + 1) };
        self.parse_precedence(next_precedence);

        match operator_kind {
            TokenKind::Plus => self.emit_op(OpCode::Add),
            TokenKind::Minus => self.emit_op(OpCode::Subtract),
            TokenKind::Star => self.emit_op(OpCode::Multiply),
            TokenKind::Slash => self.emit_op(OpCode::Divide),
            TokenKind::EqualEqual => self.emit_op(OpCode::Equal),
            TokenKind::BangEqual => {
                self.emit_op(OpCode::Equal);
                self.emit_op(OpCode::Not);
            }
            TokenKind::Less => self.emit_op(OpCode::Less),
            TokenKind::LessEqual => {
                self.emit_op(OpCode::Greater);
                self.emit_op(OpCode::Not);
            }
            TokenKind::Greater => self.emit_op(OpCode::Greater),
            TokenKind::GreaterEqual => {
                self.emit_op(OpCode::Less);
                self.emit_op(OpCode::Not);
            }
            _ => self.error(&*format!("Unexpected binary operator: {:?}", operator_kind)),
        }
    }

    fn literal(&mut self) {
        match self.previous.as_ref().unwrap().kind {
            TokenKind::True => self.emit_op(OpCode::True),
            TokenKind::False => self.emit_op(OpCode::False),
            TokenKind::Nil => self.emit_op(OpCode::Nil),
            _ => unreachable!(
                "Literal not handled {}",
                self.previous.as_ref().unwrap().lexeme
            ),
        }
    }

    fn string(&mut self) {
        let value = self
            .previous
            .as_ref()
            .unwrap()
            .lexeme
            .trim_matches('"')
            .to_string();
        self.emit_constant(Value::Str(value));
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
    fn last_opcode_is_return() {
        let mut compiler = Compiler::new("10");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[2], OpCode::Return as u8);
    }

    #[test]
    fn constant() {
        let mut compiler = Compiler::new("10");
        let chunk = compiler.compile();

        assert_eq!(chunk.constants[0], Value::Float(10.0));
        assert_eq!(chunk.code[0], OpCode::Constant as u8);
        assert_eq!(chunk.code[1], 0);
    }

    #[test]
    fn constant_string() {
        let mut compiler = Compiler::new(r#""constant""#);
        let chunk = compiler.compile();

        assert_eq!(chunk.constants[0], Value::Str(String::from("constant")));
        assert_eq!(chunk.code[0], OpCode::Constant as u8);
        assert_eq!(chunk.code[1], 0);
    }

    #[test]
    fn negation() {
        let mut compiler = Compiler::new("-1");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[2], OpCode::Negate as u8);
    }

    #[test]
    fn sum() {
        let mut compiler = Compiler::new("1 + 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[4], OpCode::Add as u8);
    }

    #[test]
    fn product() {
        let mut compiler = Compiler::new("1 * 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[4], OpCode::Multiply as u8);
    }

    #[test]
    fn difference() {
        let mut compiler = Compiler::new("1 - 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[4], OpCode::Subtract as u8);
    }

    #[test]
    fn quotient() {
        let mut compiler = Compiler::new("1 / 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[4], OpCode::Divide as u8);
    }

    #[test]
    fn arithmetic_precedence() {
        let mut compiler = Compiler::new("1 + 2 * 10");
        let chunk = compiler.compile();
        println!("{:?}", chunk.code);
        assert_eq!(chunk.code[6], OpCode::Multiply as u8);
        assert_eq!(chunk.code[7], OpCode::Add as u8);
    }

    #[test]
    fn coerced_precedence() {
        let mut compiler = Compiler::new("(1 + 2) * 10");
        let chunk = compiler.compile();
        println!("{:?}", chunk.code);
        assert_eq!(chunk.code[4], OpCode::Add as u8);
        assert_eq!(chunk.code[7], OpCode::Multiply as u8);
    }

    #[test]
    fn equal() {
        let mut compiler = Compiler::new("1 == 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[4], OpCode::Equal as u8);
    }

    #[test]
    fn not_equal() {
        let mut compiler = Compiler::new("1 != 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[4], OpCode::Equal as u8);
        assert_eq!(chunk.code[5], OpCode::Not as u8);
    }

    #[test]
    fn greater() {
        let mut compiler = Compiler::new("1 > 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[4], OpCode::Greater as u8);
    }

    #[test]
    fn greater_equal() {
        let mut compiler = Compiler::new("1 >= 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[4], OpCode::Less as u8);
        assert_eq!(chunk.code[5], OpCode::Not as u8);
    }

    #[test]
    fn less() {
        let mut compiler = Compiler::new("1 < 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[4], OpCode::Less as u8);
    }

    #[test]
    fn less_equal() {
        let mut compiler = Compiler::new("1 <= 2");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[4], OpCode::Greater as u8);
        assert_eq!(chunk.code[5], OpCode::Not as u8);
    }

    #[test]
    fn literal_true() {
        let mut compiler = Compiler::new("true");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[0], OpCode::True as u8);
    }

    #[test]
    fn literal_false() {
        let mut compiler = Compiler::new("false");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[0], OpCode::False as u8);
    }

    #[test]
    fn literal_nil() {
        let mut compiler = Compiler::new("nil");
        let chunk = compiler.compile();

        assert_eq!(chunk.code[0], OpCode::Nil as u8);
    }
}
