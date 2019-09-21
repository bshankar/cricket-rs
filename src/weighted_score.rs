use rand::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Debug)]
pub enum Outcome {
    RUNS(usize),
    OUT,
}

fn weighted_pick_index(bins: &Vec<f64>, rng: &mut ThreadRng) -> usize {
    let random_value: f64 = rng.gen();
    (0..bins.len())
        .find(|&i| random_value <= bins[i])
        .unwrap_or_else(|| bins.len() - 1)
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
    fn weighted_distribution() {
        let mut rng = rand::thread_rng();
        let probs = vec![0.0, 0.2, 0.7, 0.1];
        let bins = vec![0.0, 0.2, 0.9, 1.0];

        let simulations = 100_000.0;
        let mut counts: Vec<f64> = vec![0.0; 4];
        for _i in 0..simulations as usize {
            let a = weighted_pick_index(&bins, &mut rng);
            assert_ne!(a, 0, "A choice with zero probability was picked!");
            counts[a] += 1.0 / simulations;
        }

        // counts approaches probs as simulations -> oo
        // this test passes with a very high probability
        (0..counts.len()).for_each(|i| assert!((probs[i] - counts[i]).abs() < 0.01));
    }
}
