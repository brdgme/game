use std::collections::HashMap;

pub mod parser;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Kind {
    Int { min: Option<i32>, max: Option<i32> },
    Token(String),
    Ref(String),
    Enum(Vec<String>),
    OneOf(Vec<Spec>),
    Chain(Vec<Spec>),
}

impl Into<Spec> for Kind {
    fn into(self) -> Spec {
        self.spec()
    }
}

impl Kind {
    pub fn spec(self) -> Spec {
        Spec {
            kind: self,
            ..Default::default()
        }
    }

    pub fn token<T: Into<String>>(t: T) -> Self {
        Kind::Token(t.into())
    }

    pub fn int() -> Self {
        Kind::Int {
            min: None,
            max: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Spec {
    pub kind: Kind,
    pub min: usize,
    pub max: Option<usize>,
    pub description: Option<String>,
}

impl Default for Spec {
    fn default() -> Self {
        Spec {
            kind: Kind::Int {
                min: None,
                max: None,
            },
            min: 1,
            max: Some(1),
            description: None,
        }
    }
}

impl Spec {
    pub fn desc<T: Into<String>>(self, description: T) -> Self {
        Spec {
            description: Some(description.into()),
            ..self
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Specs {
    pub entry: Spec,
    pub specs: HashMap<String, Spec>,
}

impl Default for Specs {
    fn default() -> Self {
        Specs {
            entry: Kind::OneOf(vec![]).spec(),
            specs: HashMap::new(),
        }
    }
}

impl Specs {
    pub fn new(entry: Spec) -> Self {
        Self {
            entry: entry,
            ..Default::default()
        }
    }

    pub fn register<T: Into<String>>(&mut self, name: T, spec: Spec) {
        self.specs.insert(name.into(), spec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_acquire_command_specs_works() {
        let mut specs = Specs::default();
        specs.register("command",
                       Kind::OneOf(vec![Kind::Chain(vec![
                    Kind::token("play").spec().desc("play a tile to the board"),
                    Kind::Enum(vec![]).spec().desc("the tile to play"),
                ])
                                                .into(),
                                        Kind::Chain(vec![
                     Kind::token("buy").spec().desc("buy shares"),
                     Kind::int().spec().desc("the number of shares to buy"),
                     Kind::Enum(vec![]).spec().desc("the corporation to buy shares in"),
                                                          ])
                                                .into()])
                               .into());
        specs.entry = Kind::Ref("command".to_string()).spec();
    }
}