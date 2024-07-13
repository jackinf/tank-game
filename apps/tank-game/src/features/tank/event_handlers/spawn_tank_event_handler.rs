use crate::features::tank::events::SpawnTankEvent;
use crate::features::tank::spawn_tank;
use crate::features::unit::UnitIdCounter;
use bevy::prelude::{AssetServer, Commands, EventReader, Res, ResMut, Vec2};

pub fn spawn_tank_event_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tank_id_counter: ResMut<UnitIdCounter>,
    mut spawn_tank_event_reader: EventReader<SpawnTankEvent>,
) {
    for spawn_tank_event in spawn_tank_event_reader.read() {
        spawn_tank(
            &mut commands,
            &asset_server,
            spawn_tank_event.position,
            &mut tank_id_counter,
            Some(spawn_tank_event.player.clone()),
            spawn_tank_event.strategy.clone(),
        );
    }
}
