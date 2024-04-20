use crate::scanner::{Scanner, Token, TokenType};

struct Parser {
    current: Option<Token>,
    previous: Option<Token>,

    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn is_current_err(&self) -> bool {
        false
    }
}

struct Compiler {
    parser: Parser,
    scanner: Scanner,
}

impl Compiler {
    pub fn new(source: &str) -> Self {
        Self {
            parser: Parser {
                current: None,
                previous: None,
                had_error: false,
                panic_mode: false,
            },
            scanner: Scanner::new(source),
        }
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.take();
        loop {
            let tok = self.scanner.scan_token();

            // it feels like you either extract all data upfront
            // or you clone the data

            let is_err = tok.is_err();
            let msg = tok.message();

            self.parser.current = Some(tok);

            if !is_err {
                break;
            }

            self.error_at_current(msg)
        }
    }

    fn error_at_current(&self, message: String) {
        todo!()
    }
}

// impl Compiler {
//     pub fn new(source: &str) -> Self {
//         Self {
//             previous: None,
//             current: None,
//
//             scanner: Scanner::new(source),
//             tokens: vec![],
//
//             had_error: false,
//             panic_mode: false,
//         }
//     }
//
//     pub fn compile(&mut self) -> bool {
//         self.advance();
//         self.expression();
//         self.consume(TokenType::Eof, &"fuck this shit");
//         return !self.had_error;
//     }
//
//     pub fn advance(&mut self) -> {
//         // ToDo: This should retrieve next token from scanner
//         loop {
//             self.previous = self.current.take();
//             self.current = self.scanner.scan_token();
//
//             if self.current.is_some() {
//                 break;
//             }
//         }
//         Ok(())
//     }
//
//     fn scan_token(&mut self) -> Result<Option<Token>, CompilerError> {
//         match self.scanner.scan_token() {
//             Ok(tok) => Ok(Some(tok)),
//             Err(err) => Err(CompilerError::from_scanner_error(err)),
//         }
//     }
//
//     fn handle_tokenizer_error(&mut self, err: ScannerError) {
//         if self.panic_mode {
//             return;
//         }
//         self.panic_mode = true;
//         eprintln!("[line {:}] Error: {:}", err.line, err.message);
//         self.had_error = true;
//     }
//
//     pub fn consume(&mut self, tok_type: TokenType, message: &str) -> Result<(), CompilerError> {
//         let should_advance = match &self.current {
//             Some(tok) => tok.type_ == tok_type,
//             None => false,
//         };
//
//         if should_advance {
//             self.advance();
//             return;
//         }
//
//         self.handle_parser_error(message)
//     }
//
//     pub fn expression(&mut self) {
//         // ToDo: evaluates expression
//         todo!()
//     }
//
//     pub fn handle_parser_error(&mut self, message: &str) {
//         if self.panic_mode {
//             return;
//         }
//         self.panic_mode = true;
//
//         let part = match &self.current {
//             Some(tok) if tok.type_ == TokenType::Eof => " at end".to_string(),
//             Some(tok) => format!("at line {:}", tok.line),
//             None => "previous error at scanner".to_string(),
//         };
//
//         self.had_error = true;
//     }
// }
//
// mod test_compiler {
//     use crate::compiler::Compiler;
//
//     #[test]
//     fn test_compiler() {
//         assert!(true);
//     }
// }
