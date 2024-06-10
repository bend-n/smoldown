use crate::parser::span::parse_spans;
use crate::parser::Block;
use crate::parser::Block::Header;

pub fn parse_atx_header(lines: &[&str]) -> Option<(Block, usize)> {
    if let Some(caps) =
        crate::regex!(r"^(?P<level>#{1,6})\s(?P<text>.*?)(?:\s#*)?$").captures(lines[0])
    {
        return Some((
            Header(
                parse_spans(caps.name("text").unwrap().as_str()),
                caps.name("level").unwrap().as_str().len(),
            ),
            1,
        ));
    }
    None
}

#[cfg(test)]
mod test {
    use super::parse_atx_header;
    use crate::parser::Block::Header;
    use crate::parser::Span::Text;

    #[test]
    fn finds_atx_header() {
        assert_eq!(
            parse_atx_header(&vec!["### Test", "testtest"]).unwrap(),
            (Header(vec![Text("Test".to_owned())], 3), 1)
        );

        assert_eq!(
            parse_atx_header(&vec!["# Test", "testtest"]).unwrap(),
            (Header(vec![Text("Test".to_owned())], 1), 1)
        );

        assert_eq!(
            parse_atx_header(&vec!["###### Test", "testtest"]).unwrap(),
            (Header(vec![Text("Test".to_owned())], 6), 1)
        );

        assert_eq!(
            parse_atx_header(&vec!["### Test and a pretty long sentence", "testtest"]).unwrap(),
            (
                Header(vec![Text("Test and a pretty long sentence".to_owned())], 3),
                1
            )
        );
    }

    #[test]
    fn ignores_closing_hashes() {
        assert_eq!(
            parse_atx_header(&vec!["### Test ###", "testtest"]).unwrap(),
            (Header(vec![Text("Test".to_owned())], 3), 1)
        );

        assert_eq!(
            parse_atx_header(&vec!["# Test #", "testtest"]).unwrap(),
            (Header(vec![Text("Test".to_owned())], 1), 1)
        );

        assert_eq!(
            parse_atx_header(&vec!["###### Test ##", "testtest"]).unwrap(),
            (Header(vec![Text("Test".to_owned())], 6), 1)
        );

        assert_eq!(
            parse_atx_header(&vec![
                "### Test and a pretty long sentence #########",
                "testtest"
            ])
            .unwrap(),
            (
                Header(vec![Text("Test and a pretty long sentence".to_owned())], 3),
                1
            )
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_atx_header(&vec!["####### Test", "testtest"]), None);
        assert_eq!(parse_atx_header(&vec!["Test #", "testtest"]), None);
        assert_eq!(parse_atx_header(&vec!["T ### est #", "testtest"]), None);
    }
}
