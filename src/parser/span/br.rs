use crate::parser::Span;
use crate::parser::Span::Break;

pub fn parse_break(text: &str) -> Option<(Span, usize)> {
    crate::regex!(r"^ {2}$")
        .is_match(text)
        .then_some((Break, 2))
}

#[cfg(test)]
mod test {
    use super::parse_break;
    use crate::parser::Span::Break;

    #[test]
    fn finds_breaks() {
        assert_eq!(parse_break("  "), Some((Break, 2)));
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_break("this is a test  "), None);
        assert_eq!(parse_break(" "), None);
        assert_eq!(parse_break("  a"), None);
    }
}
