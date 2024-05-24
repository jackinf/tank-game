use crate::features::tank::components::Tank;
use crate::features::tank::resources::TankMonitoringTimer;
use crate::features::unit::UnitId;
use crate::types::player::Player;
use bevy::prelude::{Query, Res, ResMut, Time, Transform, Vec2, Vec3Swizzles, With};
use std::collections::HashMap;

pub fn monitor_for_enemies(
    mut q_tanks: Query<(&mut Tank, &Transform), With<Tank>>,
    mut timer: ResMut<TankMonitoringTimer>,
    time: Res<Time>,
) {
    // finding pairs is O(N^2), so, use timer to do it less frequently
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let id_pos: Vec<(UnitId, Vec2, f32, Option<Player>)> = q_tanks
        .iter()
        .filter(|(tank, _)| !tank.is_moving())
        .map(|(tank, transform)| {
            (
                tank.id.clone(),
                transform.translation.xy(),
                tank.get_radius(),
                tank.get_player(),
            )
        })
        .collect();

    // find pairs of tanks that are close enough to shoot each other
    let mut targets = HashMap::new();
    for (tank1_id, source, tank1_radius, tank1_player) in id_pos.iter() {
        for (tank2_id, target, tank2_radius, tank2_player) in id_pos.iter() {
            // don't attack friendly tanks and self
            if tank1_player == tank2_player {
                continue;
            }

            if source.distance(*target) < *tank1_radius {
                targets.insert(tank1_id.clone(), tank2_id.clone());
            }

            if target.distance(*source) < *tank2_radius {
                targets.insert(tank2_id.clone(), tank1_id.clone());
            }
        }
    }

    q_tanks.iter_mut().for_each(|(mut tank, _)| {
        if let Some(target_id) = targets.get(&tank.id) {
            tank.set_target(Some(target_id.clone()));
        }
    });
}
