use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
}

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
        .add_systems(Update, movement)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        transform: Transform::default()
            .with_translation(Vec3::new(-150.0, 0.0, 1.0))
            .with_scale(Vec3::new(0.5, 0.5, 0.5)),
        texture: asset_server.load("ball_blue_large.png"),
        ..default()
    })
        // .insert(input_manager_bundle)
        .insert(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(Action::Move, DualAxis::left_stick())
                .set_gamepad(Gamepad { id: 0 })
                .build()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(64.0))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0
        })
        // slow down the movement so it does not go forever
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 5.0
        })
        // 0.0 is sudden stop when hit another target, 1.0 is a full bounce
        .insert(Restitution::coefficient(1.0))
        .insert(Player);
}

const MOVE_FORCE: f32 = 10_000.0;

fn movement(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>
) {
    for (action_state, mut external_force) in &mut query {
        let axis_vector: Vec2 = action_state.clamped_axis_pair(&Action::Move).unwrap().xy();
        external_force.force = axis_vector * MOVE_FORCE * time.delta_seconds();
    }
}