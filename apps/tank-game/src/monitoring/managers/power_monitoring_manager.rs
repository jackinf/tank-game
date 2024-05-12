use crate::building::components::building::Building;
use crate::common::resources::me::Me;
use crate::monitoring::monitoring_plugin::PowerTimer;
use bevy::prelude::{Query, Res, ResMut, Time, Timer};

pub struct PowerMonitoringManager;

impl PowerMonitoringManager {
    pub fn monitor_power(
        q_buildings: Query<&Building>,
        mut me: ResMut<Me>,
        mut timer: ResMut<PowerTimer>,
        time: Res<Time>,
    ) {
        if !timer.0.tick(time.delta()).just_finished() {
            return;
        }

        let total_power: i32 = q_buildings
            .iter()
            .map(|building| building.get_building_tile().get_power_level())
            .sum::<i32>()
            .into();

        me.set_energy(total_power);
    }
}
