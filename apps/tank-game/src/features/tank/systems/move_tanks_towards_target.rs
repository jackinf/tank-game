use crate::constants::TANK_ROTATION_SPEED;
use crate::features::tank::components::{Tank, TankGun};
use crate::features::unit::UnitId;
use bevy::prelude::{Quat, Query, Res, Time, Transform, Vec2, Vec3Swizzles, With, Without};
use std::collections::HashMap;

pub fn move_tanks_towards_target(
    time: Res<Time>,
    mut tank_query: Query<(&mut Transform, &mut Tank), (With<Tank>, Without<TankGun>)>,
    mut gun_query: Query<(&mut Transform, &TankGun), (With<TankGun>, Without<Tank>)>,
) {
    let dt = time.delta_seconds();

    let tank_id_and_positions: HashMap<UnitId, Vec2> = tank_query
        .iter()
        .map(|(transform, tank)| (tank.id.clone(), transform.translation.xy()))
        .collect();

    // move all tanks via path
    for (mut transform, mut tank) in tank_query.iter_mut().filter(|(_, tank)| tank.is_moving()) {
        let current_pos = transform.translation.xy();

        if tank.get_stop_when_target_in_range() {
            // println!("STOP WHEN TARGET IN RANGE");

            // if tank has target, check if it's close enough to stop
            if let Some(target) = tank
                .get_target()
                .and_then(|target_id| tank_id_and_positions.get(&target_id))
            {
                let vector = *target - current_pos;
                let total_distance = vector.length();
                if total_distance < tank.get_radius() {
                    tank.stop();
                    continue;
                }
            }
        }

        let direction = tank.target_position - current_pos;
        let distance_to_move = tank.speed * dt;

        // Smooth movement
        if direction.length() > distance_to_move {
            let new_pos = current_pos + direction.normalize() * distance_to_move;
            transform.translation = new_pos.extend(transform.translation.z);
        } else {
            transform.translation = tank.target_position.extend(transform.translation.z);
            tank.try_take_next_position_in_path();
        }
    }

    // Rotate tank gun smoothly for all tanks
    for (transform, tank) in tank_query.iter().filter(|(_, tank)| tank.is_moving()) {
        if let Some((mut gun_transform, _)) = gun_query
            .iter_mut()
            .find(|(_, gun)| gun.parent_id.0 == tank.id.0)
        {
            let direction = tank.target_position - transform.translation.xy();
            let target_angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
            let quat = Quat::from_rotation_z(target_angle);

            gun_transform.rotation = gun_transform.rotation.slerp(quat, TANK_ROTATION_SPEED * dt);
        }
    }
}
