use crate::parser::source_location::SourceLocation;
use crate::parser::syntax_token::SyntaxToken;
use crate::parser::text_span::TextSpan;
use crate::parser::token_type::TokenType;
pub struct Lexer {
    input: &'static str,
    line_number: u32,
    position: usize,
}

impl Lexer {
    pub fn new(source: &'static str) -> Self {
        Lexer {
            input: source,
            line_number: 1,
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<SyntaxToken> {
        let mut tokens: Vec<SyntaxToken> = Vec::new();
        loop {
            match self.emit_token() {
                Some(token) => {
                    if matches!(token.kind, TokenType::EndOfFile) {
                        break;
                    }

                    tokens.push(token)
                }
                None => continue,
            }
        }

        tokens
    }

    fn emit_token(&mut self) -> Option<SyntaxToken> {
        if self.position >= self.input.len() {
            return Some(SyntaxToken::new(
                TokenType::EndOfFile,
                SourceLocation::new(self.line_number, TextSpan::new(self.position, 1)),
            ));
        }

        let mut token_type: TokenType = TokenType::Illegal;
        let start = self.position;

        match self.input.chars().nth(self.position) {
            Some(char) if char == '-' => {
                token_type = TokenType::MinusSign;
                self.position += 1
            }
            Some(char) if char == '+' => {
                token_type = TokenType::PlusSign;
                self.position += 1
            }
            Some(char) if char == '*' => {
                token_type = TokenType::AsteriskSign;
                self.position += 1
            }
            Some(char) if char == '/' => {
                token_type = TokenType::SlashSign;
                self.position += 1
            }
            Some(char) if char.is_digit(10) => {
                while self.position < self.input.len()
                    && self.input.chars().nth(self.position).unwrap().is_digit(10)
                {
                    self.position += 1;
                }

                let num = self.input[start..self.position].parse::<f64>().unwrap();

                return Some(SyntaxToken::new(
                    TokenType::Number(num),
                    SourceLocation::new(
                        self.line_number,
                        TextSpan::from_bounds(start, self.position),
                    ),
                ));
            }
            Some(char) if char.is_whitespace() => {
                while self
                    .input
                    .chars()
                    .nth(self.position)
                    .unwrap()
                    .is_whitespace()
                {
                    self.position += 1;
                }

                return None;
            }
            Some(char) if char == 0xA as char => {
                while self.input.chars().nth(self.position).unwrap() == 0xA as char {
                    self.line_number += 1;
                    self.position += 1;
                }

                return None;
            }
            _ => {}
        }

        Some(SyntaxToken::new(
            token_type,
            SourceLocation::new(
                self.line_number,
                TextSpan::from_bounds(start, self.position),
            ),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::lexer::Lexer;
    use crate::parser::token_type::TokenType;

    #[test]
    fn tokenize() {
        let mut lexer = Lexer::new("1 + 2 / 3");

        let tokens = lexer.tokenize();

        assert!(matches!(tokens.len(), 5));
        assert!(matches!(
            tokens.get(0).unwrap().kind,
            TokenType::Number(1f64)
        ));
        assert!(matches!(tokens.get(1).unwrap().kind, TokenType::PlusSign));
        assert!(matches!(
            tokens.get(2).unwrap().kind,
            TokenType::Number(2f64)
        ));
        assert!(matches!(tokens.get(3).unwrap().kind, TokenType::SlashSign));
        assert!(matches!(
            tokens.get(4).unwrap().kind,
            TokenType::Number(3f64)
        ));
    }
}
