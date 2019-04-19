use std::cmp::Ordering;

pub fn number_cmp(guess: u32, secret_number: u32) -> bool {
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
