use crate::menu::resources::menu_info::MenuInfo;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{
    default, Component, DetectChanges, Label, Query, Res, Style, Text, TextBundle, TextStyle,
    UiRect, Val, With,
};

#[derive(Component)]
pub struct MoneyText;

impl MoneyText {
    pub fn spawn(
        asset_server: &Res<AssetServer>,
        parent: &mut ChildBuilder,
        menu_info: Res<MenuInfo>,
    ) {
        parent
            .spawn((
                TextBundle::from_section(
                    format!("Credits: {}", menu_info.get_money()),
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

    pub fn update(menu_info: Res<MenuInfo>, mut query: Query<&mut Text, With<MoneyText>>) {
        // Check if the MenuInfo resource has been updated
        if menu_info.is_changed() {
            for mut text in query.iter_mut() {
                // Update the text component
                text.sections[0].value = format!("Money: {}", menu_info.get_money());
            }
        }
    }
}
