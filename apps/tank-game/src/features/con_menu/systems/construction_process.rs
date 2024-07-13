use crate::actions::calculate_tile_world_position::calculate_tile_to_world_position;
use crate::features::building::components::Building;
use crate::features::building::types::BuildingTileType;
use crate::features::con_menu::components::UnitConstructionProgressInfo;
use crate::features::con_menu::resources::BuildingConstructionProgressInfo;
use crate::features::con_menu::MenuInfo;
use crate::features::tank::events::{SpawnHarvesterEvent, SpawnTankEvent};
use crate::features::tank::TankStrategy;
use crate::features::unit::UnitTileType;
use bevy::prelude::{EventWriter, Query, Res, Time, Vec2};
use rand::prelude::SliceRandom;

pub fn construction_process(
    mut time: Res<Time>,
    mut q_building_construction_progress_info: Query<&mut BuildingConstructionProgressInfo>,
    mut q_unit_construction_progress_info: Query<&mut UnitConstructionProgressInfo>,
    mut q_menu_info: Query<&mut MenuInfo>,
    q_buildings: Query<&Building>,
    mut spawn_tank_event_writer: EventWriter<SpawnTankEvent>,
    mut spawn_harvester_event_writer: EventWriter<SpawnHarvesterEvent>,
) {
    let mut me = q_menu_info.single_mut();
    q_building_construction_progress_info
        .iter_mut()
        .for_each(|mut info| {
            if info.is_idle() {
                return;
            }

            if info.is_constructing() && me.has_enough_money(info.get_price_per_tick()) {
                if info.tick(time.delta()) {
                    me.subtract_money(info.get_price_per_tick());
                }

                if info.is_placing() {
                    println!("Building is ready!");

                    // todo: play voice that the building is ready
                }
                return;
            }

            // todo: update sprite to increase square vertically by % ready
        });

    let factory_world_locations = q_buildings
        .iter()
        .filter(|building| {
            building.get_building_tile_type() == BuildingTileType::Factory
                && building.get_player() == Some(me.player())
        })
        .map(|building| calculate_tile_to_world_position(&building.get_door()))
        .collect::<Vec<Vec2>>();

    q_unit_construction_progress_info
        .iter_mut()
        .for_each(|mut info| {
            if info.is_idle() {
                return;
            }

            if info.is_constructing() && me.has_enough_money(info.get_price_per_tick()) {
                if info.tick(time.delta()) {
                    me.subtract_money(info.get_price_per_tick());
                }
                // todo: update sprite to increase square vertically by % ready
            }
        });

    q_unit_construction_progress_info
        .iter_mut()
        .for_each(|mut info| {
            if info.is_placing() {
                info.reset();
                // randomly choose one of the factories
                let factory_pos = factory_world_locations.choose(&mut rand::thread_rng());
                if factory_pos.is_none() {
                    // no factories found
                    return;
                }
                let factory_pos = factory_pos.unwrap();

                if let Some(unit_tile) = info.get_unit_tile() {
                    if unit_tile.get_unit_type() == UnitTileType::Tank {
                        spawn_tank_event_writer.send(SpawnTankEvent {
                            position: factory_pos.clone(),
                            player: me.player().clone(),
                            strategy: TankStrategy::Idle,
                        });
                    } else if unit_tile.get_unit_type() == UnitTileType::Harvester {
                        spawn_harvester_event_writer.send(SpawnHarvesterEvent {
                            position: factory_pos.clone(),
                            player: me.player().clone(),
                        });
                    }
                }

                println!("Unit is ready!");
                // todo: play voice that the building is ready
            }
        });
}
