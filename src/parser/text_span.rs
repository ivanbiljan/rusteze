pub struct TextSpan {
    start: u16,
    length: u16
}

impl TextSpan {
    pub fn from_bounds(start: u16, end: u16) -> TextSpan {
        TextSpan {
            start,
            length: end - start
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