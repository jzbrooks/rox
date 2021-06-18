#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u16,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner { source: source.chars().collect(), start: 0, current: 0, line: 1 }
    }

    pub fn next(&mut self) -> Token {
        self.start = self.current;

        if self.start == self.source.len() {
            return Token::new(TokenKind::End, "", 0);
        }

        let char = self.source[self.current];

        return match char {
            '(' => Token::new(TokenKind::LeftParen, "(", self.line),
            _ => Token::new(TokenKind::Error, "Unexpected character.", self.line)
        }
    }
}


#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
    line: u16,
}

#[derive(Copy, Clone, Debug)]
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

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a str, line: u16) -> Token {
        Token { kind, lexeme, line }
    }
}