use time;

#[derive(Debug)]
pub struct Log {
    pub text: String,
    pub at: time::Tm,
    pub public: bool,
    pub to: Vec<usize>,
}

impl Log {
    pub fn public(text: String) -> Log {
        Log {
            text: text,
            at: time::now(),
            public: true,
            to: vec![],
        }
    }

    pub fn private(text: String, to: Vec<usize>) -> Log {
        Log {
            text: text,
            at: time::now(),
            public: false,
            to: to,
        }
    }
}
