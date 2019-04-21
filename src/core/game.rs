use super::super::helpers::{random, user_input};
use super::player;

const INITIAL_ROUNDS: i32 = 0;
const DEFAULT_PLAYERS: i32 = 1;

pub struct Game {
    pub players: Vec<player::Player>,
    pub rounds: i32,
    pub secret_number: i32,
}

impl Game {
    pub fn new() -> Game {
        println!("New Game!");
        let number_of_players = user_input::get("How many players?");
        let number_of_players: i32 = match number_of_players.trim().parse() {
            Ok(num) => num,
            Err(_) => DEFAULT_PLAYERS,
        };

        println!("We are playing {}", number_of_players);

        Game {
            players: Game::build_players(number_of_players),
            rounds: INITIAL_ROUNDS,
            secret_number: random::number(),
        }
    }

    pub fn build_players(number_of_players: i32) -> Vec<player::Player> {
        let mut players: Vec<player::Player> = vec![];

        for _ in 0..number_of_players {
            players.push(player::Player::new());
        }

        players
    }
}
