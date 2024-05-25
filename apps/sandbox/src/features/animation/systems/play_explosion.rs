use crate::features::animation::{AnimationActive, AnimationIndices, AnimationTimer};
use bevy::prelude::{Query, Res, ResMut, TextureAtlas, Time};

pub fn play_explosion(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &mut AnimationActive,
    )>,
) {
    for (indices, mut timer, mut atlas, mut active) in &mut query {
        if active.0 {
            if timer.tick(time.delta()).just_finished() {
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
