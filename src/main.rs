mod data;
mod each_ball;
mod game_state;
use game_state::GameState;

fn main() {
    let mut rng = rand::thread_rng();
    let players = data::get_player_data();
    let game_state = GameState::new();

    // Simulate each ball with weighted random number generator
    // Simulate changes after each ball
    // Until game ends
    let probs = &players[0].chances;
    println!("{:?}", each_ball::weighted_pick(probs, &mut rng));
}
