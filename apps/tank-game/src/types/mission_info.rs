use crate::features::building::types::buildings_layer::BuildingsLayer;
use crate::features::tile::GroundLayer;
use crate::features::unit::UnitsLayer;

#[derive(Debug)]
pub struct MissionInfo {
    pub ground_layer: GroundLayer,
    pub resource_layer: GroundLayer,
    pub buildings_layer: BuildingsLayer,
    pub units_layer: UnitsLayer,
}

impl MissionInfo {
    pub fn new(
        ground_layer: GroundLayer,
        resource_layer: GroundLayer,
        buildings_layer: BuildingsLayer,
        units_layer: UnitsLayer,
    ) -> Self {
        MissionInfo {
            ground_layer,
            resource_layer,
            buildings_layer,
            units_layer,
        }
    }
}
