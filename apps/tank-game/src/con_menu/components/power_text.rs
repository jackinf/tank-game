use crate::common::resources::me::Me;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{
    default, Component, Label, Query, Res, Style, Text, TextBundle, TextStyle, UiRect, Val, With,
};

#[derive(Component)]
pub struct PowerText;

impl PowerText {
    pub fn spawn(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) {
        parent
            .spawn((
                TextBundle::from_section(
                    "Power: 0",
                    TextStyle {
                        font: asset_server.load("fonts/AmericanCaptain.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                }),
                Label,
            ))
            .insert(PowerText);
    }

    pub fn update(me: Res<Me>, mut query: Query<&mut Text, With<PowerText>>) {
        // TODO: check if it's not updated too often

        // Check if the MenuInfo resource has been updated
        for mut text in query.iter_mut() {
            // Update the text component
            text.sections[0].value = format!("Power: {}", me.get_energy() * 10);
        }
    }
}
