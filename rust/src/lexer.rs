use crate::token::{Token, TokenType};

struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if (self.read_position as usize) >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position as usize]
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = Token::new(TokenType::EOF, String::from(""));

        let literal = String::from(self.ch as char);

        match self.ch as char {
            '=' => {
                tok = Token::new(TokenType::ASSIGN, String::from(self.ch as char));
            }
            ';' => {
                tok = Token::new(TokenType::SEMICOLON, literal);
            }
            '(' => {
                tok = Token::new(TokenType::LPAREN, literal);
            }
            ')' => {
                tok = Token::new(TokenType::RPAREN, literal);
            }
            ',' => {
                tok = Token::new(TokenType::COMMA, literal);
            }
            '+' => {
                tok = Token::new(TokenType::PLUS, literal);
            }
            '{' => {
                tok = Token::new(TokenType::LBRACE, literal);
            }
            '}' => {
                tok = Token::new(TokenType::RBRACE, literal);
            }
            '0' | '\0' => {
                tok.set_type(TokenType::EOF);
                tok.set_literal(String::from(""));
            }
            _ => {
                panic!("Unkown pattern");
            }
        };

        self.read_char();
        tok
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn basic_token_test() {
        let input = String::from("=+(){},;");

        let test: Vec<Token> = vec![
            Token::new(TokenType::ASSIGN, String::from("=")),
            Token::new(TokenType::PLUS, String::from("+")),
            Token::new(TokenType::LPAREN, String::from("(")),
            Token::new(TokenType::RPAREN, String::from(")")),
            Token::new(TokenType::LBRACE, String::from("{")),
            Token::new(TokenType::RBRACE, String::from("}")),
            Token::new(TokenType::COMMA, String::from(",")),
            Token::new(TokenType::SEMICOLON, String::from(";")),
            Token::new(TokenType::EOF, String::from("")),
        ];

        let mut l = Lexer::new(input);

        for (i, t) in test.iter().enumerate() {
            let tok = l.next_token();
            println!("index: {}, {}:  {}", i, tok.get_type(), t.get_type());

            assert_eq!(t.get_type(), tok.get_type(), "Type mismatch at {}", i);
            assert_eq!(t.get_literal(), tok.get_literal(), "Literal mismatch at {}", i);
        }
    }
}
