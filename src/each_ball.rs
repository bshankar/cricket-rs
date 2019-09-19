struct OutcomeData {
    score: u8,
    probability: f64,
}

pub struct PossibleOutcomes {
    runs: Vec<OutcomeData>,
    out: OutcomeData,
}

impl PossibleOutcomes {
    pub fn new(probabilities: Vec<f64>) -> Self {
        let (out, runs) = probabilities.split_last().unwrap();
        PossibleOutcomes {
            runs: (0..runs.len())
                .map(|i| OutcomeData {
                    score: i as u8,
                    probability: runs[i],
                })
                .collect(),
            out: OutcomeData {
                score: 0,
                probability: *out,
            },
        }
    }
}

enum Outcome {
    RUNS(u8),
    OUT,
}

// TODO: implement picking outcome with weighted probability
