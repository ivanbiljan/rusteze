use crate::parser::text_span::TextSpan;

pub struct SourceLocation {
    line_number: u32,
    text_span: TextSpan
}