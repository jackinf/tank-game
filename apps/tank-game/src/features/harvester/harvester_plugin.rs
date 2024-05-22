use crate::features::harvester::resources::HarvesterTimer;
use crate::features::harvester::systems::{
    begin, collect_gold, find_base_to_return, find_gold_for_hungry_harvester, move_harvester,
};
use bevy::app::App;
use bevy::prelude::{FixedUpdate, Plugin, Timer, TimerMode, Update};

pub struct HarvesterPlugin;

impl Plugin for HarvesterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HarvesterTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(Update, begin)
        .add_systems(Update, find_gold_for_hungry_harvester)
        .add_systems(FixedUpdate, move_harvester)
        .add_systems(Update, collect_gold)
        .add_systems(Update, find_base_to_return);
    }
}
