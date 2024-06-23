use crate::features::con_menu::ConMenuBuildingType;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum BuildingTileType {
    Base = 10,
    Factory = 20,
    PowerPlant = 30,
}

impl BuildingTileType {
    pub fn from_con_menu_building_type(con_menu_tile_type: &ConMenuBuildingType) -> Self {
        match con_menu_tile_type {
            ConMenuBuildingType::Base => BuildingTileType::Base,
            ConMenuBuildingType::Factory => BuildingTileType::Factory,
            ConMenuBuildingType::PowerPlant => BuildingTileType::PowerPlant,
        }
    }
}
