use crate::each_ball::Outcome;

enum Winner {
    BANGALORE,
    CHENNAI,
}

pub struct GameState {
    runs_to_win: isize,
    balls_left: usize,
    wickets_left: Vec<usize>,
    batting: Option<usize>,
    off_side: usize,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            runs_to_win: 40,
            balls_left: 24,
            wickets_left: (0..=3).collect(),
            batting: Some(0),
            off_side: 1,
        }
    }

    fn winner(&self) -> Option<Winner> {
        if (self.wickets_left.len() == 0 || self.balls_left == 0) && self.runs_to_win > 0 {
            Some(Winner::CHENNAI)
        } else if self.runs_to_win <= 0 && (self.balls_left >= 0 && self.wickets_left.len() > 0) {
            Some(Winner::BANGALORE)
        } else {
            None
        }
    }

    fn next_batsman(&self) -> Option<usize> {
        for &w in &self.wickets_left {
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

    fn play(&mut self, outcome: Outcome) {
        self.balls_left -= 1;

        match outcome {
            Outcome::OUT => {
                let batting = self.batting.unwrap();
                self.wickets_left.retain(|&w| w != batting);
                self.batting = self.next_batsman();
            }
            Outcome::RUNS(r) => {
                self.runs_to_win -= r as isize;
                if r % 2 != 0 {
                    self.swap_batsmen()
                }
            }
        }
    }
}
