use rand::Rng;

pub fn roll_single_die() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}

pub fn roll_two_dice() -> (u8, u8) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(1..=6), rng.gen_range(1..=6))
}

pub fn get_putting_strokes(green_tier: u8) -> u16 {
    match green_tier {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    }
}
