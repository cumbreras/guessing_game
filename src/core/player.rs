use super::super::helpers::user_input;

const INITIAL_POINTS: i32 = 0;

pub struct Player {
    pub name: String,
    points: i32,
}

impl Player {
    pub fn new() -> Player {
        let player_name = user_input::get("Give me your name, please");
        println!("Welcome {}", player_name);

        Player {
            name: player_name,
            points: INITIAL_POINTS,
        }
    }

    pub fn won(&mut self) {
        self.points += 1
    }

    pub fn score(&self) -> i32 {
        self.points
    }
}
