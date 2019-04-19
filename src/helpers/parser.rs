pub fn number(secret_number: i32, guess: String) -> i32 {
    return match guess.trim().parse() {
        Ok(num) => num,
        Err(val) => {
            if guess.trim() == "guess" {
                println!("The number is: {}", secret_number);
            } else {
                println!("{}: {}", val, guess);
            }
            return -1;
        }
    };
}
