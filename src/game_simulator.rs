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
            let batting = self.batting;
            let player = &players[batting];
            self.print_balls_left();
            let outcome = &weighted_pick(&player.bins, rng);
            self.play(outcome);
            self.print_ball_outcome(batting, player.name, outcome);
        }
        self.print_summary();
        self.print_scoreboard(players);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::*;
    use crate::player_data::get_player_data;

    #[test]
    fn play_games() {
        let simulations = 10_000;

        for _i in 0..simulations {
            let mut rng = rand::thread_rng();
            let mut game = GameState::new();
            let initial_balls_left = game.balls_left;
            let initial_runs_left = game.runs_to_win;
            game.run(&get_player_data(), &mut rng);

            let balls_played: usize = game.batsmen_balls.iter().sum();
            assert_eq!(
                balls_played + game.balls_left,
                initial_balls_left,
                "Balls played ({}) + Balls left ({}) should be initial_balls_left ({})",
                balls_played,
                game.balls_left,
                initial_balls_left
            );

            let runs_played: usize = game.batsmen_scores.iter().sum();
            assert_eq!(
                runs_played as isize + game.runs_to_win,
                initial_runs_left as isize,
                "Runs played ({}) + Runs left ({}) should be initial_runs_left ({})",
                runs_played,
                game.runs_to_win,
                initial_runs_left
            );

            match game.game_result() {
                Some(GameResult::BangaloreWins) => {
                    assert!(
                        game.batsmen_left.len() > 1,
                        "Banglore won: Batsman left ({}) should be greater than 1",
                        game.batsmen_left.len()
                    );
                    assert!(
                        game.runs_to_win <= 0,
                        "Banglore won: Runs left ({}) should be less than or equal to 0",
                        game.balls_left
                    );
                }
                Some(GameResult::ChennaiWins) => {
                    assert!(
                        game.batsmen_left.len() == 1 || game.balls_left == 0,
                        "Chennai won: Either Batsman left ({}) should be 1 or game.balls_left should be zero",
                        game.batsmen_left.len()
                    );
                    assert!(
                        game.runs_to_win > 1,
                        "Chennai won: Runs left ({}) should be > 1",
                        game.balls_left
                    );
                }
                Some(GameResult::Tie) => {
                    assert!(
                        game.runs_to_win == 1,
                        "Chennai won: Runs left ({}) should be 1",
                        game.balls_left
                    );
                    assert!(
                        game.batsmen_left.len() == 1 || game.balls_left == 0,
                        "Game tied: Either Batsman left ({}) should be 1 or game.balls_left should be zero",
                        game.batsmen_left.len()
                    );
                }
                _ => panic!("Game did not end!"),
            }
        }
    }
}
