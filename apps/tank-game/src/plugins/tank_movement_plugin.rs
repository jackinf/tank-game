use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

use crate::common::constants::TILE_SIZE;
use crate::common::resources::{TankIdCounter, TankLogTimer, WorldCoordinates};
use crate::components::{Tank, TankGun, TilePosition};
use crate::game_setup::setup;

pub struct TankMovementPlugin;

impl Plugin for TankMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankLogTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(TankIdCounter(1))
            .add_systems(PreStartup, setup)
            .add_systems(Update, set_tank_target_position_to_move)
            .add_systems(FixedUpdate, move_tanks_towards_target);
    }
}

fn set_tank_target_position_to_move(
    mut tank_query: Query<(&mut Tank, &mut Sprite), With<Tank>>,
    tile_query: Query<&TilePosition>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut key_button_events: EventReader<KeyboardInput>,
    my_world_coords: Res<WorldCoordinates>,
) {
    for key_button_event in key_button_events.read() {
        if let ButtonState::Pressed = key_button_event.state {
            // selects everything
            if key_button_event.key_code == KeyCode::Space {
                for (mut tank, mut sprite) in &mut tank_query.iter_mut() {
                    select_tank(&mut tank, &mut sprite);
                }
            }

            // unselects everything
            if key_button_event.key_code == KeyCode::Escape {
                for (mut tank, mut sprite) in &mut tank_query.iter_mut() {
                    unselect_tank(&mut tank, &mut sprite);
                }
            }
        }
    }

    for mouse_button_event in mouse_button_events.read() {
        if let ButtonState::Pressed = mouse_button_event.state {
            let wx = my_world_coords.0.x;
            let wy = my_world_coords.0.y;

            let clicked_on_tank = tank_query
                .iter_mut()
                .find(|(tank, _)| is_tank_clicked_on(wx, wy, tank));

            if let Some((mut tank, mut sprite)) = clicked_on_tank {
                select_tank(&mut tank, &mut sprite);
            } else {
                if let Some(tile) = tile_query.iter().find(|tile| tile.in_range(wx, wy)) {
                    for (mut tank, _) in
                        &mut tank_query.iter_mut().filter(|(tank, _)| tank.selected)
                    {
                        tank.start_moving_to(tile.get_center());
                    }
                }
            }
        }
    }
}

fn select_tank(tank: &mut Mut<Tank>, sprite: &mut Mut<Sprite>) {
    tank.selected = true;
    sprite.color = Color::rgb(1.0, 9.0, 8.0);
}

fn unselect_tank(tank: &mut Mut<Tank>, sprite: &mut Mut<Sprite>) {
    tank.moving = false;
    tank.selected = false;
    sprite.color = Color::WHITE;
}

fn is_tank_clicked_on(wx: f32, wy: f32, tank: &Mut<Tank>) -> bool {
    let x1 = tank.target_position.x;
    let x2 = tank.target_position.x + TILE_SIZE;
    let in_x = x1 <= wx && wx <= x2;

    let y1 = tank.target_position.y;
    let y2 = tank.target_position.y + TILE_SIZE;
    let in_y = y1 <= wy && wy <= y2;

    in_x && in_y
}

fn move_tanks_towards_target(
    time: Res<Time>,
    mut tank_query: Query<(&mut Transform, &mut Tank), (With<Tank>, Without<TankGun>)>,
    mut gun_query: Query<(&mut Transform, &TankGun), (With<TankGun>, Without<Tank>)>,
) {
    for (mut transform, mut tank) in tank_query
        .iter_mut()
        .filter(|(_, tank)| tank.moving && tank.selected)
    {
        let current_pos = transform.translation.xy();
        let direction = tank.target_position - current_pos;
        let distance_to_move = tank.speed * time.delta_seconds();

        // Smooth movement
        if direction.length() > distance_to_move {
            let new_pos = current_pos + direction.normalize() * distance_to_move;
            let target_vec3 = Vec3::new(new_pos.x, new_pos.y, transform.translation.z);

            // TODO: account for a bug if the speed is too high.
            // if so, use simpler:
            // transform.translation = Vec3::new(new_pos.x, new_pos.y, transform.translation.z);
            transform.translation = transform
                .translation
                .lerp(target_vec3, tank.speed / 10.0 * time.delta_seconds());
        } else {
            transform.translation.x = tank.target_position.x;
            transform.translation.y = tank.target_position.y;
            tank.stop();
        }

        // Rotate tank gun smoothly
        if let Some((mut gun_transform, _)) = gun_query
            .iter_mut()
            .find(|(_, gun)| gun.parent_id.0 == tank.id.0)
        {
            let target_angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
            gun_transform.rotation = gun_transform.rotation.slerp(
                Quat::from_rotation_z(target_angle),
                10.0 * time.delta_seconds(),
            );
        }
    }
}
