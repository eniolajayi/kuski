#[derive(Debug)]
pub struct Token {
    tkn_type: TokenType,
    lexeme: String,
    literal: T,
    line: u32,
}

impl Token {
    pub fn create(tkn_type: TokenType, lexeme: String, literal: T, line: u32) -> Token {
        Token {
            tkn_type,
            lexeme,
            literal,
            line,
        }
    }
}
