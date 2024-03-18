use crate::errors::ScannerError;

#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
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
    Str,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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
    Eof,
    Break,
    Error,
    String,
}

const RADIX: u32 = 10;

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    type_: TokenType,
    start: usize,
    length: usize,
    line: usize,
}

pub struct Scanner {
    source: Vec<char>,

    start: usize,
    current: usize,

    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_token(&mut self) -> Result<Token, ScannerError> {
        self.skip_whitespace();

        self.start = self.current; // set start of the token

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        match self.advance() {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '"' => self.string(),
            c => {
                if c.is_digit(RADIX) {
                    return self.number();
                } else if c.is_alphabetic() {
                    return self.identifier();
                }

                self.make_error("Unexpected charecter.".to_string())
            }
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let ch = self.peek();
            match ch {
                '\t' | '\r' | ' ' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if let Some('/') = self.peek_next() {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => break,
            }
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    // Looks at current char but does not advance.
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        Some(self.source[self.current + 1])
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn is_at_end(&self) -> bool {
        return self.current == self.source.len();
    }

    fn string(&mut self) -> Result<Token, ScannerError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.make_error("Unterminated string.".to_string());
        }

        self.advance();
        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Result<Token, ScannerError> {
        let radix = 10;
        while self.peek().is_digit(radix) {
            self.advance();
        }

        // It cannot parse numbers that end with `.`, example `10.` is not a number!
        if let Some(c) = self.peek_next() {
            if self.peek() == '.' && c.is_digit(radix) {
                self.advance();
                while self.peek().is_digit(radix) {
                    self.advance();
                }
            }
        }

        return self.make_token(TokenType::Number);
    }

    fn identifier(&mut self) -> Result<Token, ScannerError> {
        // One needs to check that the identifier starts with alpha before calling this function
        while self.peek().is_alphabetic() || self.peek() == '_' || self.peek().is_digit(RADIX) {
            self.advance();
        }
        self.make_token(TokenType::Identifier)
    }

    fn identifier_type(&self) -> Result<Token, ScannerError> {
        let tok_type = match self.source[self.start] {
            'a' => self.check_keyword("nd", 1, 2, TokenType::And),
            'c' => self.check_keyword("lass", 1, 4, TokenType::Class),
            'e' => self.check_keyword("lse", 1, 3, TokenType::Else),
            'i' => self.check_keyword("f", 1, 1, TokenType::If),
            'n' => self.check_keyword("il", 1, 2, TokenType::Nil),
            _ => TokenType::Identifier,
        };
        self.make_token(tok_type)
    }

    fn check_keyword(
        &self,
        postfix: &str,
        offset: usize,
        length: usize,
        tok_type: TokenType,
    ) -> TokenType {
        if compare(&self.source, postfix, self.start + offset, length) {
            return tok_type;
        }
        return TokenType::Identifier;
    }

    fn make_token(&self, type_: TokenType) -> Result<Token, ScannerError> {
        return Ok(Token {
            type_,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        });
    }
    fn make_error(&self, msg: String) -> Result<Token, ScannerError> {
        println!(
            "*{:}",
            self.source[self.current..].iter().collect::<String>()
        );
        return Err(ScannerError {
            message: msg,
            line: self.line,
        });
    }
}

pub fn compare(original: &Vec<char>, postfix: &str, start: usize, length: usize) -> bool {
    if start + length > original.len() {
        return false;
    }
    // assumes that original and postfix have same length
    let mut eq = true;
    for (a, b) in original[start..start + length].iter().zip(postfix.chars()) {
        println!("{} {}", a, b);
        eq &= *a == b
    }
    println!("out");
    eq
}

mod test_scanner {
    use crate::scanner::{compare, Scanner, Token, TokenType};

    #[test]
    fn test_compare() {
        let my_vec = "test string".chars().collect::<Vec<char>>();
        assert_eq!(compare(&my_vec, "string", 5, 6), true)
    }

    #[test]
    fn test_empty_string_scan() {
        let mut scnnr = Scanner::new(&"");
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::Eof,
                start: 0,
                length: 0,
                line: 1
            }
        )
    }

    #[test]
    fn test_parens_scan() {
        let mut scnnr = Scanner::new(&"(  (  )    )");
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::LeftParen,
                start: 0,
                length: 1,
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::LeftParen,
                start: 3,
                length: 1,
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::RightParen,
                start: 6,
                length: 1,
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::RightParen,
                start: 11,
                length: 1,
                line: 1
            }
        );
    }
    #[test]
    fn test_lookahead_scan() {
        let mut scnnr = Scanner::new(&"== =!= >= \n <= ");
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::EqualEqual,
                start: 0,
                length: 2,
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::Equal,
                start: 3,
                length: 1,
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::BangEqual,
                start: 4,
                length: 2,
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::GreaterEqual,
                start: 7,
                length: 2,
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::LessEqual,
                start: 12,
                length: 2,
                line: 2
            }
        )
    }

    #[test]
    fn test_comment() {
        let mut scnnr = Scanner::new(&"== / // this is a comment \n <= ");
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::EqualEqual,
                start: 0,
                length: 2,
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::Slash,
                start: 3,
                length: 1,
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::LessEqual,
                start: 28,
                length: 2,
                line: 2
            }
        )
    }

    #[test]
    fn test_parse_multiline_string() {
        let mut scnnr = Scanner::new("\"...\n...\n\"");
        assert_eq!(
            scnnr.scan_token().unwrap(),
            Token {
                type_: TokenType::String,
                start: 0,
                length: 10,
                line: 3
            }
        )
    }

    #[test]
    fn test_parse_number() {
        let mut s = Scanner::new("1.2 345.6");
        assert_eq!(
            s.scan_token().unwrap(),
            Token {
                type_: TokenType::Number,
                start: 0,
                length: 3,
                line: 1
            }
        );
        assert_eq!(
            s.scan_token().unwrap(),
            Token {
                type_: TokenType::Number,
                start: 4,
                length: 5,
                line: 1
            }
        )
    }
}
