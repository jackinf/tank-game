use crate::features::tank::components::TankBullet;
use bevy::prelude::{
    default, AssetServer, Color, Commands, Res, Sprite, Transform, Vec2, Vec3,
};

pub fn spawn_tank_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    from: Vec2,
    destination: Vec2,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("pixels/white.png"),
            color: Color::from(bevy::color::palettes::basic::YELLOW),
            ..default()
        },
        Transform::default()
            .with_translation(from.extend(100.))
            .with_scale(Vec3::splat(10.)),
        TankBullet::new(destination),
    ));
}
