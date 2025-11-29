use crate::features::unit::systems::{
    sys_calculate_selection_rect_coordinates, sys_display_selection_rect,
    sys_spawn_unit_selection_rect,
};
use crate::AppState;
use bevy::prelude::*;

pub struct UnitSelectionPlugin;

impl Plugin for UnitSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sys_spawn_unit_selection_rect)
            .add_systems(
                Update,
                sys_calculate_selection_rect_coordinates.run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                sys_display_selection_rect.run_if(in_state(AppState::Playing)),
            );
    }
}
