use crate::features::unit::components::UnitSelectionRect;
use bevy::asset::AssetServer;
use bevy::prelude::{default, Color, Commands, ResMut, Sprite, SpriteBundle, Transform};

pub fn spawn_unit_selection_rect(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands
        .spawn((SpriteBundle {
            texture: asset_server.load("pixels/white.png"),
            transform: Transform::from_xyz(0., 0., 100.),
            sprite: Sprite {
                color: Color::BLUE,
                ..default()
            },
            ..default()
        },))
        .insert(UnitSelectionRect::new());
}
