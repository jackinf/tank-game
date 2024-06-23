//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::prelude::*;
mod features;

use crate::features::animation::{
    explosion_animation_setup, play_explosion, trigger_animation_on_key,
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest())))
        .add_systems(
            PreStartup,
            (camera_setup, explosion_animation_setup).chain(),
        )
        .add_systems(Update, (trigger_animation_on_key, play_explosion))
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
