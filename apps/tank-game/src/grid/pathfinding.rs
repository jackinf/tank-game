//! A* pathfinding over the passable tiles: 8-directional, no corner cutting.

use super::{GameMap, Tile};
use std::collections::{BinaryHeap, HashMap};

/// An open-set entry, ordered by its f-score for the priority queue.
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

/// Octile distance heuristic for an 8-connected grid.
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
            return Some(reconstruct(&came_from, start, current));
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
            let diagonal = dc != 0 && dr != 0;
            if diagonal
                && (!map.is_passable(current.0 + dc, current.1)
                    || !map.is_passable(current.0, current.1 + dr))
            {
                continue;
            }
            let step = if diagonal { std::f32::consts::SQRT_2 } else { 1.0 };
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

/// Walk the `came_from` chain back from `goal` to (but excluding) `start`.
fn reconstruct(came_from: &HashMap<Tile, Tile>, start: Tile, goal: Tile) -> Vec<Tile> {
    let mut path = vec![goal];
    let mut c = goal;
    while let Some(&p) = came_from.get(&c) {
        if p == start {
            break;
        }
        path.push(p);
        c = p;
    }
    path.reverse();
    path
}
