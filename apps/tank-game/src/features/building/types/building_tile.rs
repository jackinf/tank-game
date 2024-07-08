use crate::constants::{TileSize, HEALTH_BAR_HEIGHT};
use crate::features::building::types::building_tile_type::BuildingTileType;
use crate::features::con_menu::SubMenuType;
use crate::types::player::Player;
use crate::types::{AssetTile, AssetTileSubType, AssetTileType};
use bevy::math::{Rect, Vec2};
use bevy::prelude::{Timer, TimerMode};

#[derive(Clone, Debug, PartialEq)]
pub struct BuildingTile {
    image_path: String,
    tile_size: TileSize,
    building_type: BuildingTileType,
    player: Option<Player>,
}

#[derive(Debug)]
pub enum BuildingTileErrors {
    TileTypeIsRequired,
    TileSubTypeIsRequired,
    InvalidBuildingType { message: String },
}

impl BuildingTile {
    pub fn new(
        image_path: String,
        tile_size: TileSize,
        building_type: BuildingTileType,
        player: Option<Player>,
    ) -> Self {
        BuildingTile {
            image_path,
            tile_size,
            building_type,
            player,
        }
    }

    pub fn get_image_path(&self) -> String {
        self.image_path.clone()
    }

    pub fn get_layer(&self) -> f32 {
        match &self.get_building_type() {
            BuildingTileType::Base => 20.,
            BuildingTileType::Factory => 20.,
            BuildingTileType::PowerPlant => 20.,
        }
    }

    pub fn get_size(&self) -> TileSize {
        self.tile_size
    }

    pub fn get_max_health(&self) -> u32 {
        match &self.get_building_type() {
            BuildingTileType::Base => 1000,
            BuildingTileType::Factory => 500,
            BuildingTileType::PowerPlant => 500,
        }
    }

    pub fn get_price(&self) -> u32 {
        match &self.get_building_type() {
            BuildingTileType::Base => 1000,
            BuildingTileType::Factory => 500,
            BuildingTileType::PowerPlant => 300,
        }
    }

    pub fn get_player(&self) -> Option<Player> {
        self.player.clone()
    }

    pub fn get_building_type(&self) -> BuildingTileType {
        self.building_type.clone()
    }

    pub fn radius(&self) -> f32 {
        match &self.get_building_type() {
            BuildingTileType::Base => 500.0,
            BuildingTileType::Factory => 500.0,
            BuildingTileType::PowerPlant => 500.0,
        }
    }

    // TODO: what is this for?
    pub fn get_sub_menu_type(&self) -> Option<SubMenuType> {
        match &self.get_building_type() {
            BuildingTileType::Base => Some(SubMenuType::Base),
            BuildingTileType::Factory => Some(SubMenuType::Factory),
            _ => None,
        }
    }

    pub fn get_power_level(&self) -> i32 {
        match &self.get_building_type() {
            BuildingTileType::Base => -50,
            BuildingTileType::Factory => -20,
            BuildingTileType::PowerPlant => 100,
        }
    }

    pub fn get_health_rect(&self, current_health: u32) -> Rect {
        let max_health = self.get_max_health() as f32;
        let tile_width = 130. * self.get_size().0 as f32;
        let health_bar_width = tile_width * (current_health as f32 / max_health);
        Rect {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(health_bar_width, HEALTH_BAR_HEIGHT),
        }
    }

    pub fn get_health_rect_default(&self) -> Rect {
        self.get_health_rect(self.get_max_health())
    }

    pub fn get_spawn_interval(&self) -> f32 {
        match &self.get_building_type() {
            BuildingTileType::Base => 100.0,
            BuildingTileType::Factory => 20.0,
            BuildingTileType::PowerPlant => 5.0, // does not make sense.
        }
    }

    pub fn get_spawn_timer(&self) -> Timer {
        Timer::from_seconds(self.get_spawn_interval(), TimerMode::Repeating)
    }
}

pub fn create_building_tile(
    value: AssetTile,
    player: Option<Player>,
) -> Result<BuildingTile, BuildingTileErrors> {
    let tile_type = value.get_tile_type();
    let tile_sub_type = value.get_tile_sub_type();

    if tile_type.is_none() {
        return Err(BuildingTileErrors::TileTypeIsRequired);
    }

    if tile_sub_type.is_none() {
        return Err(BuildingTileErrors::TileSubTypeIsRequired);
    }

    let tile_type = tile_type.unwrap();
    let tile_sub_type = tile_sub_type.unwrap();
    if tile_type != AssetTileType::Building {
        return Err(BuildingTileErrors::InvalidBuildingType {
            message: format!(
                "'{}' is not a valid AssetTileType",
                tile_sub_type.to_string()
            ),
        });
    }

    let building_tile_type = match tile_sub_type {
        AssetTileSubType::Base => Ok(BuildingTileType::Base),
        AssetTileSubType::Factory => Ok(BuildingTileType::Factory),
        AssetTileSubType::Powerplant => Ok(BuildingTileType::PowerPlant),
        _ => Err(BuildingTileErrors::InvalidBuildingType {
            message: format!(
                "'{}' is not a valid BuildingTileType",
                tile_sub_type.to_string()
            ),
        }),
    };

    if let Err(e) = building_tile_type {
        return Err(e);
    }
    let building_tile_type = building_tile_type.unwrap();

    Ok(BuildingTile::new(
        value.get_image_path(),
        value.get_tile_size(),
        building_tile_type,
        player,
    ))
}
