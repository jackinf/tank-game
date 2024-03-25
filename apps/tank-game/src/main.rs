use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_rapier2d::na::Quaternion;

#[derive(Component)]
struct Tank {
    id: Id,
    selected: bool
}

#[derive(Component)]
struct Id(usize);

#[derive(Component)]
struct SelectedUnit(bool);

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Resource)]
struct TankLogTimer(Timer);

#[derive(Resource)]
struct TankIdCounter(usize);

#[derive(Component)]
struct TargetPosition {
    position: Vec2,
    speed: f32, // Units per second
    moving: bool,
}

const MAX_WIDTH: u16 = 2000;
const MAX_HEIGHT: u16 = 1500;
const TILE_SIZE: f32 = 128.0;
// const TILE_EMPTY: usize = 0;
const TILE_TANK: usize = 1;
const OFFSET_X: f32 = -800.0;
const OFFSET_Y: f32 = -200.0;
// const GRASS_SCALE: f32 = 12.0;
// const TANK_SCALE: f32 = 1.5;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(MAX_WIDTH as f32, MAX_HEIGHT as f32)
                        .with_scale_factor_override(1.0),
                    title: "Tank Game".into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(MyWorldCoords(Vec2::new(0.0, 0.0)))
        // .insert_resource(TargetPosition {position: Vec2::new(0.0, 0.0), speed: 0.0})
        .insert_resource(TankLogTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(TankIdCounter(1))
        .add_systems(PreStartup, (setup))
        .add_systems(Update, (track_cursor, set_target_to_move, inflate_tank))
        // .add_systems(FixedUpdate, (logger, move_towards_target))
        .add_systems(FixedUpdate, (move_towards_target))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut tank_id_counter: ResMut<TankIdCounter>) {
    commands.spawn(Camera2dBundle::default());

    // 0 - empty, 1 - tank
    let tilemap = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
    ];

    let tilemap_small = vec![
      vec![0, 0, 0],
      vec![0, 0, 0],
      vec![0, 1, 0],
    ];

    // draw_tiles(&mut commands, &asset_server, tilemap);
    tilemap
        .into_iter()
        .enumerate()
        .for_each(|(col_index, row_on_row)| {
            row_on_row.into_iter().enumerate().for_each(|(row_index, cell)| {
                let x = row_index as f32 * TILE_SIZE + OFFSET_X;
                let y = col_index as f32 * TILE_SIZE + OFFSET_Y;
                let pos = Vec2::new(x, y);

                spawn_grass(&mut commands, &asset_server, pos);

                if cell == TILE_TANK {
                    println!("tank pos: {:?}", pos);
                    spawn_tank(&mut commands, &asset_server, pos, &mut tank_id_counter);
                }
            });
        });
}

fn spawn_grass(commands: &mut Commands, asset_server: &Res<AssetServer>, translation: Vec2) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::default()
                .with_translation(translation.extend(0.0)),
                // .with_scale(Vec3::new(GRASS_SCALE, GRASS_SCALE, 0.0)),
            texture: asset_server.load("grass3.png"),
            ..default()
        },
    ));
}

fn spawn_tank(commands: &mut Commands, asset_server: &Res<AssetServer>, translation: Vec2, tank_id_counter: &mut ResMut<TankIdCounter>) {
    let tank_id = tank_id_counter.0;
    tank_id_counter.0 += 1;

    let center_position = Vec2::new(translation.x - (TILE_SIZE / 2.0), translation.y - (TILE_SIZE / 2.0));
    let tank_base: Entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::default()
                    .with_translation(translation.extend(1.0)),
                texture: asset_server.load("tank3base.png"),
                ..default()
            },
        ))
        .insert(TargetPosition {
            position: center_position,
            speed: 0.0,
            moving: false,
        })
        .insert(Tank { id: Id(tank_id), selected: false })
        .id();

    commands
        .spawn((
            SpriteBundle {
                // transform: Transform::default()
                //     .with_translation(translation.extend(0.0)),
                transform: Transform::default()
                    .with_rotation(Quat::from(Quaternion::identity())), // TODO: add rotation
                texture: asset_server.load("tank3gun.png"),
                ..default()
            },
        )).set_parent(tank_base);
}

fn track_cursor(
    mut my_world_coords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.xy())
    {
        my_world_coords.0 = world_position;
    }
}

fn logger(tank_query: Query<&Tank>, mut timer: ResMut<TankLogTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        for tank in &tank_query {
            let id = tank.id .0;
            println!("Tank id: {}", id);
        }
    }
}

fn set_target_to_move(
    mut query: Query<(&mut TargetPosition, &mut Tank), With<Tank>>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    my_world_coords: Res<MyWorldCoords>,
) {
    for mouse_button_event in mouse_button_events.read() {
        match mouse_button_event.state {
            ButtonState::Pressed => {
                let wx = my_world_coords.0.x;
                let wy = my_world_coords.0.y;
                println!("clicked at {}", my_world_coords.0);

                // check if anything was selected at all
                let any_selected = query.iter().any(|(_, tank)| tank.selected);

                if any_selected {
                    // initiate the movement
                    for (mut target_position, _) in &mut query {
                        target_position.position = my_world_coords.0;
                        target_position.speed = 500.0;
                        target_position.moving = true;
                    }
                } else {
                    query.iter_mut()
                        // check if we clicked on the unit
                        .filter(|(position, _)| {
                            let x1 = position.position.x;
                            let x2 = position.position.x + TILE_SIZE;
                            let in_x = x1 <= wx && wx <= x2;
                            // println!("x1 {}, wx {}, x2 {}", x1, wx, x2);

                            let y1 = position.position.y;
                            let y2 = position.position.y + TILE_SIZE;
                            let in_y = y1 <= wy && wy <= y2;
                            // println!("y1 {}, wy {}, y2 {}", y1, wy, y2);

                            return in_x && in_y;
                        })
                        .for_each(|(_, mut tank)| tank.selected = true);
                }
            }
            _ => {}
        }
    }
}

fn move_towards_target(
    time: Res<Time>,
    mut query: Query<(&mut TargetPosition, &mut Transform, &Tank), With<Tank>>,
) {
    for (mut target_position, mut transform, tank) in query
        .iter_mut()
        .filter(|(target_position, _, tank)| target_position.moving && tank.selected)
    {
        let current_pos = transform.translation.xy();
        let direction = target_position.position - current_pos;
        let distance_to_move = target_position.speed * time.delta_seconds();

        if (direction.length() > distance_to_move) {
            let new_pos = current_pos + direction.normalize() * distance_to_move;
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
        } else {
            transform.translation.x = target_position.position.x;
            transform.translation.y = target_position.position.y;
            target_position.moving = false;
        }
    }
}

fn inflate_tank(mut query: Query<&mut Transform, With<Tank>>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut transform in &mut query {
            transform.scale *= 1.25;
        }
    }
}
