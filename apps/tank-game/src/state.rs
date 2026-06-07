//! Top-level game states and the win/lose result.

use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    GameOver,
}

/// Outcome of a finished match.
#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameResult {
    Victory,
    Defeat,
}

/// Marker for any entity that belongs to a match and should be despawned on
/// restart.
#[derive(Component)]
pub struct GameEntity;
