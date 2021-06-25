#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    pub start: usize,
    current: usize,
    line: u32,
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
            return self.make_token(TokenKind::End);
        }

        let mut c = self.advance();

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
            '"' => self.string(),
            _ => self.make_error_token("Unexpected character"),
        };
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        let lexeme = &self.source[self.start..self.current];
        Token::new(kind, lexeme.into_iter().collect(), self.line)
    }

    fn make_error_token(&self, message: &str) -> Token {
        Token::new(TokenKind::Error, message.to_string(), self.line)
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.source.len() == self.current + 1 {
            '\0'
        } else {
            self.source[self.current + 1]
        }
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
        if self.is_at_end() {
            return;
        }

        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => self.current += 1,
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
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

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
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

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // eat the '.'

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenKind::Number)
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        self.make_identifier_token()
    }

    fn make_identifier_token(&self) -> Token {
        let lexeme: String = self.source[self.start..self.current].into_iter().collect();

        let kind = match lexeme.as_str() {
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
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
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

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: u32) -> Token {
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
            Token::new(TokenKind::Identifier, String::from("railroad"), 1)
        );
    }

    #[test]
    fn keywords_are_parsed() {
        let mut scanner = Scanner::new("this");
        assert_eq!(
            scanner.next(),
            Token::new(TokenKind::This, String::from("this"), 1)
        );
    }

    #[test]
    fn punctuation_is_parsed() {
        let mut scanner = Scanner::new("{");
        assert_eq!(
            scanner.next(),
            Token::new(TokenKind::LeftBrace, String::from("{"), 1)
        );
    }

    #[test]
    fn multicharacter_tokens_are_parsed() {
        let mut scanner = Scanner::new("!=");
        assert_eq!(
            scanner.next(),
            Token::new(TokenKind::BangEqual, String::from("!="), 1)
        );
    }

    #[test]
    fn whitespace_is_skipped() {
        let mut scanner = Scanner::new(" love");
        assert_eq!(
            scanner.next(),
            Token::new(TokenKind::Identifier, String::from("love"), 1)
        );
    }

    #[test]
    fn comment_is_skipped() {
        let mut scanner = Scanner::new("// test a comment\ndifficult");
        assert_eq!(
            scanner.next(),
            Token::new(TokenKind::Identifier, String::from("difficult"), 2)
        );
    }

    #[test]
    fn newlines_increment_line_number() {
        let mut scanner = Scanner::new("\n.");
        assert_eq!(
            scanner.next(),
            Token::new(TokenKind::Dot, String::from("."), 2)
        );
    }
}
