use chrono::{NaiveDateTime, UTC};

use brdgme_markup::Node;

#[derive(Debug)]
pub struct Log {
    pub content: Vec<Node>,
    pub at: NaiveDateTime,
    pub public: bool,
    pub to: Vec<usize>,
}

impl Log {
    pub fn public(content: Vec<Node>) -> Log {
        Log {
            content: content,
            at: UTC::now().naive_utc(),
            public: true,
            to: vec![],
        }
    }

    pub fn private(content: Vec<Node>, to: Vec<usize>) -> Log {
        Log {
            content: content,
            at: UTC::now().naive_utc(),
            public: false,
            to: to,
        }
    }
}
