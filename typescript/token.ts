export enum TokenType {
  ILLEGAL,
  EOF,
  IDENT,
  INT,
  ASSIGN,
  PLUS,
  COMMA,
  SEMICOLON,
  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,
  FUNCTION,
  LET,
}

export class Token {
  tokenType: TokenType;
  literal: any;

  constructor(tokenType: TokenType, literal: any) {
    this.tokenType = tokenType;
    this.literal = literal;
  }
}
