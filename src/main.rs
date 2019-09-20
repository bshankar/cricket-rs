mod game_commentator;
mod game_simulator;
mod game_state;
mod player_data;
mod weighted_score;
use game_simulator::Simulator;

fn main() {
    let mut rng = rand::thread_rng();
    let mut game = game_state::GameState::new();
    game.run(&player_data::get_player_data(), &mut rng);
}
