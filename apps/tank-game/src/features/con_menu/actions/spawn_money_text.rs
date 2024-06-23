use crate::features::con_menu::components::MoneyText;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{default, Label, Res, Style, TextBundle, TextStyle, UiRect, Val};

pub fn spawn_money_text(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder, money: u32) {
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
