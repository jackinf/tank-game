use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct CommonHelpers;

impl CommonHelpers {
    pub fn get_timestamp() -> f32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .or::<f32>(Ok(Duration::new(0, 0))) // i don't want to crash the game if time goes backwards
            .ok()
            .unwrap()
            .as_secs_f32()
    }

    /// generate a random number between base and base + 1 with 4 decimal places
    pub fn calculate_random_layer(base: f32) -> f32 {
        (base + (rand::random::<f32>() * 1.0)).round() * 10000.0 / 10000.0
    }
}
