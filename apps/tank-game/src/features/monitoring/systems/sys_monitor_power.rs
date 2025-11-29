use crate::features::building::components::Building;
use crate::features::con_menu::MenuInfo;
use crate::features::monitoring::resources::PowerTimer;
use bevy::prelude::{Query, Res, ResMut, Time};

pub fn sys_monitor_power(
    q_buildings: Query<&Building>,
    mut timer: ResMut<PowerTimer>,
    time: Res<Time>,
    mut q_menu_info: Query<&mut MenuInfo>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let total_power: i32 = q_buildings
        .iter()
        .map(|building| building.get_building_tile().get_power_level())
        .sum::<i32>()
        .into();

    let mut me = q_menu_info.single_mut().unwrap();
    me.set_energy(total_power);
}
