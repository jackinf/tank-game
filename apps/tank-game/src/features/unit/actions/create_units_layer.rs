use crate::features::unit::actions::create_unit_tile;
use crate::features::unit::UnitsLayer;
use crate::types::mission_layer::MissionLayer;
use crate::types::PlayersLayer;

pub fn create_units_layer(mission_layer: MissionLayer, players_layer: &PlayersLayer) -> UnitsLayer {
    let units = mission_layer
        .get_tiles()
        .iter()
        .filter_map(|(coord, tile)| {
            create_unit_tile(tile.clone(), players_layer.get_by(coord))
                .ok()
                .map(|ground_tile| (*coord, ground_tile))
        })
        .collect();
    let width = mission_layer.get_width();
    let height = mission_layer.get_height();

    UnitsLayer::new(units, width, height)
}
