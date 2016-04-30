use log::Log;

pub trait Gamer {
    fn start(&mut self, players: usize) -> Result<Vec<Log>, String>;
    fn command(&mut self, player: usize, input: &[u8]) -> Result<Vec<Log>, String>;
    fn instructions(self, player: usize) -> String;
}

#[test]
fn it_works() {}
