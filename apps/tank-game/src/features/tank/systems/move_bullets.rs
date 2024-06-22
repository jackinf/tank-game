use crate::actions::calculate_tile_world_position::calculate_world_to_tile_position;
use crate::components::HealthBar;
use crate::constants::BULLET_RADIUS;
use crate::features::building::components::Building;
use crate::features::explosion::TriggerExplosionAnimationEvent;
use crate::features::tank::components::{Tank, TankBullet};
use bevy::prelude::{
    Children, Commands, DespawnRecursiveExt, Entity, EventWriter, Query, Res, Sprite, Time,
    Transform, Vec2, Vec3, Vec3Swizzles, With, Without,
};
use bevy::utils::HashSet;

pub fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut q_bullets: Query<
        (Entity, &mut Transform, &TankBullet),
        (
            With<TankBullet>,
            Without<Tank>,
            Without<Building>,
            Without<HealthBar>,
        ),
    >,
    mut q_tanks: Query<
        (Entity, &mut Tank, &Transform),
        (
            With<Tank>,
            Without<TankBullet>,
            Without<Building>,
            Without<HealthBar>,
        ),
    >,
    mut q_buildings: Query<
        (Entity, &mut Building, &Children),
        (
            With<Building>,
            Without<TankBullet>,
            Without<Tank>,
            Without<HealthBar>,
        ),
    >,
    mut q_building_health_bars: Query<
        (&mut Sprite),
        (
            With<HealthBar>,
            Without<Building>,
            Without<TankBullet>,
            Without<Tank>,
        ),
    >,
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

    // Damage tanks
    let mut destroyed_tanks = Vec::new();
    for (id, mut tank, transform) in q_tanks.iter_mut() {
        for (bullet_position, bullet_damage) in bullets_exploded_at.iter() {
            if transform.translation.xy().distance(*bullet_position) < BULLET_RADIUS {
                tank.take_damage(*bullet_damage);

                if tank.is_dead() {
                    destroyed_tanks.push(id);
                }
            }
        }
    }

    for id in destroyed_tanks {
        commands.entity(id).despawn_recursive();
    }

    // Damange buildings
    let mut destroyed_buildings = Vec::new();
    for (id, mut building, children) in q_buildings.iter_mut() {
        for (bullet_position, bullet_damage) in bullets_exploded_at.iter() {
            let bullet_tile_impact = calculate_world_to_tile_position(&bullet_position);
            if building.contains(bullet_tile_impact) {
                building.damage(*bullet_damage);
                let health = building.get_health();
                let rect = building.get_building_tile().get_health_rect(health);

                for &child in children.iter() {
                    if let Ok((mut sprite)) = q_building_health_bars.get_mut(child) {
                        sprite.rect = Some(rect);
                    }
                }

                if building.is_destroyed() {
                    destroyed_buildings.push(id);
                }
            }
        }
    }

    for id in destroyed_buildings {
        commands.entity(id).despawn_recursive();
    }
}
