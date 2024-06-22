use bevy::prelude::{BackgroundColor, BorderColor, Button, Changed, Children, Color, Interaction, Query, With};
use crate::features::con_menu::components::MenuCellInfo;

pub fn interact_with_menu(mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
        &MenuCellInfo,
    ),
    (Changed<Interaction>, With<Button>),
>,) {
    for (interaction, mut color, mut border_color, children, info) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("INFO: {:?}", info);
                color.0 = Color::DARK_GREEN;
                // TODO: enqueue the action
            }
            Interaction::Hovered => {
                color.0 = Color::GREEN;
            }
            Interaction::None => {
                color.0 = Color::WHITE;
            }
        }
    }
}