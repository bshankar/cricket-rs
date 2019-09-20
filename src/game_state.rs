use crate::weighted_score::Outcome;

#[derive(PartialEq)]
pub enum GameResult {
    BangaloreWins,
    ChennaiWins,
    Tie,
}

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
