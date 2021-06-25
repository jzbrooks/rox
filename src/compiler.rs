use crate::bytecode::{op_code, Chunk};
use crate::scanner::{Scanner, Token, TokenKind};

#[derive(Debug)]
pub struct Compiler {
    scanner: Scanner,
    current: Token,
    previous: Option<Token>,
    chunk: Option<Chunk>,
    had_error: bool,
    panic_mode: bool,
}

impl Compiler {
    fn new(source: &str) -> Compiler {
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

    fn compile(&mut self) -> Chunk {
        self.chunk = Some(Chunk::new(Vec::new(), Vec::new(), Vec::new()));
        self.expression();
        self.consume(TokenKind::End, "Expected the end of an expression.");
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
                break;
            }
            self.error_at_current("Unreachable");
        }
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        if self.current.kind == kind {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn error_at_current(&mut self, message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        let token = self.previous.as_ref().unwrap();

        eprint!("[line {}] Error", token.line);

        match token.kind {
            TokenKind::End => eprint!(" at the end."),
            _ => eprint!(" at {} ", &(token.lexeme)),
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        match token.kind {
            TokenKind::End => eprint!(" at the end."),
            _ => eprint!(" at {} ", &(token.lexeme)),
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }

    fn emit(&mut self, byte: u8) {
        let line = self.previous.as_ref().unwrap().line;
        self.chunk.as_mut().unwrap().write(byte, line);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit(op_code::RETURN);
    }

    fn emit_two(&mut self, first: u8, second: u8) {
        self.emit(first);
        self.emit(second);
    }

    fn expression(&self) {}
}
