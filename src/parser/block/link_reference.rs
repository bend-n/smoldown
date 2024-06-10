use crate::parser::Block;
use crate::parser::Block::LinkReference;

pub fn parse_link_reference(lines: &[&str]) -> Option<(Block, usize)> {
    if let Some(caps) = crate::regex!("^\\s*\\[(?P<id>[^\\[\\]]+)\\]:\\s*(?P<url>\\S+)(?:\\s+(?:'(?P<title1>.*)'|\"(?P<title2>.*)\"|\\((?P<title3>.*?)\\)))?\n?").captures(lines[0]) {
        return Some((
            LinkReference(
                caps.name("id").unwrap().as_str().to_lowercase(),
                caps.name("url").unwrap().as_str().to_owned(),
                caps.name("title1")
                    .or_else(|| caps.name("title2"))
                    .or_else(|| caps.name("title3"))
                    .map(|s| s.as_str().to_owned()),
            ),
            1,
        ));
    }

    if let Some(caps1) = crate::regex!("^\\s*\\[(?P<id>[^\\[\\]]+)\\]:").captures(lines[0]) {
        if let Some(caps2) = crate::regex!("\\s*(?P<url>\\S+)(?:\\s+(?:'(?P<title1>.*)'|\"(?P<title2>.*)\"|\\((?P<title3>.*?)\\)))?\n?").captures(lines[1]) {
        return Some((
            LinkReference(
                caps1.name("id").unwrap().as_str().to_lowercase(),
                caps2.name("url").unwrap().as_str().to_owned(),
                caps2
                    .name("title1")
                    .or_else(|| caps2.name("title2"))
                    .or_else(|| caps2.name("title3"))
                    .map(|s| s.as_str().to_owned()),
            ),
            2,
        ));
    }
    }

    None
}

#[cfg(test)]
mod test {
    use super::parse_link_reference;
    use crate::parser::Block::LinkReference;

    #[test]
    fn finds_link_reference() {
        assert_eq!(
            parse_link_reference(&vec!["[Test]: https://example.com"]).unwrap(),
            (
                LinkReference("test".to_owned(), "https://example.com".to_owned(), None),
                1
            )
        );

        assert_eq!(
            parse_link_reference(&vec!["[Test]: https://example.com \"example\""]).unwrap(),
            (
                LinkReference(
                    "test".to_owned(),
                    "https://example.com".to_owned(),
                    Some("example".to_owned())
                ),
                1
            )
        );

        assert_eq!(
            parse_link_reference(&vec!["[Test]: https://example.com (example)"]).unwrap(),
            (
                LinkReference(
                    "test".to_owned(),
                    "https://example.com".to_owned(),
                    Some("example".to_owned())
                ),
                1
            )
        );

        assert_eq!(
            parse_link_reference(&vec!["[Test]: https://example.com 'example'"]).unwrap(),
            (
                LinkReference(
                    "test".to_owned(),
                    "https://example.com".to_owned(),
                    Some("example".to_owned())
                ),
                1
            )
        );

        assert_eq!(
            parse_link_reference(&vec!["[Test]:     https://example.com        'example'"])
                .unwrap(),
            (
                LinkReference(
                    "test".to_owned(),
                    "https://example.com".to_owned(),
                    Some("example".to_owned())
                ),
                1
            )
        );

        assert_eq!(
            parse_link_reference(&vec!["[Test]:", "https://example.com \"example\""]).unwrap(),
            (
                LinkReference(
                    "test".to_owned(),
                    "https://example.com".to_owned(),
                    Some("example".to_owned())
                ),
                2
            )
        );
    }
}
