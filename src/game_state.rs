use crate::weighted_score::Outcome;

#[derive(PartialEq, Debug)]
pub enum GameResult {
    BangaloreWins,
    ChennaiWins,
    Tie,
}

#[derive(PartialEq, Debug)]
pub struct GameState {
    pub runs_to_win: isize,
    pub balls_left: usize,
    pub batsmen_left: Vec<usize>,
    pub batsmen_scores: Vec<usize>,
    pub batsmen_balls: Vec<usize>,
    pub batting: Option<usize>,
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

    pub fn game_ended(&self) -> bool {
        self.batsmen_left.len() == 1 || self.balls_left == 0 || self.runs_to_win <= 0
    }

    pub fn game_result(&self) -> Option<GameResult> {
        if self.game_ended() {
            if self.runs_to_win > 1 {
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

    pub fn play(&mut self, outcome: &Outcome) {
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
                runs_to_win: 34,
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
                runs_to_win: 39,
                balls_left: 23,
                batsmen_scores: vec![1, 0, 0, 0],
                batsmen_balls: vec![1, 0, 0, 0],
                batting: Some(1),
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
                batting: Some(2),
                batsmen_left: vec![1, 2, 3],
                ..GameState::new()
            }
        )
    }

    #[test]
    fn over_end() {
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
                batting: Some(1),
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
                runs_to_win: 39,
                balls_left: 18,
                batsmen_balls: vec![1, 0, 0, 0],
                batsmen_scores: vec![1, 0, 0, 0],
                batting: Some(0),
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
            runs_to_win: 0,
            ..GameState::new()
        };
        assert_eq!(game_state.game_ended(), true);
    }
    #[test]
    fn game_end_balls_negative() {
        let game_state = GameState {
            runs_to_win: -5,
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
    fn game_result_balls_over_runs_left() {
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
            runs_to_win: 1,
            ..GameState::new()
        };
        assert_eq!(game_state.game_result(), Some(GameResult::Tie));
    }

    #[test]
    fn game_result_balls_over_runs_over() {
        let game_state = GameState {
            balls_left: 0,
            runs_to_win: 0,
            ..GameState::new()
        };
        assert_eq!(game_state.game_result(), Some(GameResult::BangaloreWins));
    }

    #[test]
    fn game_result_runs_over() {
        let game_state = GameState {
            runs_to_win: 0,
            ..GameState::new()
        };
        assert_eq!(game_state.game_result(), Some(GameResult::BangaloreWins));
    }

    #[test]
    fn game_result_runs_over_negative() {
        let game_state = GameState {
            runs_to_win: -5,
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
}
