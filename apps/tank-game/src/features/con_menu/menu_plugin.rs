use crate::features::con_menu::systems::{
    sys_construction_process, sys_detect_mouse_over_container,
    sys_interact_with_building_construction_menu, sys_interact_with_unit_construction_menu,
    sys_setup, sys_toggle_menu_visibility, sys_update_menu_text,
};
use crate::AppState;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            sys_setup.run_if(in_state(AppState::PreparingUsingDynamicAssets)),
        )
        .add_systems(
            Update,
            (
                sys_detect_mouse_over_container,
                sys_update_menu_text,
                sys_toggle_menu_visibility,
                sys_interact_with_building_construction_menu,
                sys_interact_with_unit_construction_menu,
                sys_construction_process,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}
