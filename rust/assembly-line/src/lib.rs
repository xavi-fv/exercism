const CARS_PRODUCED_PER_HOUR: u8 = 221;

fn success_rate(speed: u8) -> Option<f64> {
    match speed {
        1..=4 => Some(1.),
        5..=8 => Some(0.9),
        9|10 => Some(0.77),
        _ => None
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if let Some(sr)  = success_rate(speed) {
        (speed as f64 * CARS_PRODUCED_PER_HOUR as f64) * sr
    } else {
        0.
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.) as u32
}
