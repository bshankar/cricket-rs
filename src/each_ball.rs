use rand::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Debug)]
pub enum Outcome {
    RUNS(usize),
    OUT,
}

fn weighted_pick_index(probs: &Vec<f64>, rng: &mut ThreadRng) -> usize {
    let random_value: f64 = rng.gen();
    for i in 0..probs.len() {
        let current_max = match i {
            0 => probs[i],
            _ => probs[i] + probs[i - 1],
        };

        if random_value <= current_max {
            return i;
        }
    }
    probs.len() - 1
}

pub fn weighted_pick(probs: Vec<f64>, rng: &mut ThreadRng) -> Outcome {
    match weighted_pick_index(&probs, rng) {
        7 => Outcome::OUT,
        s => Outcome::RUNS(s),
    }
}
