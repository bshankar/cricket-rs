use crate::players::{get_player_data, Player};
use crate::weighted_score::{weighted_pick, Outcome};
use rand::rngs::ThreadRng;

#[derive(PartialEq)]
enum Winner {
    BANGALORE,
    CHENNAI,
}

pub struct GameState {
    runs_to_win: isize,
    balls_left: usize,
    batsmen_left: Vec<usize>,
    batsmen_scores: Vec<usize>,
    batsmen_balls: Vec<usize>,
    batting: Option<usize>,
    off_side: usize,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            runs_to_win: 40,
            balls_left: 24,
            batsmen_left: (0..=3).collect(),
            batsmen_scores: vec![0; 4],
            batsmen_balls: vec![0; 4],
            batting: Some(0),
            off_side: 1,
        }
    }

    fn winner(&self) -> Option<Winner> {
        let game_ended = self.batsmen_left.len() == 0 || self.balls_left == 0;
        if game_ended && self.runs_to_win > 0 {
            Some(Winner::CHENNAI)
        } else if self.runs_to_win <= 0 && self.batsmen_left.len() > 1 {
            Some(Winner::BANGALORE)
        } else {
            None
        }
    }

    fn next_batsman(&self) -> Option<usize> {
        for &w in &self.batsmen_left {
            if w != self.off_side {
                return Some(w);
            }
        }
        None
    }

    fn swap_batsmen(&mut self) {
        let batting = self.batting.unwrap();
        self.batting = Some(self.off_side);
        self.off_side = batting;
    }

    fn rotate_batsmen(&mut self, runs: usize) {
        if runs % 2 != 0 {
            self.swap_batsmen()
        }
        if self.balls_left % 6 == 0 {
            self.swap_batsmen()
        }
    }

    fn play(&mut self, outcome: &Outcome) {
        let batsman = self.batting.unwrap();
        self.balls_left -= 1;
        self.batsmen_balls[batsman] += 1;

        match outcome {
            Outcome::OUT => {
                self.batsmen_left.retain(|&b| b != batsman);
                self.batting = self.next_batsman();
            }
            Outcome::RUNS(r) => {
                self.runs_to_win -= *r as isize;
                self.batsmen_scores[batsman] += *r;
                self.rotate_batsmen(*r);
            }
        }
    }

    fn do_commentary(&self, player: &Player, outcome: &Outcome) {
        if self.balls_left % 6 == 0 {
            println!(
                "\n{} overs left. {} runs to win",
                self.balls_left / 6,
                self.runs_to_win
            );
        }

        let batsman = self.batting.unwrap();
        match outcome {
            Outcome::OUT => println!(
                "{} {} ({}) is out!",
                player.name, self.batsmen_scores[batsman], self.batsmen_balls[batsman]
            ),
            Outcome::RUNS(r) => println!("{} scores {} runs", player.name, r),
        }
    }

    fn print_summary(&self) {}

    pub fn simulate(&mut self, rng: &mut ThreadRng) {
        let players = get_player_data();
        while self.winner() == None {
            let player = &players[self.batting.unwrap()];
            let outcome = &weighted_pick(&player.chances, rng);
            self.do_commentary(player, outcome);
            self.play(outcome);
        }
        self.print_summary();
    }
}
