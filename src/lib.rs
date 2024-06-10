//! A crate for parsing Markdown in Rust
#![deny(missing_docs)]

mod parser;

pub use parser::{Block, ListItem, Span};

/// Converts a Markdown string to a tokenset of Markdown items
pub fn tokenize(text: &str) -> Vec<Block> {
    parser::parse(text)
}

macro_rules! regex {
    ($r:literal) => {{
        crate::regex!(R = $r);
        &*R
    }};
    ($as:ident = $r:literal) => {
        static $as: crate::LazyLock<regex::Regex> =
            crate::LazyLock::new(|| regex::Regex::new($r).unwrap());
    };
}
use regex;

struct LazyLock<T, F = fn() -> T> {
    data: ::std::sync::OnceLock<T>,
    f: F,
}

impl<T, F> LazyLock<T, F> {
    pub const fn new(f: F) -> LazyLock<T, F> {
        Self {
            data: ::std::sync::OnceLock::new(),
            f,
        }
    }
}

impl<T> ::std::ops::Deref for LazyLock<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.get_or_init(self.f)
    }
}
