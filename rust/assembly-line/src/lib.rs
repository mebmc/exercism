// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        _ if speed >= 1 && speed <= 4 => (speed as f64 * 221.0),
        _ if speed >= 5 && speed <= 8 => (speed as f64 * 221.0) * 0.9,
        _ if speed >= 9 && speed <= 10 => (speed as f64 * 221.0) * 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    match speed {
        _ if speed >= 1 && speed <= 10 => (production_rate_per_hour(speed) / 60.0) as u32,
        _ => 0,
    }
}
