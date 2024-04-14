use crate::harvester::managers::harvester_manager::HarvesterManager;
use crate::harvester::resources::harvester_timer::HarvesterTimer;
use bevy::app::App;
use bevy::prelude::{Plugin, Timer, TimerMode, Update};

pub struct HarvesterPlugin;

impl Plugin for HarvesterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HarvesterTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(Update, HarvesterManager::run_state_machine_step);
    }
}
