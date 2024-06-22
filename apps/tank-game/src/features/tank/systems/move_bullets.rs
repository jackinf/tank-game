use crate::constants::{BULLET_RADIUS, TANK_MAX_HEALTH};
use crate::features::explosion::TriggerExplosionAnimationEvent;
use crate::features::tank::components::{Tank, TankBullet};
use bevy::prelude::{
    Commands, Entity, EventWriter, Query, Res, Time, Transform, Vec2, Vec3, Vec3Swizzles, With,
    Without,
};
use crate::features::building::components::Building;

pub fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut q_bullets: Query<(Entity, &mut Transform, &TankBullet), (With<TankBullet>, Without<Tank>, Without<Building>)>,
    mut q_tanks: Query<(&mut Tank, &Transform), (With<Tank>, Without<TankBullet>, Without<Building>)>,
    mut q_buildings: Query<(&mut Building, &Transform), (With<Building>, Without<TankBullet>, Without<Tank>)>,
    mut trigger_explosion_animation_event_writer: EventWriter<TriggerExplosionAnimationEvent>,
) {
    let dt = time.delta_seconds(); // Get the delta time for frame-rate independent movement

    let bullets_exploded_at = q_bullets
        .iter_mut()
        .map(|(entity, mut transform, bullet)| {
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
                let target = transform.translation.xy();
                trigger_explosion_animation_event_writer
                    .send(TriggerExplosionAnimationEvent::new(target.clone()));
                Some((target, bullet.get_damage()))
            } else {
                None
            }
        })
        .filter(|coord| coord.is_some())
        .map(|coord| coord.unwrap())
        .collect::<Vec<(Vec2, u32)>>();

    for (mut tank, transform) in q_tanks.iter_mut() {
        for (bullet_position, bullet_damage) in bullets_exploded_at.iter() {
            if transform.translation.xy().distance(*bullet_position) < BULLET_RADIUS {
                tank.take_damage(*bullet_damage);
            }
        }
    }

    for (mut building, transform) in q_buildings.iter_mut() {
        for (bullet_position, bullet_damage) in bullets_exploded_at.iter() {
            if transform.translation.xy().distance(*bullet_position) < BULLET_RADIUS {
                building.damage(*bullet_damage);
                let health_percentage = building.get_health() as f32 / building.get_max_health() as f32;
                dbg!(health_percentage);
            }
        }
    }
}
