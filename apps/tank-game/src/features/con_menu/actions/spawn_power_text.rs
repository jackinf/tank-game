use crate::features::con_menu::components::PowerText;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{default, Label, Res, Style, TextBundle, TextStyle, UiRect, Val};

pub fn spawn_power_text(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) {
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
