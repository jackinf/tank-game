use crate::features::harvester::components::Harvester;
use crate::features::tile::{find_accessible_tile_coord, Tile};
use crate::resources::game_map::GameMap;
use crate::resources::gold_map::GoldMap;
use crate::systems::find_first_gold;
use crate::utils::astar::find_path;
use bevy::prelude::{Query, Res, Transform, Vec3Swizzles, With};

pub fn find_gold_for_hungry_harvester(
    mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
    tile_query: Query<&Tile>,
    gold_map: Res<GoldMap>,
    game_map: Res<GameMap>,
) {
    q_harvesters
        .iter_mut()
        .filter(|(harvester, _)| harvester.is_searching_for_gold())
        .for_each(|(mut harvester, transform)| {
            let start =
                find_accessible_tile_coord(&tile_query, &transform.translation.xy()).unwrap();
            let found_option = find_first_gold(&gold_map.get_tile_type_grid_i32(), start);

            if let Some(goal) = found_option {
                let path = find_path(&game_map.get_tile_type_grid_usize(), start, goal);
                // dbg!(goal);
                harvester.set_movement_path(path);
                harvester.set_moving_to_gold();
            }
        });
}
