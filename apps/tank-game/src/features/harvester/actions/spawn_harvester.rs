use crate::constants::SPRITE_SCALE;
use crate::features::harvester::components::Harvester;
use crate::features::unit::UnitIdCounter;
use crate::types::player::Player;
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Res, ResMut, Sprite, SpriteBundle, Transform, Vec3};

pub fn spawn_harvester(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: Vec2,
    unit_id_counter: &mut ResMut<UnitIdCounter>,
    player: Option<Player>,
) {
    let color = match player {
        Some(Player::P1) => crate::constants::P1_COLOR,
        Some(Player::P2) => crate::constants::P2_COLOR,
        _ => crate::constants::NEUTRAL_COLOR,
    };

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
            sprite: Sprite { color, ..default() },
            ..default()
        },))
        .insert(harvester);
}
