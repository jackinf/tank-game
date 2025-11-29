//! This example illustrates the various features of Bevy UI.

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_scroll)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);

    // root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
                ))
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.75, 0.15, 0.15)),
                        ))
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                Text::new("Text Example"),
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
                            ));
                        });
                });
            // right vertical fill
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Scrolling list"),
                        TextFont {
                            font: asset_server.load("fonts/AmericanCaptain.ttf"),
                            font_size: 25.,
                            ..default()
                        },
                        Label,
                    ));
                    // List with hidden overflow
                    parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(50.),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                        ))
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                ))
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent.spawn((
                                            Text::new(format!("Item {i}")),
                                            TextFont {
                                                font: asset_server
                                                    .load("fonts/AmericanCaptain.ttf"),
                                                font_size: 20.,
                                                ..default()
                                            },
                                            Label,
                                        ));
                                    }
                                });
                        });
                });
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(210.),
                        bottom: Val::Px(10.),
                        border: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    BorderColor::all(Color::from(bevy::color::palettes::css::GREEN)),
                    BackgroundColor(Color::srgb(0.4, 0.4, 1.)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.8, 1.)),
                    ));
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(1.0, 0.0, 0.)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(20.),
                                    bottom: Val::Px(20.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(1.0, 0.3, 0.3)),
                            ));
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(40.),
                                    bottom: Val::Px(40.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(1.0, 0.5, 0.5)),
                            ));
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(60.),
                                    bottom: Val::Px(60.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(1.0, 0.7, 0.7)),
                            ));
                            // alpha test
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(80.),
                                    bottom: Val::Px(80.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(1.0, 0.9, 0.9, 0.4)),
                            ));
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|parent| {
                    // bevy logo (image)
                    parent.spawn((
                        Node {
                            width: Val::Px(500.0),
                            height: Val::Px(125.0),
                            margin: UiRect::top(Val::VMin(5.)),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                        ImageNode::new(asset_server.load("branding/bevy_logo_dark_big.png")),
                    ));
                });
        });
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Node, &ChildOf, &ComputedNode)>,
    query_node: Query<&ComputedNode>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut node, child_of, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(child_of.parent()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            node.top = Val::Px(scrolling_list.position);
        }
    }
}
