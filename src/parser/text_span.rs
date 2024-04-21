pub struct TextSpan {
    pub start: usize,
    pub length: usize,
}

impl TextSpan {
    pub fn new(start: usize, length: usize) -> TextSpan {
        TextSpan { start, length }
    }

    pub fn from_bounds(start: usize, end: usize) -> TextSpan {
        TextSpan {
            start,
            length: end - start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bounds_returns_proper_length() {
        let text_span = TextSpan::from_bounds(5, 15);
        assert_eq!(text_span.start, 5);
        assert_eq!(text_span.length, 10);
    }
}
