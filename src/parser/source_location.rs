use crate::parser::text_span::TextSpan;

pub struct SourceLocation {
    pub line_number: u32,
    pub text_span: TextSpan
}

impl SourceLocation {
    pub fn new(line_number: u32, text_span: TextSpan) -> Self {
        SourceLocation {
            line_number,
            text_span
        }
    }
}