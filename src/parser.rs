use combine::{ParseError, Stream};
use combine::primitives::Error;

use std::ascii::AsciiExt;
use std::fmt::{Display, Write};

use error::GameError;

pub fn cmp_ignore_case(l: char, r: char) -> bool {
    l.eq_ignore_ascii_case(&r)
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
