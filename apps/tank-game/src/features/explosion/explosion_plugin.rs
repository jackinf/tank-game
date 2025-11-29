use crate::features::explosion::event_handlers::trigger_explosion_animation_event_handler;
use crate::features::explosion::{
    play_explosion, prepare_explosion_animation, TriggerExplosionAnimationEvent,
};
use bevy::app::App;
use bevy::prelude::{Plugin, Startup, Update};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TriggerExplosionAnimationEvent>()
            .add_systems(Startup, prepare_explosion_animation)
            .add_systems(
                Update,
                (play_explosion, trigger_explosion_animation_event_handler),
            );
    }
}
