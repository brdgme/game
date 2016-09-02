use log::Log;

use ::error::GameError;
use brdgme_markup::ast::Node;

pub trait Gamer {
    fn start(&mut self, players: usize) -> Result<Vec<Log>, GameError>;
    fn is_finished(&self) -> bool;
    fn winners(&self) -> Vec<usize>;
    fn whose_turn(&self) -> Vec<usize>;

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

pub trait Eliminator {
    fn eliminated(&self) -> Vec<usize>;
}

pub trait Commander {
    fn command(&mut self,
               player: usize,
               input: &str,
               players: &[String])
               -> Result<Vec<Log>, GameError>;
}

pub trait Renderer {
    fn render(&self, player: Option<usize>) -> Result<Vec<Node>, GameError>;
}

#[test]
fn it_works() {}
