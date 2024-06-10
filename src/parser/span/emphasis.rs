use crate::parser::span::parse_spans;
use crate::parser::Span;
use crate::parser::Span::Emphasis;

pub fn parse_emphasis(text: &str) -> Option<(Span, usize)> {
    if let Some(caps) = crate::regex!(r"^_(?P<text>.+?)_").captures(text) {
        let t = caps.name("text").unwrap().as_str();
        return Some((Emphasis(parse_spans(t)), t.len() + 2));
    } else if let Some(caps) = crate::regex!(r"^\*(?P<text>.+?)\*").captures(text) {
        let t = caps.name("text").unwrap().as_str();
        return Some((Emphasis(parse_spans(t)), t.len() + 2));
    }
    None
}

#[cfg(test)]
mod test {
    use super::parse_emphasis;
    use crate::parser::Span::{Emphasis, Text};

    #[test]
    fn finds_emphasis() {
        assert_eq!(
            parse_emphasis("_testing things_ test"),
            Some((Emphasis(vec![Text("testing things".to_owned())]), 16))
        );

        assert_eq!(
            parse_emphasis("*testing things* test"),
            Some((Emphasis(vec![Text("testing things".to_owned())]), 16))
        );

        assert_eq!(
            parse_emphasis("_testing things_ things_ test"),
            Some((Emphasis(vec![Text("testing things".to_owned())]), 16))
        );

        assert_eq!(
            parse_emphasis("_w_ things_ test"),
            Some((Emphasis(vec![Text("w".to_owned())]), 3))
        );

        assert_eq!(
            parse_emphasis("*w* things* test"),
            Some((Emphasis(vec![Text("w".to_owned())]), 3))
        );

        assert_eq!(
            parse_emphasis("_w__ testing things test"),
            Some((Emphasis(vec![Text("w".to_owned())]), 3))
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_emphasis("__ testing things test"), None);
        assert_eq!(parse_emphasis("_ test"), None);
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(parse_emphasis("were _testing things_ test"), None);
        assert_eq!(parse_emphasis("were *testing things* test"), None);
    }
}
