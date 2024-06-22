use bevy::prelude::*;
use bevy::winit::WinitSettings;

use crate::features::con_menu::components::SubMenuInfo;
use crate::features::con_menu::systems::{detect_mouse_over_container, interact_with_menu, setup, update_money_text, update_power_text};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::desktop_app())
            .add_systems(Update, detect_mouse_over_container)
            .add_systems(Update, update_money_text)
            .add_systems(Update, update_power_text)
            .add_systems(Update, toggle_menu_visibility)
            .add_systems(Update, interact_with_menu);
    }
}

fn toggle_menu_visibility(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Visibility, &SubMenuInfo), With<SubMenuInfo>>,
) {
    if keyboard.just_pressed(KeyCode::KeyN) {
        let (mut visibility, _) = query
            .iter_mut()
            .find(|(_, info)| info.is_factory())
            .unwrap();
        *visibility = Visibility::Visible;
    } else if keyboard.just_pressed(KeyCode::KeyV) {
        let (mut visibility, _) = query
            .iter_mut()
            .find(|(_, info)| info.is_factory())
            .unwrap();
        *visibility = Visibility::Hidden;
    }
}
