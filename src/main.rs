mod core;
mod helpers;

use self::core::{game, player};
use self::helpers::{mtc, parser, random};
use std::io;

fn main() {
    const TURNS: i32 = 8;
    let secret_number = random::number();
    let new_game = game::Game::new();
    let mut players: Vec<player::Player> = Vec::new();

    for _ in 0..new_game.players {
        let p: player::Player = player::Player::new();
        players.push(p);
    }

    for _t in 0..TURNS {
        println!("Turn: {}", _t);

        for p in players.iter() {
            println!("Give a guess: {}", p.name);
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed");

            let guess: i32 = parser::number(secret_number, guess);
            if guess == -1 {
                continue;
            }

            let result = mtc::number_cmp(guess, secret_number);
            if result == true {
                p.points += p.points + 1;
                println!("{} has won, now has: {} points", p.name, p.points);
                return;
            }
        }
    }

    println!("You lost");
}
