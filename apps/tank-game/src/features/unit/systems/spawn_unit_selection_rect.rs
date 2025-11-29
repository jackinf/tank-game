use crate::features::unit::components::UnitSelectionRect;
use bevy::asset::AssetServer;
use bevy::prelude::{default, Color, Commands, ResMut, Sprite, Transform};

pub fn spawn_unit_selection_rect(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands
        .spawn((
            Sprite {
                image: asset_server.load("pixels/white.png"),
                color: Color::from(bevy::color::palettes::basic::BLUE),
                ..default()
            },
            Transform::from_xyz(0., 0., 100.),
            UnitSelectionRect::new(),
        ));
}
