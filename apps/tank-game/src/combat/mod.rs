//! Targeting, weapons, projectiles and damage.
//!
//! The systems run in a chain each frame: units and turrets pick targets
//! ([`targeting`]), armed entities fire ([`weapons`]), then projectiles fly and
//! deal damage ([`projectiles`]).

mod projectiles;
mod targeting;
mod weapons;

use crate::state::GameState;
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                targeting::unit_brain,
                targeting::turret_targeting,
                weapons::fire_weapons,
                projectiles::update_projectiles,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}
