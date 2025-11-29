use crate::features::con_menu::components::MoneyText;
use crate::features::con_menu::MenuInfo;
use bevy::prelude::{Query, Text, With};

pub fn update_money_text(
    q_menu_info: Query<&MenuInfo>,
    mut query: Query<&mut Text, With<MoneyText>>,
) {
    // TODO: check if it's not updated too often

    // Check if the MenuInfo resource has been updated
    let menu_info = q_menu_info.single().unwrap();
    for mut text in query.iter_mut() {
        // Update the text component
        **text = format!("Credits: {}", menu_info.get_money());
    }
}
