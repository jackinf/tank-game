use bevy::app::Plugin;
use bevy::prelude::*;

use crate::debug::tank_log_timer::TankLogTimer;
use crate::tank::tank::Tank;
use crate::ui_menu::menu_info::MenuInfo;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankLogTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, inflate_all_tanks)
            .add_systems(Update, buying_stuff)
            .add_systems(Update, damage_selected_tanks)
            .add_systems(FixedUpdate, logger);
    }
}

fn logger(tank_query: Query<&Tank>, mut timer: ResMut<TankLogTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        for tank in &tank_query {
            let id = tank.id.0;
            // println!("Tank id: {}", id);
        }
    }
}

fn inflate_all_tanks(
    mut query: Query<&mut Transform, With<Tank>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        for mut transform in &mut query {
            transform.scale *= 1.25;
        }
    }
}

fn buying_stuff(mut menu_info: ResMut<MenuInfo>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        menu_info.add_money(-100);
        println!("Buying stuff, money left: {}", menu_info.get_money());
    }
}

fn damage_selected_tanks(mut query: Query<&mut Tank>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyN) {
        for mut tank in &mut query.iter_mut().filter(|tank| tank.selected) {
            tank.take_damage(30);
            println!("Tank health: {}", tank.health);
        }
    }
}
