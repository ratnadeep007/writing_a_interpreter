import { Token, TokenType } from "./token";

export class Lexer {
  input: string;
  position: number;
  readPosition: number;
  ch: string;

  constructor(input: string) {
    this.input = input;
    this.position = 0;
    this.readPosition = 0;
    this.ch = "";
    this.readChar();
  }

  readChar(): void {
    if (this.readPosition >= this.input.length) {
      this.ch = "";
    } else {
      this.ch = this.input[this.readPosition];
    }
    this.position = this.readPosition;
    this.readPosition += 1;
  }

  nextToken(): Token {
    let token: Token;

    switch (this.ch) {
      case "=":
        token = new Token(TokenType.ASSIGN, "=");
        break;
      case ";":
        token = new Token(TokenType.SEMICOLON, ";");
        break;
      case "(":
        token = new Token(TokenType.LPAREN, "(");
        break;
      case ")":
        token = new Token(TokenType.RPAREN, ")");
        break;
      case ",":
        token = new Token(TokenType.COMMA, ",");
        break;
      case "+":
        token = new Token(TokenType.PLUS, "+");
        break;
      case "{":
        token = new Token(TokenType.LBRACE, "{");
        break;
      case "}":
        token = new Token(TokenType.RBRACE, "}");
        break;
      case "":
        token = new Token(TokenType.ILLEGAL, "");
        break;
      default:
        token = new Token(TokenType.ILLEGAL, "");
    }
    this.readChar();
    return token;
  }
}
