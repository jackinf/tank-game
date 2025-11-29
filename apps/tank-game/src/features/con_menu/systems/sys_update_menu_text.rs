//! Combined system for updating menu text elements (money and power)

use crate::features::con_menu::components::{MoneyText, PowerText};
use crate::features::con_menu::MenuInfo;
use bevy::prelude::{Query, Text, With, Without};

/// Updates both money and power text displays in a single system
pub fn sys_update_menu_text(
    q_menu_info: Query<&MenuInfo>,
    mut q_money: Query<&mut Text, (With<MoneyText>, Without<PowerText>)>,
    mut q_power: Query<&mut Text, (With<PowerText>, Without<MoneyText>)>,
) {
    let Ok(menu_info) = q_menu_info.single() else {
        return;
    };

    // Update money text
    for mut text in q_money.iter_mut() {
        **text = format!("Credits: {}", menu_info.get_money());
    }

    // Update power text
    for mut text in q_power.iter_mut() {
        **text = format!("Power: {}", menu_info.energy());
    }
}

