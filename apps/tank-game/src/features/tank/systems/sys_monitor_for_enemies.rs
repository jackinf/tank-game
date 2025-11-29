use crate::features::building::components::Building;
use crate::features::tank::components::Tank;
use crate::features::tank::resources::TankMonitoringTimer;
use crate::features::unit::UnitId;
use crate::types::player::Player;
use bevy::prelude::{Query, Res, ResMut, Time, Transform, Vec2, Vec3Swizzles, With};
use std::collections::HashMap;

type UnitInfo = (UnitId, Vec2, f32, Option<Player>, bool);

pub fn sys_monitor_for_enemies(
    mut q_tanks: Query<(&mut Tank, &Transform), With<Tank>>,
    mut q_buildings: Query<(&mut Building, &Transform), With<Building>>,
    mut tank_monitoring_timer: ResMut<TankMonitoringTimer>,
    time: Res<Time>,
) {
    // finding pairs is O(N^2), so, use timer to do it less frequently
    if !tank_monitoring_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let tank_id_pos_map: Vec<UnitInfo> = q_tanks
        .iter()
        .map(|(tank, transform)| {
            (
                tank.id.clone(),
                transform.translation.xy(),
                tank.get_radius(),
                tank.get_player(),
                tank.is_moving(),
            )
        })
        .collect();

    let building_id_pos_map: Vec<UnitInfo> = q_buildings
        .iter()
        .map(|(building, transform)| {
            (
                building.id().clone(),
                transform.translation.xy(),
                building.radius(),
                building.get_player(),
                false, // buildings don't move
            )
        })
        .collect();

    let mut all_ids_pos_map: Vec<UnitInfo> = Vec::new();
    all_ids_pos_map.extend(tank_id_pos_map.clone().into_iter());
    all_ids_pos_map.extend(building_id_pos_map.into_iter());

    // find pairs of tanks that are close enough to shoot each other
    let mut targets = HashMap::new();
    for (u1_id, source, u1_radius, u1_player, u1_moving) in tank_id_pos_map.into_iter() {
        if u1_moving {
            // don't attack while moving
            continue;
        }

        for (u2_id, target, u2_radius, u2_player, _) in all_ids_pos_map.iter() {
            // don't attack friendly tanks and self
            if u1_player == *u2_player {
                continue;
            }

            if source.distance(*target) < u1_radius {
                targets.insert(u1_id.clone(), u2_id.clone());
            }

            if target.distance(source) < *u2_radius {
                targets.insert(u2_id.clone(), u1_id.clone());
            }
        }
    }

    q_tanks
        .iter_mut()
        .filter(|(tank, _)| !tank.has_target())
        .for_each(|(mut tank, _)| {
            if let Some(target_id) = targets.get(&tank.id) {
                tank.set_target(Some(target_id.clone()));
            }
        });
}
