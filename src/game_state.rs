use crate::weighted_score::Outcome;
use std::mem::swap;

#[derive(PartialEq, Debug)]
pub enum GameResult {
    BangaloreWins,
    ChennaiWins,
    Tie,
}

#[derive(PartialEq, Debug)]
pub struct GameState {
    pub runs_left: usize,
    pub balls_left: usize,
    pub batsmen_left: Vec<usize>,
    pub batsmen_scores: Vec<usize>,
    pub batsmen_balls: Vec<usize>,
    pub batting: usize,
    off_side: usize,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            runs_left: 40,
            balls_left: 24,
            batsmen_left: (0..=3).collect(),
            batsmen_scores: vec![0; 4],
            batsmen_balls: vec![0; 4],
            batting: 0,
            off_side: 1,
        }
    }

    pub fn game_ended(&self) -> bool {
        self.batsmen_left.len() == 1 || self.balls_left == 0 || self.runs_left == 0
    }

    pub fn game_result(&self) -> Option<GameResult> {
        if self.game_ended() {
            if self.runs_left > 1 {
                Some(GameResult::ChennaiWins)
            } else if self.runs_left == 0 {
                Some(GameResult::BangaloreWins)
            } else {
                Some(GameResult::Tie)
            }
        } else {
            None
        }
    }

    fn next_batsman(&self) -> usize {
        let invalid_batsman = &self.batsmen_balls.len();
        *self
            .batsmen_left
            .iter()
            .find(|b| b != &&self.off_side)
            .unwrap_or_else(|| invalid_batsman)
    }

    fn rotate_batsmen(&mut self, runs: usize) {
        if runs % 2 != 0 {
            swap(&mut self.batting, &mut self.off_side)
        }
        if self.balls_left % 6 == 0 {
            swap(&mut self.batting, &mut self.off_side)
        }
    }

    pub fn play(&mut self, outcome: &Outcome) {
        let batting = self.batting;
        self.balls_left -= 1;
        self.batsmen_balls[batting] += 1;

        match outcome {
            Outcome::OUT => {
                self.batsmen_left.retain(|&b| b != batting);
                self.batting = self.next_batsman();
            }
            Outcome::RUNS(r) => {
                self.runs_left -= std::cmp::min(*r, self.runs_left);
                self.batsmen_scores[self.batting] += *r;
                self.rotate_batsmen(*r);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_runs_even() {
        let mut game_state = GameState::new();
        game_state.play(&Outcome::RUNS(6));
        assert_eq!(
            game_state,
            GameState {
                runs_left: 34,
                balls_left: 23,
                batsmen_scores: vec![6, 0, 0, 0],
                batsmen_balls: vec![1, 0, 0, 0],
                ..GameState::new()
            }
        );
    }

    #[test]
    fn score_runs_odd() {
        let mut game_state = GameState::new();
        game_state.play(&Outcome::RUNS(1));
        assert_eq!(
            game_state,
            GameState {
                runs_left: 39,
                balls_left: 23,
                batsmen_scores: vec![1, 0, 0, 0],
                batsmen_balls: vec![1, 0, 0, 0],
                batting: 1,
                off_side: 0,
                ..GameState::new()
            }
        );
    }

    #[test]
    fn player_out() {
        let mut game_state = GameState::new();
        game_state.play(&Outcome::OUT);
        assert_eq!(
            game_state,
            GameState {
                balls_left: 23,
                batsmen_balls: vec![1, 0, 0, 0],
                batting: 2,
                batsmen_left: vec![1, 2, 3],
                ..GameState::new()
            }
        )
    }

    #[test]
    fn over_end_even_runs() {
        let mut game_state = GameState {
            balls_left: 19,
            ..GameState::new()
        };
        game_state.play(&Outcome::RUNS(0));
        assert_eq!(
            game_state,
            GameState {
                balls_left: 18,
                batsmen_balls: vec![1, 0, 0, 0],
                batting: 1,
                off_side: 0,
                ..GameState::new()
            }
        )
    }

    #[test]
    fn over_end_and_odd_runs() {
        let mut game_state = GameState {
            balls_left: 19,
            ..GameState::new()
        };
        game_state.play(&Outcome::RUNS(1));
        assert_eq!(
            game_state,
            GameState {
                runs_left: 39,
                balls_left: 18,
                batsmen_balls: vec![1, 0, 0, 0],
                batsmen_scores: vec![1, 0, 0, 0],
                batting: 0,
                off_side: 1,
                ..GameState::new()
            }
        )
    }

    #[test]
    fn game_end_none() {
        let game_state = GameState {
            balls_left: 19,
            ..GameState::new()
        };
        assert_eq!(game_state.game_ended(), false);
    }

    #[test]
    fn game_end_balls_over() {
        let game_state = GameState {
            balls_left: 0,
            ..GameState::new()
        };
        assert_eq!(game_state.game_ended(), true);
    }

    #[test]
    fn game_end_runs_over() {
        let game_state = GameState {
            runs_left: 0,
            ..GameState::new()
        };
        assert_eq!(game_state.game_ended(), true);
    }

    #[test]
    fn game_end_wickets_over() {
        let game_state = GameState {
            batsmen_left: vec![0],
            ..GameState::new()
        };
        assert_eq!(game_state.game_ended(), true);
    }

    #[test]
    fn game_result_none() {
        let game_state = GameState {
            balls_left: 19,
            ..GameState::new()
        };
        assert_eq!(game_state.game_result(), None);
    }

    #[test]
    fn game_result_balls_over() {
        let game_state = GameState {
            balls_left: 0,
            ..GameState::new()
        };
        assert_eq!(game_state.game_result(), Some(GameResult::ChennaiWins));
    }

    #[test]
    fn game_result_balls_over_one_run() {
        let game_state = GameState {
            balls_left: 0,
            runs_left: 1,
            ..GameState::new()
        };
        assert_eq!(game_state.game_result(), Some(GameResult::Tie));
    }

    #[test]
    fn game_result_balls_over_runs_over() {
        let game_state = GameState {
            balls_left: 0,
            runs_left: 0,
            ..GameState::new()
        };
        assert_eq!(game_state.game_result(), Some(GameResult::BangaloreWins));
    }

    #[test]
    fn game_result_runs_over() {
        let game_state = GameState {
            runs_left: 0,
            ..GameState::new()
        };
        assert_eq!(game_state.game_result(), Some(GameResult::BangaloreWins));
    }

    #[test]
    fn game_result_wickets_over() {
        let game_state = GameState {
            batsmen_left: vec![0],
            ..GameState::new()
        };
        assert_eq!(game_state.game_result(), Some(GameResult::ChennaiWins));
    }

    #[test]
    fn game_result_wickets_over_one_run() {
        let game_state = GameState {
            batsmen_left: vec![0],
            runs_left: 1,
            ..GameState::new()
        };
        assert_eq!(game_state.game_result(), Some(GameResult::Tie));
    }
}
