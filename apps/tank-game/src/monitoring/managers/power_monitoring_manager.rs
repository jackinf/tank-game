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

        let power_plants_count = q_buildings
            .iter()
            .filter(|building| building.is_power_plant())
            .count();

        let total_energy = power_plants_count * 10;
        me.set_energy(total_energy as u32);
    }
}
