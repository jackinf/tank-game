pub struct CommonHelpers;

impl CommonHelpers {
    /// generate a random number between base and base + 1 with 4 decimal places
    pub fn calculate_random_layer(base: f32) -> f32 {
        (base + (rand::random::<f32>() * 1.0)).round() * 10000.0 / 10000.0
    }
}
