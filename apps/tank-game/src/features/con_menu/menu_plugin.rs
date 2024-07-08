use crate::features::con_menu::systems::{
    building_process, detect_mouse_over_container, interact_with_construction_menu, setup,
    toggle_menu_visibility, update_money_text, update_power_text,
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
                interact_with_construction_menu,
                building_process,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}
