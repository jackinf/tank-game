use crate::common::components::unit_id::UnitId;
use crate::tank::components::tank::Tank;
use crate::tank::components::tank_bullet::TankBullet;
use crate::tank::managers::tank_spawn_manager::TankSpawnManager;
use bevy::asset::AssetServer;
use bevy::prelude::{
    Commands, Entity, Query, Res, Time, Transform, Vec2, Vec3, Vec3Swizzles, With,
};
use std::collections::HashMap;

pub struct TankShootingManager;

impl TankShootingManager {
    pub fn periodic_shooting(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut q_tanks: Query<(&mut Tank, &Transform), With<Tank>>,
        time: Res<Time>,
    ) {
        let tank_ids_to_positions: HashMap<UnitId, Vec2> = q_tanks
            .iter()
            .map(|(tank, transform)| (tank.id.clone(), transform.translation.xy()))
            .collect();

        let tanks_that_can_shoot: HashMap<UnitId, UnitId> = q_tanks
            .iter_mut()
            .filter(|(tank, _)| {
                tank.has_target() && !tank.is_cooling_down(time.elapsed_seconds_f64())
            })
            .map(|(mut tank, transform)| {
                tank.start_cooling_down(time.elapsed_seconds_f64());

                return (tank.id.clone(), tank.get_target().unwrap().clone());
            })
            .collect();

        tanks_that_can_shoot
            .into_iter()
            .for_each(|(source_id, target_id)| {
                let source = tank_ids_to_positions.get(&source_id).unwrap();
                let target = tank_ids_to_positions.get(&target_id).unwrap();
                TankSpawnManager::spawn_tank_bullet(
                    &mut commands,
                    &asset_server,
                    source.clone(),
                    target.clone(),
                );
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
