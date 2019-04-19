use std::io;

const INITIAL_ROUNDS: i16 = 0;
const DEFAULT_PLAYERS: i16 = 1;

pub struct Game {
    pub players: i16,
    pub rounds: i16,
}

impl Game {
    pub fn new() -> Game {
        println!("New Game!");
        println!("How Many Players");

        let mut number_of_players = String::new();

        io::stdin()
            .read_line(&mut number_of_players)
            .expect("Failed");

        let number_of_players: i16 = match number_of_players.trim().parse() {
            Ok(num) => num,
            Err(_) => DEFAULT_PLAYERS,
        };

        println!("We are playing {}", number_of_players);

        Game {
            players: number_of_players,
            rounds: INITIAL_ROUNDS,
        }
    }
}
