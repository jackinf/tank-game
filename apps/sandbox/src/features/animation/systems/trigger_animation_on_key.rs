use crate::features::animation::AnimationActive;
use bevy::asset::AssetServer;
use bevy::audio::AudioBundle;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::{default, Commands, EventReader, KeyCode, Query, Res, TextureAtlas};

pub fn trigger_animation_on_key(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut key_button_events: EventReader<KeyboardInput>,
    mut query: Query<(&mut TextureAtlas, &mut AnimationActive)>,
) {
    for key_button_event in key_button_events.read() {
        if key_button_event.key_code == KeyCode::KeyM {
            for (mut atlas, mut active) in &mut query {
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/explosion.ogg"),
                    ..default()
                });

                atlas.index = 0;
                active.0 = true; // Activate the animation when 'M' is pressed
            }
        }
    }
}
