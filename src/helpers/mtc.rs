use std::cmp::Ordering;

pub fn number_cmp(guess: i32, secret_number: i32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Small");
        }
        Ordering::Greater => {
            println!("Big");
        }
        Ordering::Equal => {
            return true;
        }
    }

    return false;
}
