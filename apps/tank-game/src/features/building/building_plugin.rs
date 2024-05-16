use bevy::app::{App, FixedUpdate, PreStartup};
use bevy::prelude::{Plugin, Update};

use crate::features::building::systems::draw_construction_tiles::draw_construction_tiles;
use crate::features::building::systems::interact_with_building::interact_with_building;
use crate::features::building::systems::setup::setup;
use crate::features::building::systems::update_building_construction::update_building_construction;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(FixedUpdate, update_building_construction)
            .add_systems(FixedUpdate, draw_construction_tiles)
            .add_systems(Update, interact_with_building);
    }
}
