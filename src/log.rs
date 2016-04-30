use time;

pub struct Log<'a> {
    pub text: &'a str,
    pub at: time::Tm,
    pub public: bool,
    pub to: Vec<usize>,
}

pub fn new_public(text: &str) -> Log {
    Log {
        text: text,
        at: time::now(),
        public: true,
        to: vec![],
    }
}

pub fn new_private(text: &str, to: Vec<usize>) -> Log {
    Log {
        text: text,
        at: time::now(),
        public: false,
        to: to,
    }
}
