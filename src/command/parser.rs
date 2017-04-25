use std::marker::PhantomData;

use errors::*;

#[derive(Debug, PartialEq)]
pub struct Output<'a, T> {
    pub value: T,
    pub consumed: &'a str,
    pub remaining: &'a str,
}

pub trait Parser<T> {
    fn parse<'a>(&self, input: &'a str) -> Result<Output<'a, T>>;
}

pub struct Int {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

impl Parser<i32> for Int {
    fn parse<'a>(&self, input: &'a str) -> Result<Output<'a, i32>> {
        let mut found_digit = false;
        let consumed_count = input
            .chars()
            .enumerate()
            .take_while(|&(i, c)| if i == 0 && c == '-' {
                            true
                        } else if c.is_digit(10) {
                found_digit = true;
                true
            } else {
                false
            })
            .count();
        if !found_digit {
            bail!("expected integer");
        }
        let consumed = &input[..consumed_count];
        let value: i32 = consumed
            .parse()
            .chain_err(|| "failed to parse integer")?;
        if let Some(min) = self.min {
            if value < min {
                bail!("'{}' is less than the minimum '{}'", value, min);
            }
        }
        if let Some(max) = self.max {
            if value > max {
                bail!("'{}' is greater than the maximum '{}'", value, max);
            }
        }
        Ok(Output {
               value: value,
               consumed: consumed,
               remaining: &input[consumed_count..],
           })
    }
}

pub struct Map<T, O, F, TP>
    where F: Fn(T) -> O,
          TP: Parser<T>
{
    pub parser: TP,
    pub map: F,
    t_type: PhantomData<T>,
    o_type: PhantomData<O>,
}

impl<T, O, F, TP> Map<T, O, F, TP>
    where F: Fn(T) -> O,
          TP: Parser<T>
{
    pub fn new(parser: TP, map: F) -> Self {
        Self {
            parser: parser,
            map: map,
            t_type: PhantomData,
            o_type: PhantomData,
        }
    }
}

impl<T, O, F, TP> Parser<O> for Map<T, O, F, TP>
    where F: Fn(T) -> O,
          TP: Parser<T>
{
    fn parse<'a>(&self, input: &'a str) -> Result<Output<'a, O>> {
        let child_parse = self.parser.parse(input)?;
        Ok(Output {
               value: (self.map)(child_parse.value),
               consumed: child_parse.consumed,
               remaining: child_parse.remaining,
           })
    }
}

pub struct Whitespace {}

impl Parser<String> for Whitespace {
    fn parse<'a>(&self, input: &'a str) -> Result<Output<'a, String>> {
        let consumed = input.chars().take_while(|c| c.is_whitespace()).count();
        if consumed == 0 {
            bail!("expected whitespace");
        }
        Ok(Output {
               value: input[..consumed].to_owned(),
               consumed: &input[..consumed],
               remaining: &input[consumed..],
           })
    }
}

pub struct Chain2<A, B, PA, PB>
    where PA: Parser<A>,
          PB: Parser<B>
{
    pub a: PA,
    pub b: PB,
    a_type: PhantomData<A>,
    b_type: PhantomData<B>,
}

impl<A, B, PA, PB> Chain2<A, B, PA, PB>
    where PA: Parser<A>,
          PB: Parser<B>
{
    pub fn new(a: PA, b: PB) -> Self {
        Self {
            a: a,
            b: b,
            a_type: PhantomData,
            b_type: PhantomData,
        }
    }
}

impl<A, B, PA, PB> Parser<(A, B)> for Chain2<A, B, PA, PB>
    where PA: Parser<A>,
          PB: Parser<B>
{
    fn parse<'a>(&self, input: &'a str) -> Result<Output<'a, (A, B)>> {
        let lhs = self.a.parse(input)?;
        let sep = Whitespace {}.parse(&lhs.remaining);
        let remaining = sep.as_ref()
            .map(|s| s.remaining)
            .unwrap_or(lhs.remaining);
        let rhs = self.b.parse(&remaining)?;
        if !lhs.consumed.is_empty() && !rhs.consumed.is_empty() {
            if let Err(e) = sep {
                return Err(e);
            }
        }
        let consumed_len = lhs.consumed.len() +
                           sep.as_ref().map(|s| s.consumed.len()).unwrap_or(0) +
                           rhs.consumed.len();
        Ok(Output {
               value: (lhs.value, rhs.value),
               consumed: &input[..consumed_len],
               remaining: &input[consumed_len..],
           })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_parser_works() {
        let mut parser = Int {
            min: None,
            max: None,
        };
        parser
            .parse("fart")
            .expect_err("expected 'fart' to produce an error");
        assert_eq!(Output {
                       value: 10,
                       consumed: "10",
                       remaining: "",
                   },
                   parser.parse("10").expect("expected '10' to parse"));
        assert_eq!(Output {
                       value: 10,
                       consumed: "10",
                       remaining: " with bacon and cheese",
                   },
                   parser
                       .parse("10 with bacon and cheese")
                       .expect("expected '10 with bacon and cheese' to parse"));
        assert_eq!(Output {
                       value: -10,
                       consumed: "-10",
                       remaining: " with bacon and cheese",
                   },
                   parser
                       .parse("-10 with bacon and cheese")
                       .expect("expected '-10 with bacon and cheese' to parse"));
        parser
            .parse("-")
            .expect_err("expected '-' to produce an error");
        parser.min = Some(-5);
        parser
            .parse("-6")
            .expect_err("expected '-6' to produce an error when minimum is set");
        parser.max = Some(100);
        parser
            .parse("101")
            .expect_err("expected '101' to produce an error when maximum is set");
    }

    #[test]
    fn map_parser_works() {
        let parser = Map::new(Int {
                                  min: None,
                                  max: None,
                              },
                              |i| i.to_string());
        assert_eq!(Output {
                       value: "123".to_string(),
                       consumed: "00123",
                       remaining: "bacon",
                   },
                   parser
                       .parse("00123bacon")
                       .expect("expected '00123bacon' to parse"))
    }

    #[test]
    fn chain2_parser_works() {
        let parser = Chain2::new(Int {
                                     min: None,
                                     max: None,
                                 },
                                 Int {
                                     min: None,
                                     max: None,
                                 });
        assert_eq!(Output {
                       value: (123, 456),
                       consumed: "123 456",
                       remaining: "  chairs",
                   },
                   parser
                       .parse("123 456  chairs")
                       .expect("expected '123 456  chairs' to parse"))
    }
}
