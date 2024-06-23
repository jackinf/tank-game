use crate::features::con_menu::types::{ConMenuBuildingType, ConMenuType, ConMenuVehicleType};
use bevy::prelude::Component;
use std::error::Error;

#[derive(Debug, Clone, Component)]
pub struct MenuCellInfo {
    building_type: Option<ConMenuBuildingType>,
    vehicle_type: Option<ConMenuVehicleType>,
}

impl MenuCellInfo {
    pub fn new(con_menu_type: &Box<dyn ConMenuType>) -> Self {
        // check if con_menu_type is a ConMenuBuildingType or ConMenuVehicleType
        let building_type =
            if let Some(building) = con_menu_type.as_any().downcast_ref::<ConMenuBuildingType>() {
                Some(building.clone())
            } else {
                None
            };

        let vehicle_type =
            if let Some(vehicle) = con_menu_type.as_any().downcast_ref::<ConMenuVehicleType>() {
                Some(vehicle.clone())
            } else {
                None
            };

        Self {
            building_type,
            vehicle_type,
        }
    }

    pub fn price(&self) -> Result<u32, &str> {
        if let Some(building_type) = &self.building_type {
            Ok(building_type.price())
        } else if let Some(vehicle_type) = &self.vehicle_type {
            Ok(vehicle_type.price())
        } else {
            Err("No building or vehicle type found")
        }
    }

    pub fn get_building_type(&self) -> Option<ConMenuBuildingType> {
        self.building_type.clone()
    }

    pub fn get_vehicle_type(&self) -> Option<ConMenuVehicleType> {
        self.vehicle_type.clone()
    }
}
