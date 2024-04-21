use crate::parser::token_type::TokenType;
use crate::parser::source_location::SourceLocation;

pub struct SyntaxToken {
    kind: TokenType,
    source_location: SourceLocation
}

impl SyntaxToken {
    pub fn is_unary_operator(&self) -> bool {
        match self.kind {
            TokenType::Bang => true,
            _ => false
        }
    }
}