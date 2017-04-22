use game::Gamer;
use command::Specs as CommandSpecs;

pub trait Botter<T: Gamer> {
    fn commands(player: usize, pub_state: T::PubState, command_spec: CommandSpecs) -> Vec<String>;

    fn fuzz(n: usize) {
        for _game_number in 0..n {
            let (mut g, _) = T::new(2).unwrap();
            for c in Self::commands(0, g.pub_state(Some(0)), g.command_spec(0, &[])) {
                g.command(0, &c, &[]).unwrap();
            }
        }
    }
}
