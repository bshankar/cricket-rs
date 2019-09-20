use crate::game_state::{GameResult, GameState};
use crate::player_data::Player;
use crate::weighted_score::Outcome;

pub trait Commentator {
    fn print_balls_left(&self);
    fn print_ball_outcome(&self, batting: usize, player_name: &'static str, outcome: &Outcome);
    fn print_scoreboard(&self, players: &Vec<Player>);
    fn print_summary(&self);
}

impl Commentator for GameState {
    fn print_balls_left(&self) {
        if self.balls_left % 6 == 0 {
            println!(
                "\n{} overs left. {} runs to win",
                self.balls_left / 6,
                self.runs_to_win
            );
        }
    }

    fn print_ball_outcome(&self, batting: usize, player_name: &'static str, outcome: &Outcome) {
        match outcome {
            Outcome::OUT => println!(
                "{} - {} ({} balls) is out!",
                player_name, self.batsmen_scores[batting], self.batsmen_balls[batting]
            ),
            Outcome::RUNS(r) => println!("{} scores {} runs", player_name, r),
        }
    }

    fn print_scoreboard(&self, players: &Vec<Player>) {
        for i in 0..self.batsmen_balls.len() {
            if self.batsmen_balls[i] != 0 {
                let player = &players[i];
                let not_out = if self.batsmen_left.contains(&i) {
                    "* "
                } else {
                    " "
                };

                println!(
                    "{} - {}{}({} balls)",
                    player.name, self.batsmen_scores[i], not_out, self.batsmen_balls[i]
                )
            }
        }
    }

    fn print_summary(&self) {
        if self.game_result() == Some(GameResult::BangaloreWins) {
            println!(
                "\n\nBangalore won by {} wickets with {} balls left",
                self.batsmen_left.len() - 1,
                self.balls_left
            );
        } else if self.game_result() == Some(GameResult::ChennaiWins) {
            println!(
                "\n\nChennai won by {} runs with {} balls left",
                self.runs_to_win, self.balls_left
            );
        } else if self.game_result() == Some(GameResult::Tie) {
            println!("\n\nMatch tied between Bangalore and Chennai");
        }
    }
}
