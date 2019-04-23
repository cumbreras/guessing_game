use super::super::helpers::{mtc, parser, random, user_input};
use super::player;

const INITIAL_ROUNDS: i32 = 0;
const DEFAULT_PLAYERS: i32 = 1;
const TURNS: i32 = 8;

pub struct Game {
    pub players: Vec<player::Player>,
    pub rounds: i32,
    pub secret_number: i32,
}

impl Game {
    pub fn new() -> Game {
        println!("New Game!");
        Game {
            players: Game::build_players(),
            rounds: INITIAL_ROUNDS,
            secret_number: random::number(),
        }
    }

    fn build_players() -> Vec<player::Player> {
        let mut players: Vec<player::Player> = vec![];
        let number_of_players = Game::get_number_of_players();

        for _ in 0..number_of_players {
            players.push(player::Player::new());
        }

        players
    }

    fn get_number_of_players() -> i32 {
        let number_of_players = user_input::get("How many players?");
        return match number_of_players.trim().parse() {
            Ok(num) => {
                println!("We are playing {}", number_of_players);
                num
            }
            Err(_) => DEFAULT_PLAYERS,
        };
    }

    fn turn(self, p: &mut player::Player) -> bool {
        let guess = user_input::get(&format!("Give me a guess: {}", p.name));
        let guess: Option<i32> = parser::number(guess);

        return match guess {
            None => false,
            Some(guess) => {
                if mtc::number_cmp(guess, &self.secret_number) {
                    p.won();
                    println!(
                        "{} has won this turn, now has: {} points",
                        p.name,
                        p.score()
                    );
                    return true;
                }
                return false;
            }
        };
    }

    pub fn start(mut self) {
        for turn in 0..TURNS {
            println!("Turn number: {}", turn);

            for p in self.players.iter_mut() {
                *self.turn(p);
            }
        }

        println!("You lost");
    }
}
