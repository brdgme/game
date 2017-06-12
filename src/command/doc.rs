use brdgme_markup::Node;
use brdgme_color::GREY;

use command::Spec;

#[derive(Clone)]
pub struct Opts {
    pub split_one_of: bool,
    pub output_desc: bool,
    pub name: Option<String>,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            split_one_of: true,
            output_desc: true,
            name: None,
        }
    }
}

impl Spec {
    pub fn doc(&self, opts: &Opts) -> (Vec<Node>, Option<String>) {
        match *self {
            Spec::Int { min, max } => (doc_int(min, max), None),
            Spec::Token(ref token) => (doc_token(token), None),
            Spec::Enum { ref values, .. } => (doc_enum(values, opts), None),
            Spec::OneOf(ref specs) => (doc_one_of(specs, opts), None),
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
            } => doc_doc(name, desc, spec, opts),
            Spec::Player => (vec![Node::text("player")], None),
            Spec::Space => (vec![Node::text(" ")], None),
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
    let mut output: Vec<Node> = vec![];
    if !opts.split_one_of {
        output.push(Node::text("["));
    }
    for (k, s) in specs.iter().enumerate() {
        let (child_doc, desc_opt) = s.doc(&Opts {
                                              split_one_of: false,
                                              ..opts.clone()
                                          });
        if opts.split_one_of {
            // Split mode for top level, output a description above the child spec doc.
            if k > 0 {
                output.push(Node::text("\n"));
            }
            if let Some(desc) = desc_opt {
                output.push(Node::Fg(GREY.into(), vec![Node::text(desc)]));
                output.push(Node::text("\n  "));
            }
        } else {
            // Flat mode, ignore child spec desc.
            if k > 0 {
                output.push(Node::text(" | "));
            }
        }
        output.extend(child_doc);
    }
    if !opts.split_one_of {
        output.push(Node::text("]"));
    }
    output
}

fn doc_chain(specs: &[Spec], opts: &Opts) -> (Vec<Node>, Option<String>) {
    let mut desc: Option<String> = None;
    (specs
         .iter()
         .enumerate()
         .flat_map(|(i, s)| {
                       let (doc, desc_opt) = s.doc(opts);
                       if i == 0 {
                           desc = desc_opt;
                       }
                       doc
                   })
         .collect(),
     desc)
}

fn doc_opt(spec: &Spec, opts: &Opts) -> (Vec<Node>, Option<String>) {
    let (mut doc, desc) = spec.doc(opts);
    doc.push(Node::text("?"));
    (doc, desc)
}

fn doc_doc(name: &str,
           desc: &Option<String>,
           spec: &Spec,
           opts: &Opts)
           -> (Vec<Node>, Option<String>) {
    let (doc, child_desc) = spec.doc(&Opts {
                                         name: Some(name.to_owned()),
                                         ..*opts
                                     });
    (doc, desc.to_owned().or(child_desc))
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
                                                 },
                                                 Spec::Doc {
                                                     name: "discard".to_string(),
                                                     desc: Some("discard a card to an discard pile"
                                                                    .to_string()),
                                                     spec: Box::new(Spec::Chain(vec![
                    Spec::Token("discard".to_string()),
                    Spec::Space,
                    Spec::Doc {
                        name: "card".to_string(),
                        desc: Some("the card to discard".to_string()),
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
                                        .doc(&Opts::default())
                                        .0,
                                &[])));
    }
}
