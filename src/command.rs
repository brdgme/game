use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Kind {
    Str,
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
            kind: Kind::Str,
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

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Specs {
    pub entry: String,
    pub specs: HashMap<String, Spec>,
}

impl Specs {
    pub fn new<T: Into<String>>(entry: T) -> Self {
        Self {
            entry: entry.into(),
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
        let mut specs = Specs::new("command");
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
    }
}
