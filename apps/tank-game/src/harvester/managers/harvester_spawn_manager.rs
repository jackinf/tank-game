use crate::common::constants::SPRITE_SCALE;
use crate::common::player::Player;
use crate::harvester::components::harvester::Harvester;
use crate::unit::resources::unit_id_counter::UnitIdCounter;
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Res, ResMut, SpriteBundle, Transform, Vec3};

pub struct HarvesterSpawnManager;

impl HarvesterSpawnManager {
    pub fn spawn_harvester(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        pos: Vec2,
        unit_id_counter: &mut ResMut<UnitIdCounter>,
        player: Player,
    ) {
        let unit_id = unit_id_counter.0;
        unit_id_counter.0 += 1;

        let texture = asset_server.load("sprites/harvester.png");
        let layer = 10.0;

        let harvester = Harvester::new(player, unit_id);

        commands
            .spawn((SpriteBundle {
                transform: Transform::default()
                    .with_translation(pos.extend(layer))
                    .with_scale(Vec3::splat(SPRITE_SCALE)),
                texture,
                ..default()
            },))
            .insert(harvester);
    }
}
