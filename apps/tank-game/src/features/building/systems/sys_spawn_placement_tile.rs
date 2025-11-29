use crate::constants::TILE_SIZE;
use crate::features::building::components::GlobalBuildingPlacementTiles;
use bevy::prelude::{
    default, AssetServer, Color, Commands, Res, Sprite, Transform, Vec2, Vec3,
};

pub fn sys_spawn_placement_tile(mut commands: Commands, asset_server: Res<AssetServer>) {
    // selector entity for placing buildings
    commands.spawn((
        Sprite {
            image: asset_server.load("pixels/white.png"),
            color: Color::srgba(1.0, 0.75, 0.8, 0.0), // pink, hidden by default
            ..default()
        },
        Transform::default()
            .with_translation(Vec3::new(0., 0., 100.))
            .with_scale(Vec2::new(2.0 * TILE_SIZE, 2.0 * TILE_SIZE).extend(1.0)),
        GlobalBuildingPlacementTiles::new(),
    ));
}
