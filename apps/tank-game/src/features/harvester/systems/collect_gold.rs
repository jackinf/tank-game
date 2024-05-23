use crate::features::harvester::components::Harvester;
use crate::features::tile::Gold;
use crate::resources::game_map::GameMap;
use bevy::math::Vec2;
use bevy::prelude::{Query, Res, Time, Transform, Vec3Swizzles, With};

pub fn collect_gold(
    time: Res<Time>,
    mut q_harvesters: Query<(&mut Harvester, &Transform), With<Harvester>>,
    mut q_gold: Query<&Gold>,
    game_map: Res<GameMap>,
) {
    let timestamp = time.elapsed_seconds_f64();

    q_harvesters
        .iter_mut()
        .filter(|(harvester, _)| harvester.is_harvesting())
        .filter(|(harvester, _)| harvester.is_full() == false)
        .filter(|(harvester, _)| harvester.is_cooling_down_to_harvest(timestamp) == false)
        .for_each(|(mut harvester, transform)| {
            // TODO: check if harvester is close enough to gold
            let harvester_pos = transform.translation.xy();
            let gold_res = q_gold.iter_mut().find(|gold| {
                let gold_pos = game_map.get_tile_to_world_coordinates().get(&gold.at());
                if gold_pos.is_none() {
                    return false;
                }
                let (x, y) = gold_pos.unwrap();
                let gold_pos = Vec2::new(*x, *y);
                let distance = (harvester_pos - gold_pos).length();
                distance < 10.0
            });

            match gold_res {
                None => {
                    // search for gold again
                    harvester.set_searching_for_gold();
                }
                Some(_) => {
                    harvester.collect_gold(10, timestamp);
                    // println!("Harvester {} collected 10 gold!", harvester.get_id());

                    if harvester.is_full() {
                        harvester.set_find_base_to_return();
                    }

                    // TODO: decrease gold value somehow
                    // let mut res = q_gold.get_mut(gold_id).unwrap();
                    // let mut gold: &mut Gold = res.1;
                    // gold.reduce_value_by(10);
                }
            }
        });
}
