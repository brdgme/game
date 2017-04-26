pub mod parser;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Spec {
    Int { min: Option<i32>, max: Option<i32> },
    Token(String),
    Enum(Vec<String>),
    OneOf(Vec<Spec>),
    Chain(Vec<Spec>),
    Many {
        spec: Box<Spec>,
        min: Option<usize>,
        max: Option<usize>,
        delim: String,
    },
    Opt(Box<Spec>),
}
