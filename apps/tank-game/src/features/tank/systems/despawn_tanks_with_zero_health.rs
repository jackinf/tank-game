use crate::features::tank::components::Tank;
use bevy::prelude::{Commands, Entity, Query};

pub fn despawn_tanks_with_zero_health(mut commands: Commands, query: Query<(Entity, &Tank)>) {
    for (entity, tank) in query.iter() {
        if tank.is_dead() {
            // Despawn the tank entity
            commands.entity(entity).despawn();
        }
    }
}
