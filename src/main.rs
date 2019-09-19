use std::collections::HashMap;
mod each_ball;

fn get_player_data() -> HashMap<&'static str, Vec<f64>> {
    let mut players = HashMap::new();
    players.insert(
        "Kirat Boli",
        vec![0.05, 0.3, 0.25, 0.1, 0.15, 0.01, 0.09, 0.05],
    );
    players.insert("N.S Nodhi", vec![0.1, 0.4, 0.2, 0.05, 0.1, 0.01, 0.04, 0.1]);
    players.insert(
        "R Rumrah",
        vec![0.2, 0.3, 0.15, 0.05, 0.05, 0.01, 0.04, 0.2],
    );
    players.insert(
        "Shashi Henra",
        vec![0.3, 0.25, 0.05, 0.0, 0.05, 0.01, 0.04, 0.3],
    );
    players
}

fn main() {
    let mut rng = rand::thread_rng();
    let players = get_player_data();

    // TODO: Load initial state
    // Simulate each ball with weighted random number generator
    // Simulate changes after each ball
    // Until game ends
    println!(
        "{:?}",
        each_ball::weighted_pick(players.get("Kirat Boli").unwrap().to_vec(), &mut rng)
    );
    println!("Hello, world!");
}
