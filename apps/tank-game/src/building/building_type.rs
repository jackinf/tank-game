use crate::common::constants::TileSize;

#[derive(Clone, Debug)]
pub enum BuildingType {
    Base = 10,
    Factory = 20,
    PowerPlant = 30,
}

impl TryFrom<usize> for BuildingType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            10 => Ok(BuildingType::Base),
            20 => Ok(BuildingType::Factory),
            30 => Ok(BuildingType::PowerPlant),
            _ => Err(()),
        }
    }
}

impl BuildingType {
    pub fn get_building_type_sprite(&self) -> String {
        match &self {
            BuildingType::Base => "sprites/building_a.png".into(),
            BuildingType::Factory => "sprites/factory.png".into(),
            BuildingType::PowerPlant => "sprites/powerplant.png".into(),
        }
    }

    pub fn get_building_type_layer(&self) -> f32 {
        match &self {
            BuildingType::Base => 20.,
            BuildingType::Factory => 20.,
            BuildingType::PowerPlant => 20.,
        }
    }

    pub fn get_size(&self) -> TileSize {
        match &self {
            BuildingType::Base => (2, 2),
            BuildingType::Factory => (2, 2),
            BuildingType::PowerPlant => (2, 2),
        }
    }
}
