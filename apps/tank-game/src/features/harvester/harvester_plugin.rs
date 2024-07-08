use crate::features::harvester::resources::HarvesterTimer;
use crate::features::harvester::systems::{
    begin, collect_gold, find_base_to_return, find_gold_for_hungry_harvester, move_harvester,
};
use crate::AppState;
use bevy::app::App;
use bevy::prelude::{in_state, FixedUpdate, IntoSystemConfigs, Plugin, Timer, TimerMode, Update};

pub struct HarvesterPlugin;

impl Plugin for HarvesterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HarvesterTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (
                begin,
                find_gold_for_hungry_harvester,
                collect_gold,
                find_base_to_return,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            FixedUpdate,
            move_harvester.run_if(in_state(AppState::Playing)),
        );
    }
}
