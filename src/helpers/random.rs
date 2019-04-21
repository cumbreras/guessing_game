use rand::Rng;

pub fn number() -> i32 {
    rand::thread_rng().gen_range(1, 101)
}
