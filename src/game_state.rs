use crate::players::{get_player_data, Player};
use crate::weighted_score::{weighted_pick, Outcome};
use rand::rngs::ThreadRng;

#[derive(PartialEq)]
enum GameResult {
    BangaloreWins,
    ChennaiWins,
    Tie,
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

    fn game_ended(&self) -> bool {
        self.batsmen_left.len() == 1 || self.balls_left == 0 || self.runs_to_win <= 0
    }

    fn game_result(&self) -> Option<GameResult> {
        if self.game_ended() {
            if self.runs_to_win > 0 {
                Some(GameResult::ChennaiWins)
            } else if self.runs_to_win <= 0 {
                Some(GameResult::BangaloreWins)
            } else {
                Some(GameResult::Tie)
            }
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

    fn comment_balls_left(&self) {
        if self.balls_left % 6 == 0 {
            println!(
                "\n{} overs left. {} runs to win",
                self.balls_left / 6,
                self.runs_to_win
            );
        }
    }

    fn comment_outcome(&self, batting: usize, player: &Player, outcome: &Outcome) {
        match outcome {
            Outcome::OUT => println!(
                "{} {} ({}) is out!",
                player.name, self.batsmen_scores[batting], self.batsmen_balls[batting]
            ),
            Outcome::RUNS(r) => println!("{} scores {} runs", player.name, r),
        }
    }

    fn print_scoreboard(&self) {
        let players = get_player_data();
        for i in 0..self.batsmen_balls.len() {
            if self.batsmen_balls[i] != 0 {
                let player = &players[i];
                let not_out = if self.batsmen_left.contains(&i) {
                    "* "
                } else {
                    " "
                };

                println!(
                    "{} {}{}({})",
                    player.name, self.batsmen_scores[i], not_out, self.batsmen_balls[i]
                )
            }
        }
    }

    fn print_summary(&self) {
        if self.game_result() == Some(GameResult::BangaloreWins) {
            println!(
                "\n\nBangalore won by {} wickets!",
                self.batsmen_left.len() - 1
            );
        } else if self.game_result() == Some(GameResult::ChennaiWins) {
            println!("\n\nChennai won by {} runs!", self.runs_to_win);
        } else if self.game_result() == Some(GameResult::Tie) {
            println!("\n\nMatch tied between Bangalore and Chennai");
        }
        self.print_scoreboard();
    }

    pub fn simulate(&mut self, rng: &mut ThreadRng) {
        let players = get_player_data();
        while !self.game_ended() {
            let batting = self.batting.unwrap();
            let player = &players[batting];
            self.comment_balls_left();
            let outcome = &weighted_pick(&player.chances, rng);
            self.play(outcome);
            self.comment_outcome(batting, player, outcome);
        }
        self.print_summary();
    }
}
