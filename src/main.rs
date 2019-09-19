mod each_ball;
use each_ball::PossibleOutcomes;
use std::collections::HashMap;

fn get_player_data() -> HashMap<&'static str, PossibleOutcomes> {
    let mut players = HashMap::new();
    players.insert(
        "Kirat Boli",
        PossibleOutcomes::new(vec![0.05, 0.3, 0.25, 0.1, 0.15, 0.01, 0.09, 0.05]),
    );
    players.insert(
        "N.S Nodhi",
        PossibleOutcomes::new(vec![0.1, 0.4, 0.2, 0.05, 0.1, 0.01, 0.04, 0.1]),
    );
    players.insert(
        "R Rumrah",
        PossibleOutcomes::new(vec![0.2, 0.3, 0.15, 0.05, 0.05, 0.01, 0.04, 0.2]),
    );
    players.insert(
        "Shashi Henra",
        PossibleOutcomes::new(vec![0.3, 0.25, 0.05, 0.0, 0.05, 0.01, 0.04, 0.3]),
    );
    players
}

fn main() {
    let players = get_player_data();

    // TODO: Load initial state
    // Simulate each ball with weighted random number generator
    // Simulate changes after each ball
    // Until game ends
    println!("Hello, world!");
}
