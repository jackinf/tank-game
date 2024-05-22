use crate::features::con_menu::components::PowerText;
use crate::resources::me::Me;
use bevy::prelude::{Query, Res, Text, With};

pub fn update_power_text(me: Res<Me>, mut query: Query<&mut Text, With<PowerText>>) {
    // TODO: check if it's not updated too often

    // Check if the MenuInfo resource has been updated
    for mut text in query.iter_mut() {
        // Update the text component
        text.sections[0].value = format!("Power: {}", me.get_energy());
    }
}
