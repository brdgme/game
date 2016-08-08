use log::Log;

use ::error::GameError;

pub trait Gamer {
    fn start(&mut self, players: usize) -> Result<Vec<Log>, GameError>;
    fn is_finished(&self) -> bool;
    fn winners(&self) -> Vec<usize>;
    fn whose_turn(&self) -> Vec<usize>;

    fn assert_not_finished(&self) -> Result<(), GameError> {
        match self.is_finished() {
            true => Err(GameError::Finished),
            false => Ok(()),
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
    fn command(&mut self, player: usize, input: &[u8]) -> Result<(Vec<Log>, &[u8]), GameError>;
}

pub trait PlayerRenderer {
    fn render_for_player(&self, player: usize) -> Result<String, GameError>;
}

#[test]
fn it_works() {}
