use crate::actions::calculate_world_position::calculate_world_position;
use crate::constants::TileCoord;
use crate::features::harvester::spawn_harvester;
use crate::features::tank::spawn_tank;
use crate::features::unit::types::UnitTileType;
use crate::features::unit::{UnitIdCounter, UnitsLayer};
use bevy::prelude::{AssetServer, Commands, Res, ResMut, Vec2};

pub fn spawn_units(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut unit_id_counter: &mut ResMut<UnitIdCounter>,
    layer: UnitsLayer,
) {
    layer.get_units().into_iter().for_each(|(coord, unit)| {
        let pos = calculate_world_position(coord);

        match unit.get_unit_type() {
            UnitTileType::Tank => {
                spawn_tank(
                    &mut commands,
                    &asset_server,
                    pos,
                    &mut unit_id_counter,
                    unit.get_player(),
                );
            }
            UnitTileType::Harvester => {
                spawn_harvester(
                    &mut commands,
                    &asset_server,
                    unit.get_image_path(),
                    pos,
                    &mut unit_id_counter,
                    unit.get_player(),
                );
            }
            _ => {}
        }
    });
}
