use crate::tank::components::tank_bullet::TankBullet;
use bevy::prelude::{Commands, Entity, Query, Transform, With};

pub struct TankShootingManager;

impl TankShootingManager {
    pub fn move_bullets(
        mut commands: Commands,
        mut q_bullets: Query<(Entity, &mut Transform, &TankBullet), With<TankBullet>>,
    ) {
        q_bullets
            .iter_mut()
            .for_each(|(entity, mut transform, bullet)| {
                let destination = bullet.get_destination();
                let speed = bullet.get_speed();
                let direction = destination - transform.translation.truncate();
                let distance = direction.length();
                let velocity = direction.normalize() * speed;
                let new_position = direction + velocity * 0.02;
                transform.translation = new_position.extend(transform.translation.z);
                if distance < 10.0 {
                    commands.entity(entity).despawn();
                    // TODO: damage the units nearby
                }
            });
    }
}
