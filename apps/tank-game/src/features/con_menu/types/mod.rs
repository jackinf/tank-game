use std::any::Any;

pub trait ConMenuType: Any {
    fn image_path(&self) -> String;
    fn price(&self) -> u32;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone)]
pub enum ConMenuBuildingType {
    Base,
    Factory,
    PowerPlant,
}

impl ConMenuType for ConMenuBuildingType {
    fn image_path(&self) -> String {
        match self {
            ConMenuBuildingType::Base => "sprites/building_b_tr.png".to_string(),
            ConMenuBuildingType::Factory => "sprites/building_c_tr.png".to_string(),
            ConMenuBuildingType::PowerPlant => "sprites/building_d_tr.png".to_string(),
        }
    }

    fn price(&self) -> u32 {
        match self {
            ConMenuBuildingType::Base => 1000,
            ConMenuBuildingType::Factory => 500,
            ConMenuBuildingType::PowerPlant => 500,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub enum ConMenuVehicleType {
    TankA,
    TankB,
    TankC,
}

impl ConMenuType for ConMenuVehicleType {
    fn image_path(&self) -> String {
        match self {
            ConMenuVehicleType::TankA => "sprites/tank_a_tr.png".to_string(),
            ConMenuVehicleType::TankB => "sprites/tank_b_tr.png".to_string(),
            ConMenuVehicleType::TankC => "sprites/tank_c_tr.png".to_string(),
        }
    }

    fn price(&self) -> u32 {
        match self {
            ConMenuVehicleType::TankA => 100,
            ConMenuVehicleType::TankB => 200,
            ConMenuVehicleType::TankC => 300,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
