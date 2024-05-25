use crate::features::explosion::components::{
    AnimationActive, AnimationIndices, AnimationTimer, SmallExplosion,
};
use bevy::asset::{AssetServer, Assets};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, Commands, Res, ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout, Timer,
    TimerMode, Transform,
};

pub fn prepare_explosion_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("animations/explosion.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(31., 35.), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices::new(0, 4);
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first(),
            },
            transform: Transform::default()
                .with_translation(Vec3::new(0.0, 0.0, 100.0))
                .with_scale(Vec3::splat(1.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        AnimationActive(false),
        SmallExplosion,
    ));
}
