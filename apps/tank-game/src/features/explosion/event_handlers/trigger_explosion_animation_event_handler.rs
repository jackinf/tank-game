use crate::features::explosion::components::{AnimationActive, SmallExplosion};
use crate::features::explosion::TriggerExplosionAnimationEvent;
use bevy::prelude::{
    default, AssetServer, AudioBundle, Commands, EventReader, Query, Res, TextureAtlas, Transform,
    Vec3, With,
};

pub fn trigger_explosion_animation_event_handler(
    mut trigger_explosion_animation_event_reader: EventReader<TriggerExplosionAnimationEvent>,
    mut query: Query<
        (&mut Transform, &mut TextureAtlas, &mut AnimationActive),
        With<SmallExplosion>,
    >,
) {
    for event in trigger_explosion_animation_event_reader.read() {
        let at = event.at();
        let scale = event.scale();

        for (mut transform, mut atlas, mut active) in &mut query {
            transform.translation = at.extend(transform.translation.z);
            transform.scale = Vec3::splat(scale);
            atlas.index = 0;
            active.0 = true;
        }
    }
}
