use std::io;

pub fn get(asking: &str) -> String {
    println!("{}", asking);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    input
}
