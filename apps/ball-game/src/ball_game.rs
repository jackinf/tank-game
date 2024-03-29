use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Id(usize);

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct CollisionSound;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
}

const PLAYER_1: usize = 0;
const PLAYER_2: usize = 1;

const MAX_WIDTH: u16 = 1000;
const MAX_HEIGHT: u16 = 600;

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(MAX_WIDTH as f32, MAX_HEIGHT as f32)
                        .with_scale_factor_override(1.0),
                    title: "Rolling Game".into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0)) // 200px is a meter
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            },
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        })
        .add_systems(PreStartup, setup)
        .add_systems(FixedUpdate, (movement, collision))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    spawn_player(
        PLAYER_1,
        Vec2::new(-150.0, 0.0),
        "ball_blue_large.png",
        &mut commands,
        &asset_server,
    );
    spawn_player(
        PLAYER_2,
        Vec2::new(150.0, 0.0),
        "ball_red_large.png",
        &mut commands,
        &asset_server,
    );

    commands
        .spawn(SpriteBundle {
            transform: Transform::default()
                .with_translation(Vec3::new(-150.0, 200.0, 0.0))
                .with_scale(Vec3::new(0.5, 0.5, 0.5)),
            texture: asset_server.load("block_corner.png"),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::triangle(
            Vec2::new(-64.0, 64.0),
            Vec2::new(64.0, -64.0),
            Vec2::new(-64.0, -64.0),
        ))
        .insert(Restitution::coefficient(1.0));
}

const MOVE_FORCE: f32 = 10_000.0;

fn spawn_player(
    id: usize,
    location: Vec2,
    path: &'static str,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::default()
                .with_translation(location.extend(0.0))
                .with_scale(Vec3::new(0.5, 0.5, 0.5)),
            texture: asset_server.load(path),
            ..default()
        })
        // .insert(input_manager_bundle)
        .insert(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(Action::Move, DualAxis::left_stick())
                .insert(
                    Action::Move,
                    if id == 0 {
                        VirtualDPad::wasd()
                    } else {
                        VirtualDPad::arrow_keys()
                    },
                )
                .set_gamepad(Gamepad { id })
                .build(),
        })
        .insert(Id(id))
        .insert(Name(path.to_string()))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(64.0))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        // slow down the movement so it does not go forever
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 5.0,
        })
        // 0.0 is sudden stop when hit another target, 1.0 is a full bounce
        .insert(Restitution::coefficient(1.0))
        .insert(Player);
}

fn movement(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>,
) {
    for (action_state, mut external_force) in &mut query {
        let axis_vector: Vec2 = action_state.clamped_axis_pair(&Action::Move).unwrap().xy();
        external_force.force = axis_vector * MOVE_FORCE * time.delta_seconds();
    }
}

fn collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    id_query: Query<&Id>,
    name_query: Query<&Name>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                commands.spawn(AudioBundle {
                    source: asset_server.load("impactGlass_heavy_002.ogg"),
                    ..default()
                });

                let name1 = name_query
                    .get(*e1)
                    .map(|name| name.0.clone())
                    .unwrap_or_else(|_| "Unknown".to_string());
                let name2 = name_query
                    .get(*e2)
                    .map(|name| name.0.clone())
                    .unwrap_or_else(|_| "Unknown".to_string());

                let id1 = id_query.get(*e1).map(|id| id.0).unwrap_or(0);
                let id2 = id_query.get(*e2).map(|id| id.0).unwrap_or(0);

                println!("Received collision event ids: {:?} vs {:?}", id1, id2);
                println!("Received collision event names: {:?} vs {:?}", name1, name2);
            }
            _ => {}
        }
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
