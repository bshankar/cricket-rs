use rand::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Debug)]
pub enum Outcome {
    RUNS(usize),
    OUT,
}

fn weighted_pick_index(probs: &Vec<f64>, rng: &mut ThreadRng) -> usize {
    let random_value: f64 = rng.gen();
    let mut current_max = 0.0;
    for i in 0..probs.len() {
        current_max += probs[i];
        if random_value <= current_max {
            return i;
        }
    }
    probs.len() - 1
}

pub fn weighted_pick(probs: &Vec<f64>, rng: &mut ThreadRng) -> Outcome {
    match weighted_pick_index(&probs, rng) {
        7 => Outcome::OUT,
        s => Outcome::RUNS(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_distribution() {
        let mut rng = rand::thread_rng();
        let probs = vec![0.0, 0.2, 0.7, 0.1];

        let simulations = 1000000.0;
        let mut counts = vec![0.0; 4];
        for i in 0..1000000 {
            let a = weighted_pick_index(&probs, &mut rng);
            assert_ne!(a, 0, "A choice with zero probability was picked!");
            counts[a] += 1.0 / simulations;
        }
        println!("{:?} should be quite close to {:?}", counts, probs);
    }
}
