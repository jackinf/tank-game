use crate::features::building::types::building_queue::BuildingQueue;
use bevy::prelude::{Query, Res, Time};

pub fn sys_update_building_construction(
    time: Res<Time>,
    mut q_building_queues: Query<&mut BuildingQueue>,
) {
    let delta = time.delta_secs();

    q_building_queues.iter_mut().for_each(|mut building_queue| {
        building_queue.update(delta);
    });
}
