pub mod parser;
pub mod doc;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type SpecID = u64;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Spec {
    Int {
        min: Option<i32>,
        max: Option<i32>,
    },
    Token(String),
    Enum {
        values: Vec<String>,
        exact: bool,
    },
    OneOf(Vec<SpecID>),
    Chain(Vec<SpecID>),
    Many {
        spec: SpecID,
        min: Option<usize>,
        max: Option<usize>,
        delim: String,
    },
    Opt(SpecID),
    Doc {
        name: String,
        desc: Option<String>,
        spec: SpecID,
    },
    Player,
    Space,
}

impl Spec {
    pub fn id(&self) -> SpecID {
        let mut hasher = DefaultHasher::new();
        format!("{:p}", self).hash(&mut hasher);
        hasher.finish()
    }
}

pub type SpecMap = HashMap<SpecID, Spec>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SpecStore {
    pub entry: SpecID,
    pub specs: SpecMap,
}

impl SpecStore {
    pub fn from_spec(spec: Spec) -> Self {
        let mut specs: SpecMap = HashMap::new();
        let id = spec.id();
        specs.insert(id, spec);
        SpecStore {
            entry: id,
            specs: specs,
        }
    }

    pub fn extend(&mut self, other: SpecStore) {
        self.specs.extend(other.specs);
    }
}
