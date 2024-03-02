use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

const MAX_WIDTH: u16 = 1000;
const MAX_HEIGHT: u16 = 600;

pub fn main() {
    App::new()
        // .insert_resource(Window {
        //     title: "Rolling Game".into(),
        //     ..Default::default()
        // })
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
        .add_systems(PreStartup, (setup))
        // .add_plugins(DefaultPlugins)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-150.0, 0.0, 1.0)),
        texture: asset_server.load("ball_blue_large.png"),
        ..default()
    });
}