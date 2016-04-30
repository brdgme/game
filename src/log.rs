use time;

pub struct Log<'a> {
    pub text: &'a str,
    pub at: time::Tm,
    pub public: bool,
    pub to: Vec<usize>,
}

impl<'a> Log<'a> {
    pub fn public(text: &str) -> Log {
        Log {
            text: text,
            at: time::now(),
            public: true,
            to: vec![],
        }
    }

    pub fn private(text: &str, to: Vec<usize>) -> Log {
        Log {
            text: text,
            at: time::now(),
            public: false,
            to: to,
        }
    }
}
