use log::Log;

use ::error::GameError;

pub trait Gamer {
    fn start(&mut self, players: usize) -> Result<Vec<Log>, GameError>;
    fn is_finished(&self) -> bool;
    fn whose_turn(&self) -> Vec<usize>;

    fn assert_not_finished(&self) -> Result<(), GameError> {
        match self.is_finished() {
            true => Err(GameError::Finished),
            false => Ok(()),
        }
    }

    fn assert_players_turn(&self, player: usize) -> Result<(), GameError> {
        match self.whose_turn().iter().position(|&p| p == player) {
            Some(_) => Ok(()),
            None => Err(GameError::NotYourTurn),
        }
    }
}

pub trait Commander {
    fn command(&mut self, player: usize, input: &[u8]) -> Result<(Vec<Log>, &[u8]), GameError>;
}

#[test]
fn it_works() {}
