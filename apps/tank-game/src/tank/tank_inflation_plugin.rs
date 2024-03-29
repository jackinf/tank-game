use bevy::app::Update;
use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::tank::tank::Tank;

pub struct TankInflationPlugin;

impl Plugin for TankInflationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, inflate_tank);
    }
}

fn inflate_tank(mut query: Query<&mut Transform, With<Tank>>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        for mut transform in &mut query {
            transform.scale *= 1.25;
        }
    }
}
