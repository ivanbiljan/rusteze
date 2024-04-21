use std::iter::Peekable;
use std::ptr::eq;
use std::str::Chars;
use crate::parser::syntax_token::SyntaxToken;
use crate::parser::token_type::TokenType;

pub struct Lexer {
}

impl Lexer {
    pub fn new(source: &'static str) -> Self {
        Lexer { }
    }

    pub fn tokenize(&self, source: &'static str) -> Vec<SyntaxToken> {
        let mut tokens: Vec<SyntaxToken> = Vec::new();
        loop {
            match self.emit_token() {
                Some(token) => {
                    if matches!(token.kind, TokenType::EndOfFile) {
                        break;
                    }

                    tokens.push(token)
                },
                None => continue
            }
        }

        tokens
    }

    fn emit_token(&self) -> Option<SyntaxToken> {
        None
    }
}