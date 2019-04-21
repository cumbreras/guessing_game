mod core;
mod helpers;

use self::core::game;
use helpers::{mtc, parser, user_input};

fn main() {
    const TURNS: i32 = 8;
    let mut current_game = game::Game::new();

    for turn in 0..TURNS {
        println!("Turn number: {}", turn);

        for p in current_game.players.iter_mut() {
            let guess = user_input::get(&format!("Give me a guess: {}", p.name));
            let guess: Option<i32> = parser::number(guess);

            match guess {
                None => continue,
                Some(guess) => {
                    if mtc::number_cmp(guess, &current_game.secret_number) {
                        p.won();
                        println!(
                            "{} has won this turn, now has: {} points",
                            p.name,
                            p.score()
                        );
                        return;
                    }
                }
            }
        }
    }

    println!("You lost");
}
