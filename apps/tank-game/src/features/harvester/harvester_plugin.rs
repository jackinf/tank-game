use crate::features::harvester::resources::HarvesterTimer;
use crate::features::harvester::systems::{
    sys_begin, sys_collect_gold, sys_find_base_to_return, sys_find_gold_for_hungry_harvester,
    sys_move_harvester,
};
use crate::AppState;
use bevy::app::App;
use bevy::prelude::*;

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
                sys_begin,
                sys_find_gold_for_hungry_harvester,
                sys_collect_gold,
                sys_find_base_to_return,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            FixedUpdate,
            sys_move_harvester.run_if(in_state(AppState::Playing)),
        );
    }
}
