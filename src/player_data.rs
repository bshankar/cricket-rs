pub struct Player {
    pub name: &'static str,
    pub bins: Vec<f64>,
}

fn check_chances(chances: &Vec<f64>) {
    assert!(
        chances.len() == 8,
        "Length of chances vector should be 7. Found: {}",
        chances.len()
    );
    let total_probability: f64 = chances.iter().sum();
    assert!(
        (total_probability - 1.0).abs() < 1e-12,
        "Total probability: {} should be 1",
        total_probability
    );
}

impl Player {
    fn new(name: &'static str, chances: Vec<f64>) -> Self {
        check_chances(&chances);

        Player {
            name: name,
            bins: to_bins(&chances),
        }
    }
}

fn to_bins(chances: &Vec<f64>) -> Vec<f64> {
    (0..chances.len())
        .map(|i| chances.iter().take(i + 1).sum())
        .collect()
}

pub fn get_player_data() -> Vec<Player> {
    vec![
        Player::new(
            "Kirat Boli",
            vec![0.05, 0.3, 0.25, 0.1, 0.15, 0.01, 0.09, 0.05],
        ),
        Player::new("N.S Nodhi", vec![0.1, 0.4, 0.2, 0.05, 0.1, 0.01, 0.04, 0.1]),
        Player::new(
            "R Rumrah",
            vec![0.2, 0.3, 0.15, 0.05, 0.05, 0.01, 0.04, 0.2],
        ),
        Player::new(
            "Shashi Henra",
            vec![0.3, 0.25, 0.05, 0.0, 0.05, 0.01, 0.04, 0.3],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choices_to_bins() {
        let chances = vec![0.0, 0.1, 0.2, 0.3, 0.1, 0.05, 0.01, 0.24];
        let bins = vec![0.0, 0.1, 0.3, 0.6, 0.7, 0.75, 0.76, 1.0];
        to_bins(&chances).iter().zip(bins).for_each(|(u, v)| {
            assert!(
                (u - v).abs() < 1e-12,
                "Bins computed are wrong! u: {} != bins[i]: {}",
                u,
                v
            )
        });
    }
}
