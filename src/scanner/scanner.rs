use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType::{Eof};
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    // Do we need to pass in tokens here?
    fn new(&mut self, source: &String, tokens: Vec<Token>) -> Scanner {
        Scanner {
            // NOTE Is this a good pattern?
            source: source.to_string(),
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn set_src(&mut self, source: &String) {
        self.source = source.to_string();
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_source_end() {
            self.start = self.current;
            // TODO Return tokens before pushing end of file
            self.scan_token();
        }
        let mut all_tokens = vec![];
        all_tokens.push(Token::create(Eof, "".to_string(), self.line));
        all_tokens.append(&mut self.tokens);
        all_tokens
    }

    fn is_at_source_end(&self) -> bool {
        self.current >= (self.source.len() as u32)
    }

    fn scan_token(&mut self){

    }
}
