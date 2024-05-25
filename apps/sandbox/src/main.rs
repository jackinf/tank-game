//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

mod features;

use crate::features::animation::{
    play_explosion, prepare_explosion_animation, trigger_animation_on_key,
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, prepare_explosion_animation)
        .add_systems(Update, (trigger_animation_on_key, play_explosion))
        .run();
}
