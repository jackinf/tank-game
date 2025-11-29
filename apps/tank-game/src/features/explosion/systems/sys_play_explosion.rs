use crate::features::explosion::components::{
    AnimationActive, AnimationIndices, AnimationTimer, SmallExplosion,
};
use bevy::prelude::{Query, Res, Sprite, Time, With};

pub fn sys_play_explosion(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut Sprite,
            &mut AnimationActive,
        ),
        With<SmallExplosion>,
    >,
) {
    for (indices, mut timer, mut sprite, mut active) in &mut query {
        if active.0 {
            if timer.tick(time.delta()).just_finished() {
                if let Some(ref mut atlas) = sprite.texture_atlas {
                    if atlas.index == indices.last() {
                        atlas.index = 0;
                        active.0 = false; // Stop the animation after the last frame
                    } else {
                        atlas.index += 1;
                    }
                }
            }
        }
    }
}
