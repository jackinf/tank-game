use crate::actions::calculate_tile_world_position::calculate_tile_to_world_position;
use crate::features::con_menu::MenuInfo;
use crate::features::harvester::components::Harvester;
use bevy::math::Vec2;
use bevy::prelude::{Query, Res, Time, Transform, Vec3Swizzles, With};

pub fn move_harvester(
    time: Res<Time>,
    mut q_harvesters: Query<(&mut Harvester, &mut Transform), With<Harvester>>,
    mut q_menu_info: Query<&mut MenuInfo>,
) {
    let dt = time.delta_seconds();
    let mut menu_info = q_menu_info.single_mut();

    q_harvesters
        .iter_mut()
        .filter(|(harvester, _)| harvester.has_movement_path())
        // .filter(|(harvester, _)| harvester.is_moving_to_gold() || harvester.is_returning_to_base() || harvester.is_forced_by_player())
        .for_each(|(mut harvester, mut transform)| {
            let next_tile = harvester.get_movement_path().into_iter().next().unwrap();
            let last_world_pos = calculate_tile_to_world_position(&next_tile);

            let current_pos = transform.translation.xy();
            let direction_vector = last_world_pos - current_pos;
            let direction = direction_vector.normalize();
            let distance_to_move = harvester.get_speed() * dt;

            let is_close_enough = direction_vector.length() < distance_to_move;
            if is_close_enough {
                // Made it to the temporary destination, now pick the next one, or start mining
                transform.translation = last_world_pos.extend(transform.translation.z);
                harvester.try_take_next_position_in_path();
                if !harvester.has_movement_path() {
                    // Made it. Decide, what to do now based on the previous action.

                    if harvester.is_moving_to_gold() {
                        harvester.set_harvesting();
                    } else if harvester.is_returning_to_base() {
                        let unloaded_gold = harvester.unload_gold();
                        menu_info.add_money(unloaded_gold);
                        harvester.set_idle();
                    } else if harvester.is_forced_by_player() {
                        harvester.set_idle();
                    } else {
                        harvester.set_idle();
                    }
                }
            } else {
                // Continue movement
                let new_pos = current_pos + direction * distance_to_move;
                transform.translation = new_pos.extend(transform.translation.z);
            }
        });
}
