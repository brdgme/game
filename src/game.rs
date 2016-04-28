pub trait Game {
    fn start(&mut self, players: usize) -> Result<(), String>;
    fn command(&mut self, player: usize, input: &[u8]) -> Result<(), String>;
}

#[test]
fn it_works() {}
