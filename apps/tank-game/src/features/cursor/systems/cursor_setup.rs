use crate::features::cursor::components::{TileCoordText, WorldCoordText};
use bevy::asset::AssetServer;
use bevy::prelude::{
    default, AlignItems, BuildChildren, Commands, FlexDirection, JustifyContent, Label, NodeBundle,
    PositionType, Res, Style, TextBundle, TextStyle, UiRect, Val,
};

pub fn cursor_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                ..Default::default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            TextBundle::from_section(
                                "Cursor: (##, ##)",
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
                        .insert(WorldCoordText);

                    parent
                        .spawn((
                            TextBundle::from_section(
                                "Tile: (##, ##)",
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
                        .insert(TileCoordText);
                });
        });
}
