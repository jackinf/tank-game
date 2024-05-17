use crate::types::buildings_layer::BuildingsLayer;
use crate::types::ground_layer::GroundLayer;
use crate::types::units_layer::UnitsLayer;

#[derive(Debug)]
pub struct MissionInfo {
    pub ground_layer: GroundLayer,
    pub buildings_layer: BuildingsLayer,
    pub units_layer: UnitsLayer,
}

impl MissionInfo {
    pub fn new(
        ground_layer: GroundLayer,
        buildings_layer: BuildingsLayer,
        units_layer: UnitsLayer,
    ) -> Self {
        MissionInfo {
            ground_layer,
            buildings_layer,
            units_layer,
        }
    }
}
