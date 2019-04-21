pub fn number(guess: String) -> Option<i32> {
    return match guess.trim().parse() {
        Ok(num) => Some(num),
        Err(val) => {
            println!("{}", val);
            None
        }
    };
}
