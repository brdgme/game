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
        trace!("Botter::fuzz: starting");
        let player_counts = T::player_counts();
        let player_names: Vec<String> = (0..player_counts.iter().max().cloned().unwrap_or(0))
            .map(|c| format!("{}", c))
            .collect();
        let mut rng = rand::thread_rng();
        let mut step = 0;
        let mut game_number = 0;
        'games: loop {
            game_number += 1;
            trace!("Botter::fuzz: game {} starting", game_number);
            let player_count = *rng.choose(&player_counts)
                                    .expect("game returned no available player counts");
            let names = &player_names[..player_count];
            let (mut g, _) = T::new(player_count).expect("game failed to start");
            assert!(!g.is_finished(),
                    "game was finished immediately after starting");
            while !g.is_finished() {
                let player = *rng.choose(&g.whose_turn())
                                  .expect("no players in whose_turn");
                for c in Self::commands(player,
                                        g.pub_state(Some(player)),
                                        names,
                                        g.command_spec(player, names)) {
                    trace!("Botter::fuzz: player {} command '{}'", player, c);
                    let cmd_res = g.command(player, &c, &[]);
                    match cmd_res {
                        Ok(..) => {}
                        Err(Error(ErrorKind::InvalidInput(_), _)) => {
                            trace!("Botter::fuzz: invalid input: {}", c)
                        }
                        _ => cmd_res.map(|_| ()).unwrap(),
                    }
                }
                step += 1;
                if step >= steps {
                    break 'games;
                }
            }
        }
        trace!("Botter::fuzz: completed {} games", game_number);
    }
}
