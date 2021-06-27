#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn next(&mut self) -> Token<'a> {
        self.skip_whitespace();

        self.start = self.current;

        if self.start == self.source.len() {
            return self.make_token(TokenKind::End);
        }

        let c = self.advance().chars().next().unwrap();

        if c.is_ascii_alphabetic() {
            return self.identifier();
        }

        if c.is_ascii_digit() {
            return self.number();
        }

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
                let kind = if self.matches("=") {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.make_token(kind)
            }
            '=' => {
                let kind = if self.matches("=") {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.make_token(kind)
            }
            '<' => {
                let kind = if self.matches("=") {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Equal
                };
                self.make_token(kind)
            }
            '>' => {
                let kind = if self.matches("=") {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Equal
                };
                self.make_token(kind)
            }
            '"' => self.string(),
            _ => self.make_error_token("Unexpected character"),
        };
    }

    fn make_token(&self, kind: TokenKind) -> Token<'a> {
        let lexeme = &self.source[self.start..self.current];
        Token::new(kind, lexeme, self.line)
    }

    fn make_error_token(&self, message: &'a str) -> Token<'a> {
        Token::new(TokenKind::Error, message, self.line)
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn peek(&self) -> &str {
        if self.is_at_end() {
            "\0"
        } else {
            &self.source[self.current..self.current + 1]
        }
    }

    fn peek_next(&self) -> &str {
        if self.source.len() == self.current + 1 {
            "\0"
        } else {
            &self.source[self.current + 1..self.current + 2]
        }
    }

    fn advance(&mut self) -> &str {
        self.current += 1;
        &self.source[self.current - 1..self.current]
    }

    fn matches(&mut self, expected: &str) -> bool {
        if self.start == self.source.len() {
            return false;
        }

        if &self.source[self.current..self.current + 1] != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn skip_whitespace(&mut self) {
        if self.is_at_end() {
            return;
        }

        loop {
            let c = self.peek().chars().next().unwrap();
            match c {
                ' ' | '\r' | '\t' => self.current += 1,
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == "/" {
                        while self.peek() != "\n" && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek() != "\"" && !self.is_at_end() {
            if self.peek() == "\n" {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.make_error_token("Unterminated string.");
        }

        self.advance();
        self.make_token(TokenKind::String)
    }

    fn number(&mut self) -> Token<'a> {
        while self.peek().chars().next().unwrap().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == "." && self.peek_next().chars().next().unwrap().is_ascii_digit() {
            self.advance(); // eat the '.'

            while self.peek().chars().next().unwrap().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenKind::Number)
    }

    fn identifier(&mut self) -> Token<'a> {
        while self.peek().chars().next().unwrap().is_ascii_alphanumeric() {
            self.advance();
        }
        self.make_identifier_token()
    }

    fn make_identifier_token(&self) -> Token<'a> {
        let lexeme = &self.source[self.start..self.current];

        let kind = match lexeme {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::False,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        };

        Token::new(kind, lexeme, self.line)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub line: u32,
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

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a str, line: u32) -> Token<'a> {
        Token { kind, lexeme, line }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_are_parsed() {
        let mut scanner = Scanner::new("railroad");
        assert_eq!(
            scanner.next(),
            Token::new(TokenKind::Identifier, "railroad", 1)
        );
    }

    #[test]
    fn keywords_are_parsed() {
        let mut scanner = Scanner::new("this");
        assert_eq!(scanner.next(), Token::new(TokenKind::This, "this", 1));
    }

    #[test]
    fn punctuation_is_parsed() {
        let mut scanner = Scanner::new("{");
        assert_eq!(scanner.next(), Token::new(TokenKind::LeftBrace, "{", 1));
    }

    #[test]
    fn multicharacter_tokens_are_parsed() {
        let mut scanner = Scanner::new("!=");
        assert_eq!(scanner.next(), Token::new(TokenKind::BangEqual, "!=", 1));
    }

    #[test]
    fn whitespace_is_skipped() {
        let mut scanner = Scanner::new(" love");
        assert_eq!(scanner.next(), Token::new(TokenKind::Identifier, "love", 1));
    }

    #[test]
    fn comment_is_skipped() {
        let mut scanner = Scanner::new("// test a comment\ndifficult");
        assert_eq!(
            scanner.next(),
            Token::new(TokenKind::Identifier, "difficult", 2)
        );
    }

    #[test]
    fn newlines_increment_line_number() {
        let mut scanner = Scanner::new("\n.");
        assert_eq!(scanner.next(), Token::new(TokenKind::Dot, ".", 2));
    }
}
