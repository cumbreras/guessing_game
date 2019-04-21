use std::cmp::Ordering;

pub fn number_cmp(guess: i32, secret_number: &i32) -> bool {
    return match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Small");
            false
        }
        Ordering::Greater => {
            println!("Big");
            false
        }
        Ordering::Equal => true,
    };
}
