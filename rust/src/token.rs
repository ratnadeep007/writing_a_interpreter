use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION(String),
    LET(String),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = match *self {
            TokenType::ASSIGN => "ASSIGN",
            TokenType::EOF => "EOF",
            TokenType::ILLEGAL => "ILLEGAL",
            TokenType::IDENT => "IDENT",
            TokenType::INT => "INT",
            TokenType::PLUS => "PLUS",
            TokenType::COMMA => "COMMA",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::LPAREN => "LPAREN",
            TokenType::RPAREN => "RPAREN",
            TokenType::LBRACE => "LBRACE",
            TokenType::RBRACE => "RBRACE",
            TokenType::FUNCTION(_) => "FUNCTION",
            TokenType::LET(_) => "LET",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }

    pub fn set_literal(&mut self, i: String) {
        self.literal = i;
    }

    pub fn set_type(&mut self, t_type: TokenType) {
        self.token_type = t_type;
    }

    pub fn get_literal(&self) -> String {
        self.literal.clone()
    }

    pub fn get_type(&self) -> TokenType {
        self.token_type.clone()
    }
}

