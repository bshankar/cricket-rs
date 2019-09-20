mod game_state;
mod players;
mod weighted_score;
use game_state::GameState;

fn main() {
    let mut rng = rand::thread_rng();
    let mut game = GameState::new();
    game.simulate(&mut rng);
}
