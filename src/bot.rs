use rand::{self, Rng};

use game::Gamer;
use command::Spec as CommandSpec;
use errors::*;

pub trait Botter<T: Gamer> {
    fn commands(&mut self,
                player: usize,
                pub_state: &T::PubState,
                players: &[String],
                command_spec: &CommandSpec)
                -> Vec<String>;
}

pub struct Fuzzer<G: Gamer, B: Botter<G>> {
    game: Option<G>,
    player_counts: Vec<usize>,
    player_names: Vec<String>,
    player_count: usize,
    bot: B,
    rng: rand::ThreadRng,
    game_count: usize,
    command_count: usize,
    invalid_input_count: usize,
}

impl<G: Gamer, B: Botter<G>> Fuzzer<G, B> {
    pub fn new(bot: B) -> Self {
        let player_counts = G::player_counts();
        Self {
            game: None,
            player_names: (0..player_counts.iter().max().cloned().unwrap_or(0))
                .map(|c| format!("{}", c))
                .collect(),
            player_counts: player_counts,
            player_count: 0,
            bot: bot,
            rng: rand::thread_rng(),
            game_count: 0,
            command_count: 0,
            invalid_input_count: 0,
        }
    }

    pub fn status(&self) -> String {
        format!("Games: {}\tCommands: {}\tInvalid inputs: {}",
                self.game_count,
                self.command_count,
                self.invalid_input_count)
    }
}

impl<G: Gamer, B: Botter<G>> Iterator for Fuzzer<G, B> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.game.as_ref().map(|g| g.is_finished()).unwrap_or(true) {
            self.game_count += 1;
            self.player_count = *self.rng
                                     .choose(&self.player_counts)
                                     .expect("no player counts for game type");
            self.game = Some(G::new(self.player_count)
                                 .expect("failed to create new game")
                                 .0);
        } else if let Some(ref mut game) = self.game {
            let player = *self.rng
                              .choose(&game.whose_turn())
                              .expect("is nobody's turn");
            let pub_state = game.pub_state(Some(player));
            let command_spec = game.command_spec(player).expect("expected a command spec");
            let input = self.bot
                .commands(player,
                          &pub_state,
                          &self.player_names[..self.player_count],
                          &command_spec)
                [0]
                    .to_owned();
            let cmd_res = game.command(player, &input, &self.player_names);
            self.command_count += 1;
            match cmd_res {
                Ok(..) => {}
                Err(Error(ErrorKind::InvalidInput(e), _)) => {
                    self.invalid_input_count += 1;
                    trace!("invalid input '{}' for player {}: {}", input, player, e)
                }
                _ => {
                    panic!("error running command '{}' for player {}, {:?}",
                           input,
                           player,
                           cmd_res)
                }
            }
        }
        Some(())
    }
}
