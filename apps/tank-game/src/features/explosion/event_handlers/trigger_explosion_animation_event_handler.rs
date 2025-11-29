use crate::features::explosion::components::{AnimationActive, SmallExplosion};
use crate::features::explosion::TriggerExplosionAnimationEvent;
use bevy::prelude::*;

pub fn trigger_explosion_animation_event_handler(
    mut trigger_explosion_animation_event_reader: EventReader<TriggerExplosionAnimationEvent>,
    mut query: Query<
        (&mut Transform, &mut Sprite, &mut AnimationActive),
        With<SmallExplosion>,
    >,
) {
    for event in trigger_explosion_animation_event_reader.read() {
        let at = event.at();
        let scale = event.scale();

        for (mut transform, mut sprite, mut active) in &mut query {
            transform.translation = at.extend(transform.translation.z);
            transform.scale = Vec3::splat(scale);
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = 0;
            }
            active.0 = true;
        }
    }
}
