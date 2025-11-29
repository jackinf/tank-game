use crate::features::con_menu::components::MoneyText;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub fn spawn_money_text(asset_server: &Res<AssetServer>, parent: &mut ChildSpawnerCommands, money: u32) {
    parent
        .spawn((
            Text::new(format!("Credits: {}", money)),
            TextFont {
                font: asset_server.load("fonts/AmericanCaptain.ttf"),
                font_size: 30.0,
                ..default()
            },
            Node {
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            Label,
            MoneyText,
        ));
}
