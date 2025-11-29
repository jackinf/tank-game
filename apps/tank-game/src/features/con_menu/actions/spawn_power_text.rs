use crate::features::con_menu::components::PowerText;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub fn spawn_power_text(asset_server: &Res<AssetServer>, parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Text::new("Power: 0"),
            TextFont {
                font: asset_server.load("fonts/AmericanCaptain.ttf"),
                font_size: 20.0,
                ..default()
            },
            Node {
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            Label,
            PowerText,
        ));
}
