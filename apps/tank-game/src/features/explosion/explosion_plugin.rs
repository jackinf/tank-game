use crate::features::explosion::event_handlers::trigger_explosion_animation_event_handler;
use crate::features::explosion::{
    sys_play_explosion, sys_prepare_explosion_animation, TriggerExplosionAnimationEvent,
};
use bevy::app::App;
use bevy::prelude::{Plugin, Startup, Update};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TriggerExplosionAnimationEvent>()
            .add_systems(Startup, sys_prepare_explosion_animation)
            .add_systems(
                Update,
                (sys_play_explosion, trigger_explosion_animation_event_handler),
            );
    }
}
