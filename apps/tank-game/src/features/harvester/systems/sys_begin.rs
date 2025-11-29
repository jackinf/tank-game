use crate::features::harvester::components::Harvester;
use crate::features::harvester::resources::HarvesterTimer;
use bevy::prelude::{Query, Res, ResMut, Time, With};

pub fn sys_begin(
    mut timer: ResMut<HarvesterTimer>,
    time: Res<Time>,
    mut q_harvesters: Query<&mut Harvester, With<Harvester>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    q_harvesters.iter_mut().for_each(|mut harvester| {
        if harvester.is_idle() {
            harvester.set_searching_for_gold();
        }
    });
}
