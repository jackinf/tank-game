//! Builds a fresh match: loads a map, spawns terrain and both bases, and
//! switches to the playing state. Also cleans up on restart.

use crate::ai::AiState;
use crate::config::TILE;
use crate::defs::*;
use crate::economy::Economy;
use crate::faction::Faction;
use crate::grid::{GameMap, Tile};
use crate::maps::{all_maps, load_map, MapData};
use crate::spawn::{find_spot, spawn_building, spawn_unit};
use crate::state::{GameEntity, GameState};
use crate::terrain::spawn_terrain;
use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), (cleanup_match, setup_match).chain());
    }
}

fn cleanup_match(mut commands: Commands, entities: Query<Entity, With<GameEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn setup_match(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next: ResMut<NextState<GameState>>,
    mut ai: ResMut<AiState>,
) {
    let maps = all_maps();
    let MapData {
        mut map,
        player_start,
        enemy_start,
    } = load_map(maps[0].1);

    spawn_terrain(&mut commands, &asset_server, &map);

    build_base(&mut commands, &mut map, Faction::Player, player_start, false);
    build_base(&mut commands, &mut map, Faction::Enemy, enemy_start, true);

    ai.base_tile = enemy_start;
    ai.base_pos = map.tile_center(enemy_start.0, enemy_start.1);
    ai.decision_timer = 0.0;
    ai.attack_timer = 40.0;

    // Reset per-match resources.
    commands.insert_resource(Economy::default());
    commands.insert_resource(map);

    next.set(GameState::Playing);
}

fn build_base(
    commands: &mut Commands,
    map: &mut GameMap,
    faction: Faction,
    start: Tile,
    aggressive: bool,
) {
    // Spread the starting buildings out around the Construction Yard, fanning
    // toward the centre of the map so each base feels like a real settlement
    // rather than a solid block. `find_spot` enforces a one-tile gap, so these
    // anchors only need to be roughly right.
    let sx = if (start.0 as usize) < map.width / 2 { 1 } else { -1 };
    let sy = if (start.1 as usize) < map.height / 2 { 1 } else { -1 };
    let anchor = |dc: i32, dr: i32| (start.0 + dc * sx, start.1 + dr * sy);

    // Every base starts fully built: one of each structure, fanned out from the
    // Construction Yard toward the centre of the map. `find_spot` enforces a
    // one-tile gap, so these anchors only need to be roughly right.
    place(commands, map, BuildingKind::ConstructionYard, faction, anchor(0, 0));
    place(commands, map, BuildingKind::PowerPlant, faction, anchor(5, 0));
    place(commands, map, BuildingKind::WarFactory, faction, anchor(9, 0));
    place(commands, map, BuildingKind::Refinery, faction, anchor(0, 5));
    place(commands, map, BuildingKind::Barracks, faction, anchor(5, 5));
    place(commands, map, BuildingKind::PowerPlant, faction, anchor(9, 5));
    place(commands, map, BuildingKind::AntiInfantryTurret, faction, anchor(3, 9));
    place(commands, map, BuildingKind::AntiTankTurret, faction, anchor(8, 9));

    let base = map.tile_center(start.0, start.1);
    spawn_unit(commands, UnitKind::Harvester, faction, base + Vec2::new(TILE * 2.0, -TILE * 2.0));
    spawn_unit(commands, UnitKind::BASIC_SOLDIER, faction, base + Vec2::new(-TILE, -TILE * 3.0));
    spawn_unit(commands, UnitKind::AT_SOLDIER, faction, base + Vec2::new(TILE, -TILE * 3.0));
    if aggressive {
        spawn_unit(commands, UnitKind::BASIC_TANK, faction, base + Vec2::new(0.0, -TILE * 4.0));
        spawn_unit(commands, UnitKind::BASIC_SOLDIER, faction, base + Vec2::new(TILE * 2.0, -TILE * 3.0));
    }
}

fn place(commands: &mut Commands, map: &mut GameMap, kind: BuildingKind, faction: Faction, near: Tile) {
    if let Some(origin) = find_spot(map, near, kind.footprint()) {
        spawn_building(commands, map, kind, faction, origin);
    }
}
