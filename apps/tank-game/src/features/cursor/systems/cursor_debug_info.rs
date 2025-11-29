use crate::features::cursor::components::{TileCoordText, WorldCoordText};
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub fn cursor_debug_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Cursor: (##, ##)"),
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
                        WorldCoordText,
                    ));

                    parent.spawn((
                        Text::new("Tile: (##, ##)"),
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
                        TileCoordText,
                    ));
                });
        });
}
