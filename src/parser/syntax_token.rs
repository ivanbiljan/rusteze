use crate::parser::token_type::TokenType;
use crate::parser::source_location::SourceLocation;

pub struct SyntaxToken {
    pub kind: TokenType,
    pub source_location: SourceLocation
}

impl SyntaxToken {
    pub fn new(kind: TokenType, source_location: SourceLocation) -> Self {
        SyntaxToken {
            kind,
            source_location
        }
    }

    pub fn is_unary_operator(&self) -> bool {
        match self.kind {
            TokenType::Bang => true,
            TokenType::MinusSign => true,
            _ => false
        }
    }
}