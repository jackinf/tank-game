use crate::features::building::components::UnitSpawner;
use crate::features::tank::events::SpawnTankEvent;
use crate::features::tank::TankStrategy;
use crate::types::player::Player;
use bevy::prelude::{EventWriter, Query, Res, Time, Timer};

pub fn sys_spawn_enemy_units(
    time: Res<Time>,
    mut query: Query<&mut UnitSpawner>,
    mut spawn_tank_event_writer: EventWriter<SpawnTankEvent>,
) {
    let delta = time.delta();

    query.iter_mut().for_each(|mut spawner| {
        if spawner.player != Some(Player::P2) {
            return;
        }

        let mut timer = &mut spawner.spawn_timer;

        timer.tick(delta);
        if timer.finished() {
            timer.reset();
            let position = spawner.spawn_position;
            spawn_tank_event_writer.send(SpawnTankEvent {
                position,
                player: Player::P2,
                strategy: TankStrategy::Aggressive,
            });
        }
    });
}
