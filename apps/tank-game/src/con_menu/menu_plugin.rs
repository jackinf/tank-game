use bevy::prelude::*;

use crate::con_menu::components::menu_info::MenuInfo;
use crate::con_menu::components::money_text::MoneyText;
use crate::con_menu::components::power_text::PowerText;
use crate::con_menu::components::submenu_info::SubMenuInfo;
use crate::con_menu::managers::base_menu_manager::BaseMenuManager;
use crate::con_menu::managers::factory_menu_manager::FactoryMenuManager;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(Update, detect_mouse_over_container)
            .add_systems(Update, MoneyText::update)
            .add_systems(Update, PowerText::update)
            .add_systems(Update, toggle_menu_visibility);
    }
}

fn toggle_menu_visibility(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Visibility, &SubMenuInfo), With<SubMenuInfo>>,
) {
    if keyboard.just_pressed(KeyCode::KeyN) {
        let (mut visibility, _) = query
            .iter_mut()
            .find(|(_, info)| info.is_factory())
            .unwrap();
        *visibility = Visibility::Visible;
    } else if keyboard.just_pressed(KeyCode::KeyV) {
        let (mut visibility, _) = query
            .iter_mut()
            .find(|(_, info)| info.is_factory())
            .unwrap();
        *visibility = Visibility::Hidden;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Assume you have your sprite sheet or individual sprites ready
    let menu_info = MenuInfo::new();

    // Create a parent entity for the grid
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .insert(Interaction::None)
        .insert(menu_info.clone())
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row_parent| {
                    MoneyText::spawn(&asset_server, row_parent, menu_info.get_money());
                    PowerText::spawn(&asset_server, row_parent);
                });

            FactoryMenuManager::show_menu(parent, &asset_server);
            BaseMenuManager::show_menu(parent, &asset_server);
        });
}

fn detect_mouse_over_container(
    query: Query<&Interaction, (Changed<Interaction>, Without<Button>)>,
    mut q_menu_info: Query<&mut MenuInfo>,
) {
    let mut menu_info = q_menu_info.single_mut();
    for interaction in query.iter() {
        match *interaction {
            Interaction::Hovered => menu_info.set_hovered(true),
            Interaction::None => menu_info.set_hovered(false),
            _ => {} // Handle other states as needed
        }
    }
}
