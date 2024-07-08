use crate::features::con_menu::systems::{
    construction_process, detect_mouse_over_container, interact_with_building_construction_menu,
    interact_with_unit_construction_menu, setup, toggle_menu_visibility, update_money_text,
    update_power_text,
};
use crate::AppState;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup.run_if(in_state(AppState::PreparingUsingDynamicAssets)),
        )
        .add_systems(
            Update,
            (
                detect_mouse_over_container,
                update_money_text,
                update_power_text,
                toggle_menu_visibility,
                interact_with_building_construction_menu,
                interact_with_unit_construction_menu,
                construction_process,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}
