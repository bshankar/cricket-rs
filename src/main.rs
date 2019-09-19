mod data;
mod each_ball;
mod game_state;
use game_state::GameState;

fn main() {
    let mut rng = rand::thread_rng();
    let mut game = GameState::new();
    game.simulate(&mut rng);
}
