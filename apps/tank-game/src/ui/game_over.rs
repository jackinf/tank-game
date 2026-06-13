//! The victory / defeat overlay and the "press R to play again" input.

use super::*;
use crate::state::{GameResult, GameState};
use bevy::prelude::*;

pub(super) fn spawn_game_over(mut commands: Commands, result: Option<Res<GameResult>>) {
    let (msg, color) = match result.as_deref() {
        Some(GameResult::Victory) => ("VICTORY!", Color::srgb(0.4, 1.0, 0.5)),
        _ => ("DEFEAT", Color::srgb(1.0, 0.4, 0.4)),
    };
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            GameOverUi,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new(msg),
                TextFont { font_size: 72.0, ..default() },
                TextColor(color),
            ));
            p.spawn((
                Text::new("Press R to play again"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
}

pub(super) fn despawn_game_over(mut commands: Commands, ui: Query<Entity, With<GameOverUi>>) {
    for e in &ui {
        commands.entity(e).despawn();
    }
}

pub(super) fn restart_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::KeyR) {
        next.set(GameState::Loading);
    }
}
