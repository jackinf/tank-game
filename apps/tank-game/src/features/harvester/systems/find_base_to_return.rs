use crate::constants::TileCoord;
use crate::features::building::components::Building;
use crate::features::building::types::BuildingTileType;
use crate::features::harvester::components::Harvester;
use crate::features::tile::{find_accessible_tile_coord, Tile};
use crate::resources::ground_map::GroundMap;
use crate::resources::map_trait::MapTrait;
use crate::types::player::Player;
use crate::utils::astar::find_path;
use bevy::prelude::{Query, Res, Transform, Vec3Swizzles, With};

pub fn find_base_to_return(
    mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
    ground_map: Res<GroundMap>,
    q_buildings: Query<&Building>,
    q_tiles: Query<&Tile>,
) {
    let building_infos: Vec<(TileCoord, Option<Player>)> = q_buildings
        .iter()
        .filter(|building| building.get_building_tile_type() == BuildingTileType::Base)
        .map(|building| {
            let tile_coord = building.get_door();
            (tile_coord, building.get_player().clone())
        })
        .collect();

    q_harvesters
        .iter_mut()
        .filter(|(harvester, _)| harvester.is_find_base_to_return())
        .for_each(|(mut harvester, transform)| {
            let building_res = building_infos
                .iter()
                .find(|(_, player)| player == &harvester.get_player());

            if let Some((base_tile, _)) = building_res {
                let start =
                    find_accessible_tile_coord(&q_tiles, &transform.translation.xy()).unwrap();
                let path = find_path(&ground_map.get_tile_type_grid_usize(), start, *base_tile);
                harvester.set_movement_path(path);
                harvester.set_returning_to_base();
            }
        });
}
