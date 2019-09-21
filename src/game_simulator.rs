use crate::game_commentator::Commentator;
use crate::game_state::GameState;
use crate::player_data::Player;
use crate::weighted_score::weighted_pick;
use rand::rngs::ThreadRng;

pub trait Simulator {
    fn run(&mut self, players: &Vec<Player>, rng: &mut ThreadRng);
}

impl Simulator for GameState {
    fn run(&mut self, players: &Vec<Player>, rng: &mut ThreadRng) {
        while !self.game_ended() {
            let player = &players[self.batting];
            self.print_balls_left();
            let outcome = &weighted_pick(&player.bins, rng);
            self.print_ball_outcome(self.batting, player.name, outcome);
            self.play(outcome);
        }
        self.print_summary();
        self.print_scoreboard(players);
    }
}
