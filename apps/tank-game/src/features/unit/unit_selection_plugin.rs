use bevy::prelude::*;

use crate::features::unit::systems::{
    calculate_selection_rect_coordinates, display_selection_rect, spawn_unit_selection_rect,
};

pub struct UnitSelectionPlugin;

impl Plugin for UnitSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_unit_selection_rect)
            .add_systems(Update, calculate_selection_rect_coordinates)
            .add_systems(FixedUpdate, display_selection_rect);
    }
}
