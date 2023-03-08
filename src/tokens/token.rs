use crate::tokens::token_type::TokenType;
// TODO Learn more about Generic parameters in structs
#[derive(Debug)]
pub struct Token {
    tkn_type: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn create(tkn_type: TokenType, lexeme: String, line: u32) -> Token {
        Token {
            tkn_type,
            lexeme,
            line,
        }
    }
}
