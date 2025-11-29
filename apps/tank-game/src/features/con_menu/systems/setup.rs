use crate::features::con_menu::actions::{
    show_con_base_menu_grid, show_factory_menu_grid, spawn_money_text, spawn_power_text,
};
use crate::features::con_menu::MenuInfo;
use crate::types::main_asset_info_resource::MainAssetInfoResource;
use crate::AppState;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    main_asset_info_resource: ResMut<MainAssetInfoResource>,
    mut state: ResMut<NextState<AppState>>,
) {
    // Assume you have your sprite sheet or individual sprites ready
    let menu_info = MenuInfo::new();

    // Create a parent entity for the grid
    commands
        .spawn((
            Node {
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            Interaction::None,
            menu_info.clone(),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|row_parent| {
                    spawn_money_text(&asset_server, row_parent, menu_info.get_money());
                    spawn_power_text(&asset_server, row_parent);
                });

            show_factory_menu_grid(parent, &asset_server, &main_asset_info_resource);
            show_con_base_menu_grid(parent, &asset_server, &main_asset_info_resource);
        });

    state.set(AppState::Playing);
}
