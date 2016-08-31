use time;

use brdgme_markup::ast::Node;

#[derive(Debug)]
pub struct Log {
    pub content: Vec<Node>,
    pub at: time::Tm,
    pub public: bool,
    pub to: Vec<usize>,
}

impl Log {
    pub fn public(content: Vec<Node>) -> Log {
        Log {
            content: content,
            at: time::now(),
            public: true,
            to: vec![],
        }
    }

    pub fn private(content: Vec<Node>, to: Vec<usize>) -> Log {
        Log {
            content: content,
            at: time::now(),
            public: false,
            to: to,
        }
    }
}
