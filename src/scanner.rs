#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u16,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.start == self.source.len() {
            return Token::new(TokenKind::End, "".to_string(), 0);
        }

        let c = self.advance();

        return match c {
            '(' => self.make_token(TokenKind::LeftParen),
            ')' => self.make_token(TokenKind::RightParen),
            '{' => self.make_token(TokenKind::LeftBrace),
            '}' => self.make_token(TokenKind::RightBrace),
            ';' => self.make_token(TokenKind::Semicolon),
            ',' => self.make_token(TokenKind::Comma),
            '.' => self.make_token(TokenKind::Dot),
            '-' => self.make_token(TokenKind::Minus),
            '+' => self.make_token(TokenKind::Plus),
            '/' => self.make_token(TokenKind::Slash),
            '*' => self.make_token(TokenKind::Star),
            '!' => {
                let kind = if self.matches('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.make_token(kind)
            }
            '=' => {
                let kind = if self.matches('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.make_token(kind)
            }
            '<' => {
                let kind = if self.matches('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Equal
                };
                self.make_token(kind)
            }
            '>' => {
                let kind = if self.matches('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Equal
                };
                self.make_token(kind)
            }
            _ => Token::new(
                TokenKind::Error,
                "Unexpected character.".to_string(),
                self.line,
            ),
        };
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        let lexeme = &self.source[self.start..self.current];
        Token::new(kind, lexeme.into_iter().collect(), self.line)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.start == self.source.len() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn skip_whitespace(&mut self) {
        if self.current == self.source.len() {
            return;
        }

        loop {
            let c = self.source[self.current];
            match c {
                ' ' | '\r' | '\t' => self.current += 1,
                _ => break,
            }
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    lexeme: String,
    line: u16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    End,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: u16) -> Token {
        Token { kind, lexeme, line }
    }
}
