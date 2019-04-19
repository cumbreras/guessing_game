use std::io;

pub struct Player {
    pub name: String,
    pub points: i16,
}

impl Player {
    pub fn new() -> Player {
        println!("Name please");
        let mut player_name = String::new();
        io::stdin().read_line(&mut player_name).expect("Failed");
        println!("Welcome {}", player_name);

        Player {
            name: player_name,
            points: 0,
        }
    }
}
