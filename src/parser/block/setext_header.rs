use crate::parser::span::parse_spans;
use crate::parser::Block;
use crate::parser::Block::Header;

pub fn parse_setext_header(lines: &[&str]) -> Option<(Block, usize)> {
    if lines.len() > 1 && !lines[0].is_empty() {
        if crate::regex!(r"^===+$").is_match(lines[1]) {
            return Some((Header(parse_spans(lines[0]), 1), 2));
        } else if crate::regex!(r"^---+$").is_match(lines[1]) {
            return Some((Header(parse_spans(lines[0]), 2), 2));
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::parse_setext_header;
    use crate::parser::Block::Header;
    use crate::parser::Span::Text;

    #[test]
    fn finds_atx_header() {
        assert_eq!(
            parse_setext_header(&vec!["Test", "=========="]).unwrap(),
            (Header(vec![Text("Test".to_owned())], 1), 2)
        );

        assert_eq!(
            parse_setext_header(&vec!["Test", "----------"]).unwrap(),
            (Header(vec![Text("Test".to_owned())], 2), 2)
        );

        assert_eq!(
            parse_setext_header(&vec!["This is a test", "==="]).unwrap(),
            (Header(vec![Text("This is a test".to_owned())], 1), 2)
        );

        assert_eq!(
            parse_setext_header(&vec!["This is a test", "---"]).unwrap(),
            (Header(vec![Text("This is a test".to_owned())], 2), 2)
        );
    }
}
