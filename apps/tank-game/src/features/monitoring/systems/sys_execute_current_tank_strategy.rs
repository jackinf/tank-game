use crate::features::building::components::Building;
use crate::features::monitoring::resources::StrategyMonitoringTimer;
use crate::features::tank::events::SetPathToTargetEvent;
use crate::features::tank::{Tank, TankStrategy};
use crate::features::unit::UnitId;
use crate::types::player::Player;
use bevy::prelude::*;
use rand::seq::IteratorRandom;
use std::collections::HashSet;

/// This system is responsible for executing the current strategy of each enemy tank.
pub fn sys_execute_current_tank_strategy(
    time: Res<Time>,
    mut timer: ResMut<StrategyMonitoringTimer>,
    mut q_tanks: Query<&mut Tank>,
    q_buildings: Query<&Building>,
    mut set_path_to_target_event_handler: MessageWriter<SetPathToTargetEvent>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    // i want components Tank for P2 with agressive TankStrategy to search for P1 units or buildings. Find first, and then calculate path to it. Ignore tanks that are not agressive or have target
    let p1_units = get_all_p1_units(&mut q_tanks, &q_buildings);

    q_tanks
        .iter_mut()
        .filter(|(tank)| tank.get_player() == Some(Player::P2))
        .for_each(|(mut tank)| {
            if tank.get_current_strategy() == TankStrategy::Aggressive {
                // is current target still alive?
                if let Some(target) = tank.get_target() {
                    if !p1_units.contains(&target) {
                        tank.set_target(None);
                    }
                }

                if !tank.has_target() {
                    // randomly select a target
                    let mut rng = rand::rng();
                    if let Some(target) = p1_units.iter().choose(&mut rng) {
                        set_path_to_target_event_handler.write(SetPathToTargetEvent {
                            source: tank.get_id(),
                            target: target.clone(),
                        });
                        // tank.set_target(Some(target.clone()));
                    }
                }
            }
        });
}

fn get_all_p1_units(
    q_tanks: &mut Query<&mut Tank>,
    q_buildings: &Query<(&Building)>,
) -> HashSet<UnitId> {
    let p1_tank_ids: HashSet<UnitId> = q_tanks
        .iter()
        .filter(|(tank)| tank.get_player() == Some(Player::P1))
        .map(|tank| tank.get_id())
        .collect::<HashSet<UnitId>>();
    let p1_building_ids: HashSet<UnitId> = q_buildings
        .iter()
        .filter(|building| building.get_player() == Some(Player::P1))
        .map(|building| building.id())
        .collect::<HashSet<UnitId>>();

    p1_tank_ids
        .iter()
        .chain(p1_building_ids.iter())
        .cloned()
        .collect::<HashSet<UnitId>>()
}
