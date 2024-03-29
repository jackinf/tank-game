use bevy::app::Plugin;
use bevy::prelude::*;

use crate::logger::tank_log_timer::TankLogTimer;
use crate::tank::tank::Tank;

pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankLogTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(FixedUpdate, logger);
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
