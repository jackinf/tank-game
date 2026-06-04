//! Factions / players.

use crate::config::*;
use bevy::prelude::*;

/// Which side an entity belongs to.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Faction {
    Player,
    Enemy,
    Neutral,
}

impl Faction {
    pub fn color(self) -> Color {
        match self {
            Faction::Player => PLAYER_COLOR,
            Faction::Enemy => ENEMY_COLOR,
            Faction::Neutral => NEUTRAL_COLOR,
        }
    }

    /// True if `self` and `other` are enemies of each other.
    pub fn is_hostile_to(self, other: Faction) -> bool {
        use Faction::*;
        matches!(
            (self, other),
            (Player, Enemy) | (Enemy, Player)
        )
    }
}
