mod core;
mod helpers;

use self::core::game;

fn main() {
    game::Game::new().start()
}
