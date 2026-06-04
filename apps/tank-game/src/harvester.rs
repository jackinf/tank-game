//! Harvester logic: seek ore, mine it, return to a refinery, and deposit for
//! credits.

use crate::components::*;
use crate::config::*;
use crate::economy::Economy;
use crate::faction::Faction;
use crate::grid::{GameMap, Tile};
use crate::state::GameState;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum HarvestState {
    /// Looking for an ore tile to head to.
    Seeking,
    /// Driving to a known ore tile.
    MovingToOre(Tile),
    /// Sitting on an ore tile, mining.
    Mining(Tile),
    /// Full; driving back to the nearest refinery.
    Returning,
    /// Player took manual control; do nothing automatic.
    Manual,
}

#[derive(Component)]
pub struct Harvester {
    pub cargo: u32,
    pub capacity: u32,
    pub state: HarvestState,
    pub mine_timer: f32,
}

impl Default for Harvester {
    fn default() -> Self {
        Self {
            cargo: 0,
            capacity: 100,
            state: HarvestState::Seeking,
            mine_timer: 0.0,
        }
    }
}

pub struct HarvesterPlugin;

impl Plugin for HarvesterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, harvester_ai.run_if(in_state(GameState::Playing)));
    }
}

/// Find the nearest ore tile to a world position.
fn nearest_ore(map: &GameMap, from: Vec2) -> Option<Tile> {
    let origin = map.world_to_tile(from);
    let mut best: Option<(Tile, i32)> = None;
    for row in 0..map.height as i32 {
        for col in 0..map.width as i32 {
            if map.ore_at(col, row) > 0 {
                let d = (col - origin.0).pow(2) + (row - origin.1).pow(2);
                if best.map_or(true, |(_, bd)| d < bd) {
                    best = Some(((col, row), d));
                }
            }
        }
    }
    best.map(|(t, _)| t)
}

/// Find the nearest friendly refinery, returning its world position.
fn nearest_refinery(
    from: Vec2,
    faction: Faction,
    refineries: &Query<(&Building, &Faction, &Transform)>,
) -> Option<Vec2> {
    let mut best: Option<(Vec2, f32)> = None;
    for (building, bf, tf) in refineries.iter() {
        if *bf == faction && building.kind == crate::defs::BuildingKind::Refinery {
            let p = tf.translation.truncate();
            let d = p.distance_squared(from);
            if best.map_or(true, |(_, bd)| d < bd) {
                best = Some((p, d));
            }
        }
    }
    best.map(|(p, _)| p)
}

#[allow(clippy::type_complexity)]
fn harvester_ai(
    time: Res<Time>,
    mut map: ResMut<GameMap>,
    mut economy: ResMut<Economy>,
    mut harvesters: Query<
        (&mut Harvester, &mut Mover, &mut Order, &Transform, &Faction),
        With<Unit>,
    >,
    refineries: Query<(&Building, &Faction, &Transform)>,
) {
    let dt = time.delta_secs();

    for (mut harv, mut mover, mut command, transform, faction) in &mut harvesters {
        let pos = transform.translation.truncate();

        // If the player issued a manual order, stop auto-harvesting until idle again.
        match *command {
            Order::Move(_) | Order::AttackMove(_) | Order::Attack(_) => {
                harv.state = HarvestState::Manual;
                // Once the manual move finishes, resume automatic harvesting.
                if !mover.is_moving() {
                    *command = Order::Idle;
                }
                continue;
            }
            Order::Idle => {
                if harv.state == HarvestState::Manual {
                    harv.state = if harv.cargo >= harv.capacity {
                        HarvestState::Returning
                    } else {
                        HarvestState::Seeking
                    };
                }
            }
        }

        match harv.state {
            HarvestState::Seeking => {
                if harv.cargo >= harv.capacity {
                    harv.state = HarvestState::Returning;
                } else if let Some(ore_tile) = nearest_ore(&map, pos) {
                    set_path(&map, &mut mover, pos, map.tile_center(ore_tile.0, ore_tile.1));
                    harv.state = HarvestState::MovingToOre(ore_tile);
                }
            }
            HarvestState::MovingToOre(tile) => {
                if map.ore_at(tile.0, tile.1) == 0 {
                    harv.state = HarvestState::Seeking;
                    mover.stop();
                } else if !mover.is_moving() {
                    harv.state = HarvestState::Mining(tile);
                }
            }
            HarvestState::Mining(tile) => {
                harv.mine_timer -= dt;
                if harv.mine_timer <= 0.0 {
                    harv.mine_timer = 0.4;
                    let want = 10.min(harv.capacity - harv.cargo);
                    let got = map.take_ore(tile.0, tile.1, want);
                    harv.cargo += got;
                    if got == 0 || harv.cargo >= harv.capacity {
                        harv.state = if harv.cargo > 0 {
                            HarvestState::Returning
                        } else {
                            HarvestState::Seeking
                        };
                    }
                }
            }
            HarvestState::Returning => {
                if let Some(refinery) = nearest_refinery(pos, *faction, &refineries) {
                    if pos.distance(refinery) < TILE * 1.6 {
                        // Deposit.
                        let gained = harv.cargo as i64 * CREDITS_PER_ORE;
                        economy.get_mut(*faction).credits += gained;
                        harv.cargo = 0;
                        harv.state = HarvestState::Seeking;
                        mover.stop();
                    } else if !mover.is_moving() {
                        set_path(&map, &mut mover, pos, refinery);
                    }
                } else {
                    // No refinery; just idle in place.
                    harv.state = HarvestState::Seeking;
                }
            }
            HarvestState::Manual => {}
        }
    }
}

fn set_path(map: &GameMap, mover: &mut Mover, from: Vec2, to: Vec2) {
    let start = map.world_to_tile(from);
    let goal = map.world_to_tile(to);
    if let Some(path) = crate::grid::find_path(map, start, goal) {
        mover.path = path
            .into_iter()
            .map(|(c, r)| map.tile_center(c, r))
            .collect();
        // Make the final waypoint the exact destination.
        if let Some(last) = mover.path.back_mut() {
            *last = to;
        }
    }
}
