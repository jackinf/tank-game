use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

#[derive(Component)]
struct Tank(Id);

#[derive(Component)]
struct Id(usize);

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Resource)]
struct TankLogTimer(Timer);

#[derive(Component)]
struct TargetPosition {
    position: Vec2,
    speed: f32, // Units per second
    moving: bool,
}

const MAX_WIDTH: u16 = 1000;
const MAX_HEIGHT: u16 = 600;

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
        .add_systems(PreStartup, (setup))
        .add_systems(Update, (track_cursor, set_target_to_move, inflate_tank))
        .add_systems(FixedUpdate, (logger, move_towards_target))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    spawn_tank(&mut commands, &asset_server);
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
        .map(|ray| ray.origin.truncate())
    {
        my_world_coords.0 = world_position;
    }
}

fn spawn_tank(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::default().with_translation(Vec3::new(10.0, 0.0, 0.0)),
            texture: asset_server.load("ball_red_large.png"),
            ..default()
        })
        .insert(TargetPosition {
            position: Vec2::new(0.0, 0.0),
            speed: 0.0,
            moving: false,
        })
        .insert(Tank(Id(1)));
}

fn logger(tank_query: Query<&Tank>, mut timer: ResMut<TankLogTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        for tank in &tank_query {
            let id = tank.0 .0;
            println!("Tank id: {}", id);
        }
    }
}

fn set_target_to_move(
    mut query: Query<&mut TargetPosition, With<Tank>>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    my_world_coords: Res<MyWorldCoords>,
) {
    for mouse_button_event in mouse_button_events.read() {
        match mouse_button_event.state {
            ButtonState::Pressed => {
                println!("clicked at {}", my_world_coords.0);
                for mut target_position in &mut query {
                    target_position.position = my_world_coords.0;
                    target_position.speed = 500.0;
                    target_position.moving = true;
                }
            }
            _ => {}
        }
    }
}

fn move_towards_target(
    time: Res<Time>,
    mut query: Query<(&mut TargetPosition, &mut Transform), With<Tank>>,
) {
    for (mut target_position, mut transform) in query
        .iter_mut()
        .filter(|(target_position, _)| target_position.moving)
    {
        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
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
