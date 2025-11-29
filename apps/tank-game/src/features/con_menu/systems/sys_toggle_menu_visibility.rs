use crate::features::con_menu::SubMenuInfo;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, Visibility, With};

pub fn sys_toggle_menu_visibility(
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
