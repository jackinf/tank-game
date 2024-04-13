use crate::common::components::unit_id::UnitId;
use crate::common::constants::Player;
use crate::tank::components::tank::Tank;
use crate::tank::components::tank_bullet::TankBullet;
use crate::tank::managers::tank_spawn_manager::TankSpawnManager;
use crate::tank::resources::tank_monitoring_timer::TankMonitoringTimer;
use bevy::asset::AssetServer;
use bevy::prelude::{
    Commands, Entity, Query, Res, ResMut, Time, Transform, Vec2, Vec3, Vec3Swizzles, With,
};
use std::collections::HashMap;

pub struct TankShootingManager;

impl TankShootingManager {
    pub fn monitor_for_enemies(
        mut q_tanks: Query<(&mut Tank, &Transform), With<Tank>>,
        mut timer: ResMut<TankMonitoringTimer>,
        time: Res<Time>,
    ) {
        // finding pairs is O(N^2), so, use timer to do it less frequently
        if !timer.0.tick(time.delta()).just_finished() {
            return;
        }

        let id_pos: Vec<(UnitId, Vec2, f32, Player)> = q_tanks
            .iter()
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
            } else {
                tank.set_target(None);
            }
        });
    }

    pub fn periodic_shooting(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut q_tanks: Query<(&mut Tank, &Transform), With<Tank>>,
        time: Res<Time>,
    ) {
        let id_pos: HashMap<UnitId, Vec2> = q_tanks
            .iter()
            .map(|(tank, transform)| (tank.id.clone(), transform.translation.xy()))
            .collect();

        q_tanks
            .iter_mut()
            .filter(|(tank, _)| {
                tank.has_target() && !tank.is_cooling_down(time.elapsed_seconds_f64())
            })
            .for_each(|(mut tank, transform)| {
                let source_option = id_pos.get(&tank.id);
                let target_option = id_pos.get(&tank.get_target().unwrap());
                if let (Some(source), Some(target)) = (source_option, target_option) {
                    if source.distance(*target) > tank.get_radius() {
                        return;
                    }

                    tank.start_cooling_down(time.elapsed_seconds_f64());
                    TankSpawnManager::spawn_tank_bullet(
                        &mut commands,
                        &asset_server,
                        source.clone(),
                        target.clone(),
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
