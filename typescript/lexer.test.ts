import { expect, test } from "bun:test";
import { Token, TokenType } from "./token";
import { Lexer } from "./lexer";

type expectedType = {
  expectedType: TokenType;
  expectedLiteral: string;
};

test("Test Next Token", () => {
  const input = `=+(){},;`;

  const tests = [
    {
      expectedType: TokenType.ASSIGN,
      expectedLiteral: "=",
    },
    {
      expectedType: TokenType.PLUS,
      expectedLiteral: "+",
    },
    {
      expectedType: TokenType.LPAREN,
      expectedLiteral: "(",
    },
    {
      expectedType: TokenType.RPAREN,
      expectedLiteral: ")",
    },
    {
      expectedType: TokenType.LBRACE,
      expectedLiteral: "{",
    },
    {
      expectedType: TokenType.RBRACE,
      expectedLiteral: "}",
    },
    {
      expectedType: TokenType.COMMA,
      expectedLiteral: ",",
    },
    {
      expectedType: TokenType.SEMICOLON,
      expectedLiteral: ";",
    },
  ] as expectedType[];

  const lexer = new Lexer(input);

  tests.forEach(({ expectedType, expectedLiteral }) => {
    const token = lexer.nextToken();
    expect(token.tokenType).toBe(expectedType);
    expect(token.literal).toBe(expectedLiteral);
  });
});
