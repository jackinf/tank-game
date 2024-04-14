use crate::common::constants::TileSize;

#[derive(Clone)]
pub enum BuildingType {
    None = 0,
    Base = 10,
    Factory = 20,
    PowerPlant = 30,
}

impl From<usize> for BuildingType {
    fn from(value: usize) -> Self {
        match value {
            0 => BuildingType::None,
            10 => BuildingType::Base,
            20 => BuildingType::Factory,
            30 => BuildingType::PowerPlant,
            _ => panic!("Invalid building type: {}", value),
        }
    }
}

impl BuildingType {
    pub fn get_building_type_sprite(&self) -> String {
        match &self {
            BuildingType::None => panic!("Invalid building type: {:?}", self),
            BuildingType::Base => "sprites/building_a.png".into(),
            BuildingType::Factory => "sprites/factory.png".into(),
            BuildingType::PowerPlant => "sprites/powerplant.png".into(),
        }
    }

    pub fn get_building_type_layer(&self) -> f32 {
        match &self {
            BuildingType::None => panic!("Invalid building type: {:?}", self),
            BuildingType::Base => 20.,
            BuildingType::Factory => 20.,
            BuildingType::PowerPlant => 20.,
        }
    }

    pub fn get_size(&self) -> TileSize {
        match &self {
            BuildingType::None => panic!("Invalid building type: {:?}", self),
            BuildingType::Base => (2, 2),
            BuildingType::Factory => (2, 2),
            BuildingType::PowerPlant => (2, 2),
        }
    }
}
