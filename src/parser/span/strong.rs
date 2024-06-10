use crate::parser::span::parse_spans;
use crate::parser::Span;
use crate::parser::Span::Strong;

pub fn parse_strong(text: &str) -> Option<(Span, usize)> {
    if let Some(caps) = crate::regex!(r"^__(?P<text>.+?)__").captures(text) {
        let t = caps.name("text").unwrap().as_str();
        return Some((Strong(parse_spans(t)), t.len() + 4));
    } else if let Some(caps) = crate::regex!(r"^\*\*(?P<text>.+?)\*\*").captures(text) {
        let t = caps.name("text").unwrap().as_str();
        return Some((Strong(parse_spans(t)), t.len() + 4));
    }
    None
}

#[cfg(test)]
mod test {
    use super::parse_strong;
    use crate::parser::Span::{Strong, Text};

    #[test]
    fn finds_strong() {
        assert_eq!(
            parse_strong("__testing things__ test"),
            Some((Strong(vec![Text("testing things".to_owned())]), 18))
        );

        assert_eq!(
            parse_strong("**testing things** test"),
            Some((Strong(vec![Text("testing things".to_owned())]), 18))
        );

        assert_eq!(
            parse_strong("__testing things__ things__ test"),
            Some((Strong(vec![Text("testing things".to_owned())]), 18))
        );

        assert_eq!(
            parse_strong("__w__ things_ test"),
            Some((Strong(vec![Text("w".to_owned())]), 5))
        );

        assert_eq!(
            parse_strong("**w** things** test"),
            Some((Strong(vec![Text("w".to_owned())]), 5))
        );

        assert_eq!(
            parse_strong("__w___ testing things test"),
            Some((Strong(vec![Text("w".to_owned())]), 5))
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_strong("__ testing things test"), None);
        assert_eq!(parse_strong("__testing things** test"), None);
        assert_eq!(parse_strong("____ testing things test"), None);
        assert_eq!(parse_strong("** test"), None);
        assert_eq!(parse_strong("**** test"), None);
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(parse_strong("were __testing things__ test"), None);
        assert_eq!(parse_strong("were **testing things** test"), None);
    }
}
