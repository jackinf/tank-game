use bevy::app::{App, FixedUpdate, PreStartup};
use bevy::prelude::{Plugin, Update};

use crate::features::building::systems::sys_draw_construction_tiles::sys_draw_construction_tiles;
use crate::features::building::systems::sys_interact_with_building::sys_interact_with_building;
use crate::features::building::systems::sys_spawn_placement_tile::sys_spawn_placement_tile;
use crate::features::building::systems::sys_update_building_construction::sys_update_building_construction;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, sys_spawn_placement_tile)
            .add_systems(FixedUpdate, sys_update_building_construction)
            .add_systems(FixedUpdate, sys_draw_construction_tiles)
            .add_systems(Update, sys_interact_with_building);
    }
}
