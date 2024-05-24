use crate::features::tank::components::TankBullet;
use bevy::prelude::{
    default, AssetServer, Color, Commands, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3,
};

pub fn spawn_tank_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    from: Vec2,
    destination: Vec2,
) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("pixels/white.png"),
            transform: Transform::default()
                .with_translation(from.extend(100.))
                .with_scale(Vec3::splat(10.)),
            sprite: Sprite {
                color: Color::YELLOW,
                ..default()
            },
            ..default()
        })
        .insert(TankBullet::new(destination));
}
