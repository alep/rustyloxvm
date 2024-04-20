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
    lexeme: String, // in the compiler book this is a pointer which works to set the error msg.
    line: usize,
}

impl Token {
    pub fn is_err(&self) -> bool {
        self.type_ == TokenType::Error
    }

    pub fn message(&self) -> String {
        self.lexeme.clone()
    }
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

    pub fn scan_token(&mut self) -> Token {
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

    pub fn advance(&mut self) -> char {
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

    fn string(&mut self) -> Token {
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

    fn number(&mut self) -> Token {
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

    fn identifier(&mut self) -> Token {
        // One needs to check that the identifier starts with alpha before calling this function
        while self.peek().is_alphabetic() || self.peek() == '_' || self.peek().is_digit(RADIX) {
            self.advance();
        }
        let tok_type = self.identifier_type();
        self.make_token(tok_type)
    }

    fn identifier_type(&self) -> TokenType {
        let got_next = self.current - self.start > 1;
        match (self.source[self.start], got_next) {
            ('a', _) => self.check_keyword("nd", 1, 2, TokenType::And),
            ('c', _) => self.check_keyword("lass", 1, 4, TokenType::Class),
            ('e', _) => self.check_keyword("lse", 1, 3, TokenType::Else),
            ('f', true) => match self.source[self.start + 1] {
                'a' => self.check_keyword("lse", 2, 3, TokenType::False),
                'o' => self.check_keyword("r", 2, 1, TokenType::For),
                'u' => self.check_keyword("n", 2, 1, TokenType::Fun),
                _ => TokenType::Identifier,
            },
            ('i', _) => self.check_keyword("f", 1, 1, TokenType::If),
            ('n', _) => self.check_keyword("il", 1, 2, TokenType::Nil),
            ('o', _) => self.check_keyword("r", 1, 1, TokenType::Or),
            ('p', _) => self.check_keyword("rint", 1, 4, TokenType::Print),
            ('r', _) => self.check_keyword("eturn", 1, 5, TokenType::Return),
            ('s', _) => self.check_keyword("uper", 1, 4, TokenType::Super),
            ('t', _) => match self.source[self.start + 1] {
                'h' => self.check_keyword("is", 2, 2, TokenType::This),
                'r' => self.check_keyword("ue", 2, 2, TokenType::True),
                _ => TokenType::Identifier,
            },
            ('v', _) => self.check_keyword("ar", 1, 2, TokenType::Var),
            ('w', _) => self.check_keyword("hile", 1, 4, TokenType::While),

            _ => TokenType::Identifier,
        }
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

    fn make_token(&self, type_: TokenType) -> Token {
        return Token {
            type_,
            lexeme: self.source[self.start..self.current]
                .iter()
                .collect::<String>(),
            line: self.line,
        };
    }
    fn make_error(&self, msg: String) -> Token {
        return Token {
            type_: TokenType::Error,
            lexeme: msg,
            line: self.line,
        };
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
            scnnr.scan_token(),
            Token {
                type_: TokenType::Eof,
                lexeme: "".to_string(),
                line: 1
            }
        )
    }

    #[test]
    fn test_parens_scan() {
        let mut scnnr = Scanner::new(&"(  (  )    )");
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::LeftParen,
                lexeme: "(".to_string(),
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::LeftParen,
                lexeme: "(".to_string(),
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::RightParen,
                lexeme: ")".to_string(),
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::RightParen,
                lexeme: ")".to_string(),
                line: 1
            }
        );
    }
    #[test]
    fn test_lookahead_scan() {
        let mut scnnr = Scanner::new(&"== =!= >= \n <= ");
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::EqualEqual,
                lexeme: "==".to_string(),
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::Equal,
                lexeme: "=".to_string(),
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::BangEqual,
                lexeme: "!=".to_string(),
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::GreaterEqual,
                lexeme: ">=".to_string(),
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::LessEqual,
                lexeme: "<=".to_string(),
                line: 2
            }
        )
    }

    #[test]
    fn test_comment() {
        let mut scnnr = Scanner::new(&"== / // this is a comment \n <= ");
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::EqualEqual,
                lexeme: "==".to_string(),
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::Slash,
                lexeme: "/".to_string(),
                line: 1
            }
        );
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::LessEqual,
                lexeme: "<=".to_string(),
                line: 2
            }
        )
    }

    #[test]
    fn test_parse_multiline_string() {
        let mut scnnr = Scanner::new("\"...\n...\n\"");
        assert_eq!(
            scnnr.scan_token(),
            Token {
                type_: TokenType::String,
                lexeme: "\"...\n...\n\"".to_string(),
                line: 3
            }
        )
    }

    #[test]
    fn test_parse_number() {
        let mut s = Scanner::new("1.2 345.6");
        assert_eq!(
            s.scan_token(),
            Token {
                type_: TokenType::Number,
                lexeme: "1.2".to_string(),
                line: 1
            }
        );
        assert_eq!(
            s.scan_token(),
            Token {
                type_: TokenType::Number,
                lexeme: "345.6".to_string(),
                line: 1
            }
        )
    }

    #[test]
    fn test_parse_identifiers() {
        let mut s = Scanner::new("while true { print \"hello, world\"; }");
        assert_eq!(
            s.scan_token(),
            Token {
                type_: TokenType::While,
                lexeme: "while".to_string(),
                line: 1
            }
        );
        assert_eq!(
            s.scan_token(),
            Token {
                type_: TokenType::True,
                lexeme: "true".to_string(),
                line: 1,
            }
        );
        s.scan_token();
        assert_eq!(
            s.scan_token(),
            Token {
                type_: TokenType::Print,
                lexeme: "print".to_string(),
                line: 1,
            }
        );
    }
}
