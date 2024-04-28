use crate::building::components::building_placement_tiles::BuildingPlacementTiles;
use crate::building::managers::building_interaction_manager::BuildingInteractionManager;
use crate::building::managers::building_spawn_manager::BuildingSpawnManager;
use crate::common::constants::TILE_SIZE;
use bevy::app::{App, FixedUpdate, PreStartup};
use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, Color, Commands, Plugin, Res, Sprite, SpriteBundle, Transform, Update,
};

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(FixedUpdate, BuildingSpawnManager::draw_construction_tiles)
            .add_systems(Update, BuildingInteractionManager::interact);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // selector entity for placing buildings
    commands
        .spawn((SpriteBundle {
            texture: asset_server.load("pixels/white.png"),
            transform: Transform::default()
                .with_translation(Vec3::new(0., 0., 100.))
                .with_scale(Vec2::new(2.0 * TILE_SIZE, 2.0 * TILE_SIZE).extend(1.0)),
            sprite: Sprite {
                color: Color::PINK.with_a(0.0), // hide by default
                ..default()
            },
            ..default()
        },))
        .insert(BuildingPlacementTiles::new());
}
