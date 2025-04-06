mod game_core;

use game_core::game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
