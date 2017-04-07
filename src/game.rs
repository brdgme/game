use log::Log;
use serde::Serialize;

use error::GameError;
use brdgme_markup::Node;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Status {
    Active {
        whose_turn: Vec<usize>,
        eliminated: Vec<usize>,
    },
    Finished { winners: Vec<usize> },
}

pub trait Gamer: Sized {
    type PubState: Serialize + Renderer;

    fn new(players: usize) -> Result<(Self, Vec<Log>), GameError>;
    fn pub_state(&self, player: Option<usize>) -> Self::PubState;
    fn command(&mut self,
               player: usize,
               input: &str,
               players: &[String])
               -> Result<(Vec<Log>, String), GameError>;
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
            Status::Finished { winners: w } => w,
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
