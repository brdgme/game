use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    PlayerCount(usize, usize, usize),
    InvalidInput(String),
    NotYourTurn,
    Finished,
    Internal(String),
}

impl Error for GameError {
    fn description(&self) -> &str {
        match *self {
            GameError::PlayerCount(_, _, _) => "incorrect player count",
            GameError::InvalidInput(_) => "invalid input",
            GameError::NotYourTurn => "not your turn",
            GameError::Finished => "game is already finished",
            GameError::Internal(_) => "internal error",
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameError::PlayerCount(min, max, given) => {
                write!(f, "not for {} players, expecting {} to {}", given, min, max)
            }
            GameError::InvalidInput(ref message) => write!(f, "{}", message),
            GameError::NotYourTurn => write!(f, "not your turn"),
            GameError::Finished => write!(f, "game is already finished"),
            GameError::Internal(ref message) => write!(f, "{}", message),
        }
    }
}
