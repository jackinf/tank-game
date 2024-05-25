use crate::features::animation::{AnimationActive, AnimationIndices, AnimationTimer};
use bevy::asset::{AssetServer, Assets};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, Camera2dBundle, Commands, Res, ResMut, SpriteSheetBundle, TextureAtlas,
    TextureAtlasLayout, Timer, TimerMode, Transform,
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
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first(),
            },
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        AnimationActive(false),
    ));
}
