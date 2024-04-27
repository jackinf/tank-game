use crate::harvester::managers::harvester_state_manager::HarvesterStateManager;
use crate::harvester::resources::harvester_timer::HarvesterTimer;
use bevy::app::App;
use bevy::prelude::{FixedUpdate, Plugin, Timer, TimerMode, Update};

pub struct HarvesterPlugin;

impl Plugin for HarvesterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HarvesterTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(Update, HarvesterStateManager::begin)
        .add_systems(
            Update,
            HarvesterStateManager::find_gold_for_hungry_harvester,
        )
        .add_systems(
            FixedUpdate,
            HarvesterStateManager::move_harvester_towards_path,
        )
        .add_systems(
            Update,
            HarvesterStateManager::collect_gold,
        )
        .add_systems(
            Update,
            HarvesterStateManager::find_base_to_return,
        )
        .add_systems(
            FixedUpdate,
            HarvesterStateManager::return_to_base,
        );
    }
}
