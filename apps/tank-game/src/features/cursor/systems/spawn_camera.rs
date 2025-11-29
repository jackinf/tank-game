use bevy::prelude::{Camera2d, Commands};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
