//! The tile map: terrain, ore, and passability, plus coordinate conversions
//! and A* pathfinding.

use crate::config::TILE;
use bevy::prelude::*;
use std::collections::{BinaryHeap, HashMap, VecDeque};

/// The kind of terrain on a tile.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Terrain {
    Grass,
    Water,
    Mountain,
    /// Ore-bearing ground. Harvestable; the amount lives in `GameMap::ore`.
    Ore,
}

impl Terrain {
    /// Base (no-ore) colour for rendering.
    pub fn color(self) -> Color {
        match self {
            Terrain::Grass => Color::srgb(0.22, 0.42, 0.18),
            Terrain::Water => Color::srgb(0.12, 0.28, 0.55),
            Terrain::Mountain => Color::srgb(0.32, 0.30, 0.28),
            Terrain::Ore => Color::srgb(0.22, 0.42, 0.18),
        }
    }

    /// Terrain that units/buildings can never occupy.
    pub fn is_solid(self) -> bool {
        matches!(self, Terrain::Water | Terrain::Mountain)
    }
}

/// A tile coordinate (column, row), with row 0 at the top of the map.
pub type Tile = (i32, i32);

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
        let mut seen: HashMap<Tile, bool> = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back(target);
        seen.insert(target, true);
        let mut steps = 0;
        while let Some((c, r)) = q.pop_front() {
            steps += 1;
            if steps > 4096 {
                break;
            }
            for (dc, dr) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (c + dc, r + dr);
                if self.in_bounds(n.0, n.1) && !seen.contains_key(&n) {
                    seen.insert(n, true);
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

// ---------------------------------------------------------------------------
// A* pathfinding
// ---------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq)]
struct Node {
    f: f32,
    tile: Tile,
}
impl Eq for Node {}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse so BinaryHeap behaves as a min-heap on f.
        other.f.partial_cmp(&self.f).unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn octile(a: Tile, b: Tile) -> f32 {
    let dx = (a.0 - b.0).abs() as f32;
    let dy = (a.1 - b.1).abs() as f32;
    let (min, max) = if dx < dy { (dx, dy) } else { (dy, dx) };
    max + (std::f32::consts::SQRT_2 - 1.0) * min
}

/// A* over the passable tiles, 8-directional, no corner cutting.
/// Returns a list of tiles from start (exclusive) to goal (inclusive).
pub fn find_path(map: &GameMap, start: Tile, goal: Tile) -> Option<Vec<Tile>> {
    let goal = if map.is_passable(goal.0, goal.1) {
        goal
    } else {
        map.nearest_passable(goal)?
    };
    if start == goal {
        return Some(vec![]);
    }

    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<Tile, Tile> = HashMap::new();
    let mut g_score: HashMap<Tile, f32> = HashMap::new();

    g_score.insert(start, 0.0);
    open.push(Node { f: octile(start, goal), tile: start });

    let mut iterations = 0;
    while let Some(Node { tile: current, .. }) = open.pop() {
        iterations += 1;
        if iterations > 20_000 {
            break;
        }
        if current == goal {
            // Reconstruct path.
            let mut path = vec![current];
            let mut c = current;
            while let Some(&p) = came_from.get(&c) {
                if p == start {
                    break;
                }
                path.push(p);
                c = p;
            }
            path.reverse();
            return Some(path);
        }

        let cur_g = *g_score.get(&current).unwrap_or(&f32::INFINITY);
        for (dc, dr) in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            let n = (current.0 + dc, current.1 + dr);
            if !map.is_passable(n.0, n.1) {
                continue;
            }
            // Disallow cutting corners diagonally.
            if dc != 0 && dr != 0 {
                if !map.is_passable(current.0 + dc, current.1) || !map.is_passable(current.0, current.1 + dr) {
                    continue;
                }
            }
            let step = if dc != 0 && dr != 0 { std::f32::consts::SQRT_2 } else { 1.0 };
            let tentative = cur_g + step;
            if tentative < *g_score.get(&n).unwrap_or(&f32::INFINITY) {
                came_from.insert(n, current);
                g_score.insert(n, tentative);
                open.push(Node { f: tentative + octile(n, goal), tile: n });
            }
        }
    }
    None
}
