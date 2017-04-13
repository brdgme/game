use log::Log;
use serde::Serialize;

use brdgme_markup::Node;

use std::collections::{HashSet, HashMap};

use error::GameError;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Stat {
    Int(i32),
    Float(f32),
    Set(HashSet<String>),
    Fraction(i32, i32),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Status {
    Active {
        whose_turn: Vec<usize>,
        eliminated: Vec<usize>,
    },
    Finished {
        winners: Vec<usize>,
        stats: Vec<HashMap<String, Stat>>,
    },
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub logs: Vec<Log>,
    pub can_undo: bool,
    pub remaining_input: String,
}

pub trait Gamer: Sized {
    type PubState: Serialize + Renderer;

    fn new(players: usize) -> Result<(Self, Vec<Log>), GameError>;
    fn pub_state(&self, player: Option<usize>) -> Self::PubState;
    fn command(&mut self,
               player: usize,
               input: &str,
               players: &[String])
               -> Result<CommandResponse, GameError>;
    fn status(&self) -> Status;

    fn is_finished(&self) -> bool {
        match self.status() {
            Status::Finished { .. } => true,
            _ => false,
        }
    }

    fn whose_turn(&self) -> Vec<usize> {
        match self.status() {
            Status::Active { whose_turn: wt, .. } => wt,
            _ => vec![],
        }
    }

    fn eliminated(&self) -> Vec<usize> {
        match self.status() {
            Status::Active { eliminated: e, .. } => e,
            _ => vec![],
        }
    }

    fn winners(&self) -> Vec<usize> {
        match self.status() {
            Status::Finished { winners, .. } => winners,
            _ => vec![],
        }
    }

    fn stats(&self) -> Vec<HashMap<String, Stat>> {
        match self.status() {
            Status::Finished { stats, .. } => stats,
            _ => vec![],
        }
    }

    fn assert_not_finished(&self) -> Result<(), GameError> {
        if self.is_finished() {
            Err(GameError::Finished)
        } else {
            Ok(())
        }
    }

    fn assert_player_turn(&self, player: usize) -> Result<(), GameError> {
        match self.whose_turn().iter().position(|&p| p == player) {
            Some(_) => Ok(()),
            None => Err(GameError::NotYourTurn),
        }
    }
}

pub trait Renderer {
    fn render(&self) -> Vec<Node>;
}

#[test]
fn it_works() {}
