use crate::components::HealthBar;
use crate::constants::{FULL_HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT, TANK_MAX_HEALTH};
use crate::features::tank::components::Tank;
use bevy::prelude::{Children, Query, Rect, Sprite, Vec2};

pub fn update_health_bar(
    query: Query<(&Tank, &Children)>,
    mut health_bar_query: Query<(&mut Sprite, &HealthBar)>,
) {
    for (tank, children) in query.iter() {
        for &child in children.iter() {
            if let Ok((mut sprite, _)) = health_bar_query.get_mut(child) {
                // Calculate the current health percentage based on the Tank component
                let health_percentage = tank.health as f32 / TANK_MAX_HEALTH as f32;
                let full_health_bar_width = FULL_HEALTH_BAR_WIDTH;
                let current_health_bar_width = full_health_bar_width * health_percentage;

                let rect = Rect {
                    min: Vec2::new(0.0, 0.0),
                    max: Vec2::new(current_health_bar_width, HEALTH_BAR_HEIGHT),
                };
                sprite.rect = Some(rect);
            }
        }
    }
}
