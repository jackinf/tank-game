use crate::features::con_menu::components::menu_info::MenuInfo;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{
    default, Component, Label, Query, Res, Style, Text, TextBundle, TextStyle, UiRect, Val, With,
};

#[derive(Component)]
pub struct MoneyText;

impl MoneyText {
    pub fn spawn(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder, money: i32) {
        parent
            .spawn((
                TextBundle::from_section(
                    format!("Credits: {}", money),
                    TextStyle {
                        font: asset_server.load("fonts/AmericanCaptain.ttf"),
                        font_size: 30.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                }),
                Label,
            ))
            .insert(MoneyText);
    }

    pub fn update(q_menu_info: Query<&MenuInfo>, mut query: Query<&mut Text, With<MoneyText>>) {
        // TODO: check if it's not updated too often

        // Check if the MenuInfo resource has been updated
        let menu_info = q_menu_info.single();
        for mut text in query.iter_mut() {
            // Update the text component
            text.sections[0].value = format!("Credits: {}", menu_info.get_money());
        }
    }
}