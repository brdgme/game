use combine::{Parser, ParseError, Stream, parser};
use combine::primitives::{Error, ParseResult};
use combine::combinator::{FnParser, satisfy, many, many1, between, none_of, token, choice};

use std::ascii::AsciiExt;
use std::fmt::{Display, Write};

use error::GameError;

type FnP<T, I> = FnParser<I, fn(I) -> ParseResult<T, I>>;

pub fn arg<I>() -> FnP<String, I>
    where I: Stream<Item = char>
{
    fn arg_<I>(input: I) -> ParseResult<String, I>
        where I: Stream<Item = char>
    {
        choice([quoted_single, quoted_double, non_spaces]).parse_stream(input)
    }
    parser(arg_)
}

pub fn quoted_single<I>(input: I) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    between(token('\''), token('\''), many(none_of(vec!['\'']))).parse_stream(input)
}

pub fn quoted_double<I>(input: I) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    between(token('"'), token('"'), many(none_of(vec!['"']))).parse_stream(input)
}

pub fn non_spaces<I>(input: I) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    many1(satisfy(|c: char| !c.is_whitespace())).parse_stream(input)
}

pub fn cmp_ignore_case(l: char, r: char) -> bool {
    l.eq_ignore_ascii_case(&r)
}

pub fn match_first<'a, N, S, I, T>(needle: N, haystack: I) -> Result<&'a T, GameError>
    where S: 'a + Into<String> + Clone,
          N: Into<String>,
          T: Clone,
          I: Iterator<Item = &'a (S, T)>
{
    let lower_needle = needle.into().to_lowercase();
    let matching =
        haystack.filter(|&&(ref key, _)| key.to_owned().into().to_lowercase().starts_with(&lower_needle))
            .collect::<Vec<&'a (S, T)>>();
    match matching.len() {
        1 => Ok(&matching[0].1),
        0 => Err(GameError::InvalidInput("Couldn't find any matching options".to_string())),
        _ => Err(GameError::InvalidInput("Ambiguous".to_string())),
    }
}

pub fn to_game_error<S>(err: ParseError<S>) -> GameError
    where S: Stream,
          S::Item: Display,
          S::Range: Display,
          S::Position: Display
{
    let mut s = String::new();
    let mut written = false;

    // Output messages if there are any.
    for message in err.errors
        .iter()
        .filter(|e| {
            match **e {
                Error::Message(_) => true,
                _ => false,
            }
        }) {
        writeln!(s, "{}", message).unwrap();
        written = true;
    }
    if written {
        return GameError::InvalidInput(s);
    }

    // Output expected if there are any.
    let expected = || {
        err.errors
            .iter()
            .filter_map(|e| {
                match *e {
                    Error::Expected(ref ee) => Some(ee),
                    _ => None,
                }
            })
    };
    let expected_count = expected().count();
    for (i, err_message) in expected().enumerate() {
        write!(s,
               "{} '{}'",
               match i {
                   0 => "Expected",
                   _ if i < expected_count - 1 => ",",
                   _ => " or",
               },
               err_message)
            .unwrap();
        written = true;
    }
    if written {
        return GameError::InvalidInput(s);
    }

    GameError::InvalidInput("Invalid input".to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use combine::{Parser, parser};
    use ::GameError;

    #[test]
    fn match_first_works() {
        let hay: Vec<(&'static str, usize)> = vec![("EGGBACON", 1), ("EGGcheese", 2)];
        assert_eq!(Ok(&1), match_first("eggb", hay.iter()));
        assert_eq!(Ok(&2), match_first("eggc", hay.iter()));
        assert_eq!(Err(GameError::InvalidInput("Ambiguous".to_string())),
                   match_first("egg", hay.iter()));
        assert_eq!(Err(GameError::InvalidInput("Couldn't find any matching options".to_string())),
                   match_first("bacon", hay.iter()));
    }

    #[test]
    fn non_spaces_works() {
        assert_eq!(parser(non_spaces).parse("egg bacon cheese"),
                   Ok(("egg".to_string(), " bacon cheese")));
        assert_eq!(parser(non_spaces).parse("egg\nbacon cheese"),
                   Ok(("egg".to_string(), "\nbacon cheese")));
    }

    #[test]
    fn quoted_single_works() {
        assert_eq!(parser(quoted_single).parse("'egg bacon 'cheese"),
                   Ok(("egg bacon ".to_string(), "cheese")));
    }

    #[test]
    fn quoted_double_works() {
        assert_eq!(parser(quoted_double).parse("\"egg bacon \"cheese"),
                   Ok(("egg bacon ".to_string(), "cheese")));
    }

    #[test]
    fn arg_works() {
        assert_eq!(arg().parse("egg bacon cheese"),
                   Ok(("egg".to_string(), " bacon cheese")));
        assert_eq!(arg().parse("'egg bacon 'cheese"),
                   Ok(("egg bacon ".to_string(), "cheese")));
        assert_eq!(arg().parse("\"egg bacon \"cheese"),
                   Ok(("egg bacon ".to_string(), "cheese")));
    }
}
