use bevy::app::Plugin;
use bevy::prelude::*;

use crate::components::Tank;
use crate::resources::TankLogTimer;

pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, logger);
    }
}

fn logger(tank_query: Query<&Tank>, mut timer: ResMut<TankLogTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        for tank in &tank_query {
            let id = tank.id.0;
            println!("Tank id: {}", id);
        }
    }
}
