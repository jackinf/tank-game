//! Headless logic tests for the core simulation (no rendering required).

use crate::defs::*;
use crate::economy::PlayerEconomy;
use crate::faction::Faction;
use crate::grid::{find_path, GameMap, Terrain};
use crate::maps::{all_maps, load_map};

#[test]
fn tile_world_roundtrip() {
    let map = GameMap::new(20, 14);
    for row in 0..14 {
        for col in 0..20 {
            let world = map.tile_center(col, row);
            assert_eq!(map.world_to_tile(world), (col, row));
        }
    }
}

#[test]
fn ore_take_depletes_and_clears() {
    let mut map = GameMap::new(8, 8);
    map.set_ore(2, 2, 30);
    assert_eq!(map.terrain_at(2, 2), Terrain::Ore);
    assert_eq!(map.take_ore(2, 2, 10), 10);
    assert_eq!(map.ore_at(2, 2), 20);
    assert_eq!(map.take_ore(2, 2, 100), 20);
    assert_eq!(map.ore_at(2, 2), 0);
    assert_eq!(map.terrain_at(2, 2), Terrain::Grass);
}

#[test]
fn pathfinding_finds_route_around_walls() {
    let mut map = GameMap::new(12, 5);
    // Vertical wall with a single gap at the bottom.
    for row in 0..4 {
        map.set_terrain(6, row, Terrain::Mountain);
    }
    let path = find_path(&map, (1, 2), (10, 2)).expect("a path should exist");
    assert!(!path.is_empty());
    assert_eq!(*path.last().unwrap(), (10, 2));
    // Every step must be passable.
    for &(c, r) in &path {
        assert!(map.is_passable(c, r), "stepped onto a blocked tile {:?}", (c, r));
    }
}

#[test]
fn pathfinding_blocked_goal_routes_to_nearest() {
    let mut map = GameMap::new(8, 8);
    map.set_terrain(4, 4, Terrain::Water);
    // Goal tile is solid; path should still succeed to an adjacent tile.
    let path = find_path(&map, (0, 0), (4, 4));
    assert!(path.is_some());
}

#[test]
fn maps_load_with_distinct_starts() {
    for (_name, ascii) in all_maps() {
        let data = load_map(ascii);
        assert!(data.map.width > 10 && data.map.height > 10);
        assert_ne!(data.player_start, data.enemy_start);
        // Some ore should exist on every map.
        let total_ore: u32 = data.map.ore.iter().sum();
        assert!(total_ore > 0, "map has no ore");
    }
}

#[test]
fn tech_tree_prerequisites() {
    assert_eq!(BuildingKind::PowerPlant.prerequisite(), Some(BuildingKind::ConstructionYard));
    assert_eq!(BuildingKind::WarFactory.prerequisite(), Some(BuildingKind::Refinery));
    assert_eq!(UnitKind::BASIC_TANK.produced_by(), BuildingKind::WarFactory);
    assert_eq!(UnitKind::BASIC_SOLDIER.produced_by(), BuildingKind::Barracks);
    assert_eq!(UnitKind::Harvester.produced_by(), BuildingKind::WarFactory);
}

#[test]
fn armor_damage_table() {
    use crate::defs::{damage_multiplier, ArmorKind, Role};
    // Small arms shred infantry but glance off armour and buildings.
    assert!(damage_multiplier(Role::AntiInfantry, ArmorKind::Infantry) > 0.9);
    assert!(damage_multiplier(Role::AntiInfantry, ArmorKind::Vehicle) < 0.4);
    assert!(damage_multiplier(Role::AntiInfantry, ArmorKind::Building) < 0.4);
    // AT rounds wreck vehicles, weak vs infantry.
    assert!(damage_multiplier(Role::AntiTank, ArmorKind::Vehicle) > 0.9);
    assert!(damage_multiplier(Role::AntiTank, ArmorKind::Infantry) < 0.5);
    // Demolition warheads flatten buildings.
    assert!(damage_multiplier(Role::AntiBuilding, ArmorKind::Building) > 1.0);
}

#[test]
fn unit_matrix_is_complete() {
    assert_eq!(UnitKind::infantry().len(), 9);
    assert_eq!(UnitKind::vehicles().len(), 10); // 9 tanks + harvester
    // Heavier armour is tougher and slower.
    assert!(UnitKind::HEAVY_TANK.max_health() > UnitKind::LIGHT_TANK.max_health());
    assert!(UnitKind::HEAVY_TANK.speed() < UnitKind::LIGHT_TANK.speed());
}

#[test]
fn factions_are_hostile_correctly() {
    assert!(Faction::Player.is_hostile_to(Faction::Enemy));
    assert!(Faction::Enemy.is_hostile_to(Faction::Player));
    assert!(!Faction::Player.is_hostile_to(Faction::Player));
    assert!(!Faction::Player.is_hostile_to(Faction::Neutral));
}

#[test]
fn power_factor_scales_when_low() {
    let mut eco = PlayerEconomy::default();
    eco.power_produced = 100;
    eco.power_consumed = 50;
    assert!(eco.has_power());
    assert_eq!(eco.power_factor(), 1.0);

    eco.power_produced = 50;
    eco.power_consumed = 100;
    assert!(!eco.has_power());
    assert!(eco.power_factor() < 1.0 && eco.power_factor() >= 0.25);
}

#[test]
fn buildings_need_a_gap_between_them() {
    use std::collections::HashSet;
    let mut map = GameMap::new(20, 20);
    let footprint = (2, 2);
    // Place a building at (5,5)-(6,6) and mark its footprint as built.
    for dr in 0..2 {
        for dc in 0..2 {
            map.set_blocked(5 + dc, 5 + dr, true);
            map.set_built(5 + dc, 5 + dr, true);
        }
    }
    // Directly adjacent placements (touching) are rejected...
    assert!(!map.can_build((3, 5), footprint), "left-adjacent should be blocked");
    assert!(!map.can_build((7, 5), footprint), "right-adjacent should be blocked");
    assert!(!map.can_build((5, 3), footprint), "top-adjacent should be blocked");
    assert!(!map.can_build((3, 3), footprint), "corner-touch should be blocked");
    // ...but a placement that leaves a one-tile gap is allowed.
    assert!(map.can_build((8, 5), footprint), "one-tile gap should be allowed");

    // Sanity: the built tiles are a subset of blocked tiles.
    let built: HashSet<_> = (5..7).flat_map(|c| (5..7).map(move |r| (c, r))).collect();
    for (c, r) in built {
        assert!(map.is_built(c, r) && map.is_blocked(c, r));
    }
}

#[test]
fn unit_names_are_distinct_and_characterful() {
    use std::collections::HashSet;
    let mut names = HashSet::new();
    let mut all = UnitKind::infantry();
    all.extend(UnitKind::vehicles());
    for kind in all {
        let name = kind.name();
        assert!(!name.is_empty());
        assert!(names.insert(name.clone()), "duplicate unit name: {name}");
        assert!(!kind.description().is_empty());
    }
    // A couple of the requested characterful names exist.
    assert_eq!(UnitKind::HEAVY_TANK.produced_by(), BuildingKind::WarFactory);
    assert!(names.contains("Bazooka Joe"));
    assert!(names.contains("Heavy Tank"));
}

#[test]
fn building_footprints_are_positive() {
    for kind in BuildingKind::ALL {
        let (w, h) = kind.footprint();
        assert!(w > 0 && h > 0);
        assert!(kind.cost() > 0);
        assert!(kind.max_health() > 0.0);
    }
}
