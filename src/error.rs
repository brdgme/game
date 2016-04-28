use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    PlayerCount(PlayerCountError),
    InvalidInput(InvalidInputError),
    NotYourTurn,
    Error(String),
}

impl Error for GameError {
    fn description(&self) -> &str {
        match *self {
            GameError::PlayerCount(ref err) => err.description(),
            GameError::InvalidInput(ref err) => err.description(),
            GameError::NotYourTurn => "not your turn",
            GameError::Error(ref err) => err,
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameError::PlayerCount(ref err) => err.fmt(f),
            GameError::InvalidInput(ref err) => err.fmt(f),
            GameError::NotYourTurn => write!(f, "not your turn"),
            GameError::Error(ref err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
pub struct PlayerCountError {
    pub min: usize,
    pub max: usize,
    pub given: usize,
}

impl Error for PlayerCountError {
    fn description(&self) -> &str {
        "incorrect player count"
    }
}

impl fmt::Display for PlayerCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "incorrect player count: {}", self.given)
    }
}

#[derive(Debug)]
pub struct InvalidInputError {
    pub input: String,
    pub message: String,
}

impl Error for InvalidInputError {
    fn description(&self) -> &str {
        "invalid input"
    }
}

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid input: {}", self.message)
    }
}
