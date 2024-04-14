use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, AssetServer, Commands, Res, Sprite, Transform};
use bevy::sprite::{Anchor, SpriteBundle};

use crate::building::building_type::BuildingType;
use crate::building::components::building::Building;
use crate::common::constants::{Player, SPRITE_SCALE};
use crate::common::utils::enum_helpers::EnumHelpers;

pub struct BuildingSpawnManager;

impl BuildingSpawnManager {
    pub fn spawn(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        translation: Vec2,
        building_type_raw: usize,
        map_coord: (usize, usize),
        player: Player,
    ) {
        let building_type: BuildingType =
            EnumHelpers::assert_valid_enum::<BuildingType>(building_type_raw);
        let sprite_path = building_type.get_building_type_sprite();
        let layer = building_type.get_building_type_layer();
        // let tile_size = building_type.get_size();

        let color = match player {
            Player::P1 => crate::common::constants::P1_COLOR,
            Player::P2 => crate::common::constants::P2_COLOR,
        };
        let building = Building::new(building_type, map_coord, player);

        commands
            .spawn((SpriteBundle {
                transform: Transform::default()
                    .with_translation(translation.extend(layer))
                    .with_scale(Vec3::splat(SPRITE_SCALE)),
                texture: asset_server.load(sprite_path),
                sprite: Sprite {
                    color,
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },))
            .insert(building.clone());
    }
}
