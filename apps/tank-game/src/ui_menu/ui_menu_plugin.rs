use bevy::prelude::*;

pub struct UiMenuPlugin;

impl Plugin for UiMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            add_money_text(asset_server, parent);

                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(200.0),
                                    // position_type: PositionType::Absolute,
                                    // left: Val::Px(210.),
                                    // bottom: Val::Px(10.),
                                    border: UiRect::all(Val::Px(20.)),
                                    ..default()
                                },
                                border_color: Color::GREEN.into(),
                                background_color: Color::rgb(0.4, 0.4, 1.).into(),
                                ..default()
                            });
                        });
                });
        });
}

fn add_money_text(asset_server: Res<AssetServer>, parent: &mut ChildBuilder) {
    parent.spawn((
        TextBundle::from_section(
            "CR: 5000",
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
    ));
}
