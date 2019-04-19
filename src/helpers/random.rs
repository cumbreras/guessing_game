use rand::Rng;

pub fn number() -> u32 {
    return rand::thread_rng().gen_range(1, 101);
}
