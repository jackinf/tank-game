//! Per-building construction queues, prerequisites, and building placement.
//!
//! Every production building (Construction Yard, Barracks, War Factory) owns
//! its own [`ProductionQueue`] component, so two barracks build independently.
//!
//! - [`queue`]: the [`ProductionQueue`] / [`OwnedBuildings`] state and ticking.
//! - [`rules`]: prerequisite checks and what each building can make.
//! - [`placement`]: putting finished structures down on the map.

mod placement;
mod queue;
mod rules;

pub use placement::PlacementMode;
pub use queue::{OwnedBuildings, ProductionQueue};
pub use rules::{is_producer, prerequisites_met, producible_menu, try_enqueue};

use crate::state::GameState;
use bevy::prelude::*;

pub struct ProductionPlugin;

impl Plugin for ProductionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OwnedBuildings>()
            .init_resource::<PlacementMode>()
            .add_systems(
                Update,
                (
                    queue::update_owned_buildings,
                    queue::tick_production,
                    placement::placement_controls,
                    placement::placement_input,
                    placement::draw_placement_preview,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
