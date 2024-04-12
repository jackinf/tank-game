use crate::common::components::unit_id::UnitId;
use crate::common::utils::common_helpers::CommonHelpers;
use crate::tank::components::tank::Tank;
use crate::tank::components::tank_bullet::TankBullet;
use crate::tank::managers::tank_spawn_manager::TankSpawnManager;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Entity, Query, Res, Time, Transform, Vec2, Vec3, With};

pub struct TankShootingManager;

impl TankShootingManager {
    pub fn periodic_shooting(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut q_tanks1: Query<(&mut Tank, &Transform), With<Tank>>,
        time: Res<Time>,
    ) {
        let target_ids: Vec<(UnitId, Vec2)> = q_tanks1
            .iter_mut()
            .filter(|(tank, _)| {
                tank.has_target() && !tank.is_cooling_down(time.elapsed_seconds_f64())
            })
            .map(|(mut tank, transform)| {
                tank.start_cooling_down(time.elapsed_seconds_f64());
                let x = transform.translation.x;
                let y = transform.translation.y;

                return (tank.get_target().unwrap(), Vec2::new(x, y));
            })
            .collect::<Vec<(UnitId, Vec2)>>();

        target_ids.into_iter().for_each(|(target_id, from)| {
            let enemy = q_tanks1
                .iter()
                .find(|(enemy, _)| enemy.get_id() == target_id);
            if let Some((_, enemy_transform)) = enemy {
                TankSpawnManager::spawn_tank_bullet(
                    &mut commands,
                    &asset_server,
                    from,
                    enemy_transform.translation.truncate(),
                );
            }
        });
    }

    pub fn move_bullets(
        mut commands: Commands,
        time: Res<Time>,
        mut q_bullets: Query<(Entity, &mut Transform, &TankBullet), With<TankBullet>>,
    ) {
        let dt = time.delta_seconds(); // Get the delta time for frame-rate independent movement

        q_bullets
            .iter_mut()
            .for_each(|(entity, mut transform, bullet)| {
                let destination = bullet.get_destination();
                let speed = bullet.get_speed();
                let vector = destination - transform.translation.truncate();
                let direction = vector.normalize();
                let distance = vector.length();
                let velocity = direction.normalize() * speed;
                transform.translation = Vec3::new(
                    transform.translation.x + velocity.x * dt,
                    transform.translation.y + velocity.y * dt,
                    transform.translation.z,
                );
                if distance.abs() < 10.0 {
                    commands.entity(entity).despawn();
                }
            });
    }
}
