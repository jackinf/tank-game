//! The [`GameMap`] resource: terrain, ore, passability and the coordinate
//! conversions between tiles and world space.

use super::{Terrain, Tile};
use crate::config::TILE;
use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};

/// The whole map as a resource.
#[derive(Resource)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub terrain: Vec<Terrain>,
    /// Remaining ore on each tile (only meaningful where terrain == Ore).
    pub ore: Vec<u32>,
    /// Static blocking: solid terrain or footprints of buildings.
    pub blocked: Vec<bool>,
    /// Tiles occupied specifically by a building footprint (a subset of
    /// `blocked`). Used to enforce a gap between adjacent buildings.
    pub built: Vec<bool>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Self {
        let n = width * height;
        Self {
            width,
            height,
            terrain: vec![Terrain::Grass; n],
            ore: vec![0; n],
            blocked: vec![false; n],
            built: vec![false; n],
        }
    }

    #[inline]
    pub fn in_bounds(&self, col: i32, row: i32) -> bool {
        col >= 0 && row >= 0 && (col as usize) < self.width && (row as usize) < self.height
    }

    #[inline]
    pub fn idx(&self, col: i32, row: i32) -> usize {
        row as usize * self.width + col as usize
    }

    pub fn terrain_at(&self, col: i32, row: i32) -> Terrain {
        if !self.in_bounds(col, row) {
            return Terrain::Mountain;
        }
        self.terrain[self.idx(col, row)]
    }

    pub fn set_terrain(&mut self, col: i32, row: i32, t: Terrain) {
        if self.in_bounds(col, row) {
            let i = self.idx(col, row);
            self.terrain[i] = t;
            if t.is_solid() {
                self.blocked[i] = true;
            }
        }
    }

    pub fn ore_at(&self, col: i32, row: i32) -> u32 {
        if !self.in_bounds(col, row) {
            return 0;
        }
        self.ore[self.idx(col, row)]
    }

    pub fn set_ore(&mut self, col: i32, row: i32, amount: u32) {
        if self.in_bounds(col, row) {
            let i = self.idx(col, row);
            self.ore[i] = amount;
            self.terrain[i] = if amount > 0 { Terrain::Ore } else { Terrain::Grass };
        }
    }

    /// Take up to `amount` ore from a tile, returning how much was actually taken.
    pub fn take_ore(&mut self, col: i32, row: i32, amount: u32) -> u32 {
        if !self.in_bounds(col, row) {
            return 0;
        }
        let i = self.idx(col, row);
        let taken = amount.min(self.ore[i]);
        self.ore[i] -= taken;
        if self.ore[i] == 0 && self.terrain[i] == Terrain::Ore {
            self.terrain[i] = Terrain::Grass;
        }
        taken
    }

    pub fn is_blocked(&self, col: i32, row: i32) -> bool {
        if !self.in_bounds(col, row) {
            return true;
        }
        self.blocked[self.idx(col, row)]
    }

    pub fn set_blocked(&mut self, col: i32, row: i32, blocked: bool) {
        if self.in_bounds(col, row) {
            let i = self.idx(col, row);
            // Never unblock solid terrain.
            self.blocked[i] = blocked || self.terrain[i].is_solid();
        }
    }

    pub fn is_passable(&self, col: i32, row: i32) -> bool {
        self.in_bounds(col, row) && !self.is_blocked(col, row)
    }

    /// Is this tile part of a building footprint?
    pub fn is_built(&self, col: i32, row: i32) -> bool {
        self.in_bounds(col, row) && self.built[self.idx(col, row)]
    }

    /// Mark (or clear) a tile as occupied by a building footprint.
    pub fn set_built(&mut self, col: i32, row: i32, built: bool) {
        if self.in_bounds(col, row) {
            let i = self.idx(col, row);
            self.built[i] = built;
        }
    }

    /// Can a building of `footprint` (w, h) sit with its top-left at `origin`
    /// while leaving at least a one-tile gap to every other building? The gap
    /// border may run off the map edge or touch solid terrain — it only has to
    /// stay clear of other building footprints.
    pub fn can_build(&self, origin: Tile, footprint: (i32, i32)) -> bool {
        // Footprint itself must be in-bounds, unblocked and ore-free.
        for dr in 0..footprint.1 {
            for dc in 0..footprint.0 {
                let (c, r) = (origin.0 + dc, origin.1 + dr);
                if !self.in_bounds(c, r) || self.is_blocked(c, r) || self.ore_at(c, r) > 0 {
                    return false;
                }
            }
        }
        // One-tile border must not touch another building.
        for dr in -1..=footprint.1 {
            for dc in -1..=footprint.0 {
                let inside = dc >= 0 && dc < footprint.0 && dr >= 0 && dr < footprint.1;
                if inside {
                    continue;
                }
                if self.is_built(origin.0 + dc, origin.1 + dr) {
                    return false;
                }
            }
        }
        true
    }

    /// World position of the centre of a tile. The map is centred on the origin.
    pub fn tile_center(&self, col: i32, row: i32) -> Vec2 {
        let x = (col as f32 - self.width as f32 / 2.0 + 0.5) * TILE;
        let y = (self.height as f32 / 2.0 - row as f32 - 0.5) * TILE;
        Vec2::new(x, y)
    }

    /// Tile containing a world position.
    pub fn world_to_tile(&self, pos: Vec2) -> Tile {
        let col = (pos.x / TILE + self.width as f32 / 2.0).floor() as i32;
        let row = (self.height as f32 / 2.0 - pos.y / TILE).floor() as i32;
        (col, row)
    }

    pub fn world_bounds(&self) -> (Vec2, Vec2) {
        let half = Vec2::new(self.width as f32 * TILE, self.height as f32 * TILE) * 0.5;
        (-half, half)
    }

    /// Find the nearest passable tile to `target`, breadth-first.
    pub fn nearest_passable(&self, target: Tile) -> Option<Tile> {
        if self.is_passable(target.0, target.1) {
            return Some(target);
        }
        let mut seen: HashSet<Tile> = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(target);
        seen.insert(target);
        let mut steps = 0;
        while let Some((c, r)) = q.pop_front() {
            steps += 1;
            if steps > 4096 {
                break;
            }
            for (dc, dr) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (c + dc, r + dr);
                if self.in_bounds(n.0, n.1) && seen.insert(n) {
                    if self.is_passable(n.0, n.1) {
                        return Some(n);
                    }
                    q.push_back(n);
                }
            }
        }
        None
    }
}
