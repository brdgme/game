use rand::{self, Rng};

use game::Gamer;
use command::Specs as CommandSpecs;
use errors::*;

pub trait Botter<T: Gamer> {
    fn commands(player: usize,
                pub_state: T::PubState,
                players: &[String],
                command_spec: CommandSpecs)
                -> Vec<String>;

    fn fuzz(steps: usize) {
        debug!("Starting fuzz");
        let player_counts = T::player_counts();
        let player_names: Vec<String> = (0..player_counts.iter().max().cloned().unwrap_or(0))
            .map(|c| format!("{}", c))
            .collect();
        let mut rng = rand::thread_rng();
        let mut step = 0;
        let mut game_number = 0;
        loop {
            game_number += 1;
            debug!("Game {} starting", game_number);
            let player_count = *rng.choose(&player_counts).unwrap();
            let names = &player_names[..player_count];
            let (mut g, _) = T::new(player_count).unwrap();
            while !g.is_finished() {
                let player = *rng.choose(&g.whose_turn()).unwrap();
                for c in Self::commands(player,
                                        g.pub_state(Some(player)),
                                        names,
                                        g.command_spec(player, names)) {
                    match g.command(0, &c, &[]) {
                        Ok(..) => {}
                        Err(Error(ErrorKind::InvalidInput(_), _)) => debug!("Invalid input: {}", c),
                        Err(e) => panic!(e),
                    }
                }
                step += 1;
                if step >= steps {
                    return;
                }
            }
        }
    }
}
