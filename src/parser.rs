use combine::{ParseError, Stream};
use combine::primitives::Error;

use std::ascii::AsciiExt;
use std::fmt::{Display, Write};
use std::collections::HashMap;

use error::GameError;

pub fn cmp_ignore_case(l: char, r: char) -> bool {
    l.eq_ignore_ascii_case(&r)
}

pub fn match_first<'a, T>(needle: &str,
                          haystack: &'a HashMap<&str, T>)
                          -> Result<&'a T, GameError> {
    let lower_needle = needle.to_lowercase();
    let matching = haystack.iter()
        .filter(|&(key, _)| key.to_lowercase().starts_with(&lower_needle))
        .collect::<Vec<(&&str, &T)>>();
    match matching.len() {
        1 => Ok(matching[0].1),
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
    use std::collections::HashMap;
    use ::GameError;

    #[test]
    fn match_first_works() {
        let mut hm = HashMap::new();
        hm.insert("EGGBACON", 1);
        hm.insert("EGGcheese", 2);
        assert_eq!(Ok(&1), match_first("eggb", &hm));
        assert_eq!(Ok(&2), match_first("eggc", &hm));
        assert_eq!(Err(GameError::InvalidInput("Ambiguous".to_string())),
                   match_first("egg", &hm));
        assert_eq!(Err(GameError::InvalidInput("Couldn't find any matching options".to_string())),
                   match_first("bacon", &hm));
    }
}
