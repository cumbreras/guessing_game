pub fn number(guess: String, secret_number: &i32) -> Option<i32> {
    return match guess.trim().parse() {
        Ok(num) => Some(num),
        Err(val) => {
            if guess.trim() == "guess" {
                println!("The number is: {}", secret_number);
            } else {
                println!("{}: {}", val, guess);
            }
            None
        }
    };
}
