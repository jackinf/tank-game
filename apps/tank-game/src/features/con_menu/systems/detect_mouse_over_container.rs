use crate::features::con_menu::MenuInfo;
use bevy::prelude::{Button, Changed, Interaction, Query, Without};

pub fn detect_mouse_over_container(
    query: Query<&Interaction, (Changed<Interaction>, Without<Button>)>,
    mut q_menu_info: Query<&mut MenuInfo>,
) {
    let mut menu_info = q_menu_info.single_mut();
    for interaction in query.iter() {
        match *interaction {
            Interaction::Hovered => menu_info.set_hovered(true),
            Interaction::None => menu_info.set_hovered(false),
            _ => {} // Handle other states as needed
        }
    }
}
