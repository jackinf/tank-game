use crate::common::constants::TANK_FULL_HEALTH_BAR_WIDTH;
use bevy::prelude::Component;

#[derive(Component)]
pub struct TankHealth {
    current: f32,
    max: f32,
}

impl TankHealth {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn get_current_health_bar_width(&self) -> f32 {
        let health_percentage = self.current / self.max;
        let full_health_bar_width = TANK_FULL_HEALTH_BAR_WIDTH;
        full_health_bar_width * health_percentage
    }
}

#[derive(Component)]
pub struct HealthBar;
