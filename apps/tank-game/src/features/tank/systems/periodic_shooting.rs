use crate::features::tank::actions::spawn_tank_bullet;
use crate::features::tank::components::Tank;
use crate::features::unit::UnitId;
use bevy::audio::AudioBundle;
use bevy::prelude::{
    default, AssetServer, Commands, EventWriter, Query, Res, Time, Transform, Vec2, Vec3Swizzles,
    With,
};
use std::collections::HashMap;
use crate::features::building::components::Building;

pub fn periodic_shooting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_tanks: Query<(&mut Tank, &Transform), With<Tank>>,
    mut q_buildings: Query<(&mut Building, &Transform), With<Building>>,
    time: Res<Time>,
) {
    let tank_id_pos_map: HashMap<UnitId, Vec2> = q_tanks
        .iter()
        .map(|(tank, transform)| (tank.id.clone(), transform.translation.xy()))
        .collect();

    let building_id_pos_map: HashMap<UnitId, Vec2> = q_buildings
        .iter()
        .map(|(building, transform)| (building.id().clone(), transform.translation.xy()))
        .map(|(id, pos)| (id.clone(), Vec2::new(pos.x + 50.0, pos.y - 50.0)))
        .collect();

    let mut all_id_pos_map: HashMap<UnitId, Vec2> = HashMap::new();
    all_id_pos_map.extend(tank_id_pos_map.clone());
    all_id_pos_map.extend(building_id_pos_map);

    q_tanks
        .iter_mut()
        .filter(|(tank, _)| tank.has_target() && !tank.is_cooling_down(time.elapsed_seconds_f64()))
        .for_each(|(mut tank, _)| {
            let source_option = tank_id_pos_map.get(&tank.id);
            let target_option = all_id_pos_map.get(&tank.get_target().unwrap());

            if let (Some(source), Some(target)) = (source_option, target_option) {
                if source.distance(*target) > tank.get_radius() {
                    return;
                }

                tank.start_cooling_down(time.elapsed_seconds_f64());
                spawn_tank_bullet(&mut commands, &asset_server, source.clone(), target.clone());

                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/explosion.ogg"),
                    ..default()
                });
            }
        });
}
