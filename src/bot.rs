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
        trace!("Starting fuzz");
        let player_counts = T::player_counts();
        let player_names: Vec<String> = (0..player_counts.iter().max().cloned().unwrap_or(0))
            .map(|c| format!("{}", c))
            .collect();
        let mut rng = rand::thread_rng();
        let mut step = 0;
        let mut game_number = 0;
        loop {
            game_number += 1;
            trace!("Game {} starting", game_number);
            let player_count = *rng.choose(&player_counts)
                                    .expect("game returned no available player counts");
            let names = &player_names[..player_count];
            let (mut g, _) = T::new(player_count).expect("game failed to start");
            while !g.is_finished() {
                let player = *rng.choose(&g.whose_turn())
                                  .expect("no players in whose_turn");
                for c in Self::commands(player,
                                        g.pub_state(Some(player)),
                                        names,
                                        g.command_spec(player, names)) {
                    let cmd_res = g.command(0, &c, &[]);
                    match cmd_res {
                        Ok(..) => {}
                        Err(Error(ErrorKind::InvalidInput(_), _)) => trace!("Invalid input: {}", c),
                        _ => cmd_res.map(|_| ()).unwrap(),
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
