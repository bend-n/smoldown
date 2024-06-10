use crate::parser::Block;
use crate::parser::Block::Hr;

pub fn parse_hr(lines: &[&str]) -> Option<(Block, usize)> {
    crate::regex!(r"^(===+)$|^(---+)$")
        .is_match(lines[0])
        .then_some((Hr, 1))
}

#[cfg(test)]
mod test {
    use super::parse_hr;
    use crate::parser::Block::Hr;

    #[test]
    fn finds_hr() {
        assert_eq!(parse_hr(&vec!["-------"]).unwrap(), (Hr, 1));
        assert_eq!(parse_hr(&vec!["---"]).unwrap(), (Hr, 1));
        assert_eq!(
            parse_hr(&vec!["----------------------------"]).unwrap(),
            (Hr, 1)
        );
        assert_eq!(parse_hr(&vec!["-------", "abc"]).unwrap(), (Hr, 1));

        assert_eq!(parse_hr(&vec!["======="]).unwrap(), (Hr, 1));
        assert_eq!(parse_hr(&vec!["==="]).unwrap(), (Hr, 1));
        assert_eq!(
            parse_hr(&vec!["============================"]).unwrap(),
            (Hr, 1)
        );
        assert_eq!(parse_hr(&vec!["=======", "abc"]).unwrap(), (Hr, 1));
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_hr(&vec!["a-------"]), None);
        assert_eq!(parse_hr(&vec!["--- a"]), None);
        assert_eq!(parse_hr(&vec!["--a-"]), None);
        assert_eq!(parse_hr(&vec!["-------====--------------"]), None);

        assert_eq!(parse_hr(&vec!["a======"]), None);
        assert_eq!(parse_hr(&vec!["=== a"]), None);
        assert_eq!(parse_hr(&vec!["==a="]), None);
        assert_eq!(parse_hr(&vec!["=======---================="]), None);
    }
}
