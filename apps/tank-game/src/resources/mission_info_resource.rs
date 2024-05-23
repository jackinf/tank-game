use crate::features::building::types::buildings_layer::BuildingsLayer;
use crate::features::tile::GroundLayer;
use crate::features::unit::UnitsLayer;
use crate::types::mission_info::MissionInfo;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct MissionInfoResource {
    loaded: bool,
    ground_layer: GroundLayer,
    resource_layer: GroundLayer,
    buildings_layer: BuildingsLayer,
    units_layer: UnitsLayer,
}

impl MissionInfoResource {
    pub fn new() -> Self {
        MissionInfoResource {
            loaded: false,
            ground_layer: GroundLayer::empty(),
            resource_layer: GroundLayer::empty(),
            buildings_layer: BuildingsLayer::empty(),
            units_layer: UnitsLayer::empty(),
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.loaded
    }

    pub fn initialize(&mut self, mission_info: MissionInfo) {
        self.ground_layer = mission_info.ground_layer;
        self.resource_layer = mission_info.resource_layer;
        self.buildings_layer = mission_info.buildings_layer;
        self.units_layer = mission_info.units_layer;
        self.loaded = true;
    }

    pub fn get_ground_layer(&self) -> GroundLayer {
        self.ground_layer.clone()
    }

    pub fn get_resource_layer(&self) -> GroundLayer {
        self.resource_layer.clone()
    }

    pub fn get_buildings_layer(&self) -> BuildingsLayer {
        self.buildings_layer.clone()
    }

    pub fn get_units_layer(&self) -> UnitsLayer {
        self.units_layer.clone()
    }
}
