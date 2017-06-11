use brdgme_markup::Node;

use command::Spec;

#[derive(Clone)]
pub struct Opts {
    pub split_one_of: bool,
    pub output_desc: bool,
    pub name: Option<String>,
    pub desc: Option<String>,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            split_one_of: true,
            output_desc: true,
            name: None,
            desc: None,
        }
    }
}

impl Spec {
    pub fn doc(&self, opts: &Opts) -> Vec<Node> {
        match *self {
            Spec::Int { min, max } => doc_int(min, max),
            Spec::Token(ref token) => doc_token(token),
            Spec::Enum { ref values, .. } => doc_enum(values, opts),
            Spec::OneOf(ref specs) => doc_one_of(specs, opts),
            Spec::Chain(ref specs) => doc_chain(specs, opts),
            Spec::Many {
                ref spec,
                min,
                max,
                ref delim,
            } => unimplemented!(),
            Spec::Opt(ref spec) => doc_opt(spec, opts),
            Spec::Doc {
                ref name,
                ref desc,
                ref spec,
            } => {
                spec.doc(&Opts {
                             name: Some(name.to_owned()),
                             desc: desc.to_owned(),
                             ..*opts
                         })
            }
            Spec::Player => vec![Node::text("player")],
            Spec::Space => vec![Node::text(" ")],
        }
    }
}

fn doc_int(min: Option<i32>, max: Option<i32>) -> Vec<Node> {
    match (min, max) {
        (None, None) => vec![Node::text("#")],
        (Some(min), Some(max)) if min == max => {
            vec![Node::Bold(vec![Node::text(format!("{}", min))])]
        }
        (min, Some(max)) => vec![Node::text(format!("{}-{}", min.unwrap_or(0), max))],
        (Some(min), None) => vec![Node::text(format!("{}+", min))],
    }
}

fn doc_token(token: &str) -> Vec<Node> {
    vec![Node::Bold(vec![Node::text(token)])]
}

fn doc_enum(values: &[String], opts: &Opts) -> Vec<Node> {
    if let Some(ref name) = opts.name {
        return vec![Node::text(format!("[{}]", name))];
    }
    vec![Node::text(format!("[{}]", values.join(" | ")))]
}

fn doc_one_of(specs: &[Spec], opts: &Opts) -> Vec<Node> {
    let join = Node::text(if opts.split_one_of { "\n" } else { " | " });
    let mut output: Vec<Node> = vec![];
    if !opts.split_one_of {
        output.push(Node::text("["));
    }
    for (k, s) in specs.iter().enumerate() {
        if k > 0 {
            output.push(join.clone());
        }
        output.extend(s.doc(&Opts {
                                split_one_of: false,
                                ..opts.clone()
                            }));
    }
    if !opts.split_one_of {
        output.push(Node::text("]"));
    }
    output
}

fn doc_chain(specs: &[Spec], opts: &Opts) -> Vec<Node> {
    specs.iter().flat_map(|s| s.doc(opts)).collect()
}

fn doc_opt(spec: &Spec, opts: &Opts) -> Vec<Node> {
    let mut doc = spec.doc(opts);
    doc.push(Node::text("]?"));
    doc
}

#[cfg(test)]
mod test {
    use super::*;
    use brdgme_markup::{ansi, transform};

    #[test]
    fn it_works() {
        println!("{}",
                 ansi(&transform(&Spec::OneOf(vec![Spec::Doc {
                                                       name: "play".to_string(),
                                                       desc: Some("play a card to an expedition"
                                                                      .to_string()),
                                                       spec: Box::new(Spec::Chain(vec![
                    Spec::Token("play".to_string()),
                    Spec::Space,
                    Spec::Doc {
                        name: "card".to_string(),
                        desc: Some("the card to play".to_string()),
                        spec: Box::new(Spec::Enum {
                            values: vec![
                                "BX".to_string(),
                                "B10".to_string(),
                                "G3".to_string(),
                            ],
                            exact: true,
                        }),
                    },
                    Spec::Space,
                    Spec::Token("from".to_string()),
                    Spec::Space,
                    Spec::Doc {
                        name: "card".to_string(),
                        desc: Some("the card to play".to_string()),
                        spec: Box::new(Spec::Enum {
                            values: vec![
                                "BX".to_string(),
                                "B10".to_string(),
                                "G3".to_string(),
                            ],
                            exact: true,
                        }),
                    },
                ])),
                                                   }])
                                          .doc(&Opts::default()),
                                 &[])));
    }
}
