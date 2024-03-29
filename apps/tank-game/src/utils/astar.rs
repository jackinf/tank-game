use crate::common::constants::TILE_WALL;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize, // This is the f score: g + h
    position: (usize, usize),
}

// The priority queue relies on `Ord` for ordering. States will be ordered by cost in ascending order.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the order is reversed; we want to pop smallest costs first
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Heuristic: Manhattan distance for a 2D grid, using usize
fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    let dx = (a.0 as i32 - b.0 as i32).abs() as usize;
    let dy = (a.1 as i32 - b.1 as i32).abs() as usize;
    dx + dy
}

// Generate neighbors considering grid bounds and using usize
fn neighbors(pos: (usize, usize), bounds: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if pos.0 > 0 {
        result.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        result.push((pos.0, pos.1 - 1));
    }
    if pos.0 < bounds.0 - 1 {
        result.push((pos.0 + 1, pos.1));
    }
    if pos.1 < bounds.1 - 1 {
        result.push((pos.0, pos.1 + 1));
    }
    result
}

pub fn a_star(
    grid: &Vec<Vec<usize>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<(usize, HashMap<(usize, usize), (usize, usize)>)> {
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        position: start,
    });

    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut cost_so_far: HashMap<(usize, usize), usize> = HashMap::new();

    came_from.insert(start, start);
    cost_so_far.insert(start, 0);

    let bounds = (grid.len(), grid[0].len());

    while let Some(State {
        cost: _,
        position: current,
    }) = frontier.pop()
    {
        if current == goal {
            return Some((*cost_so_far.get(&current).unwrap(), came_from));
        }

        for next in neighbors(current, bounds).iter() {
            if grid[next.0][next.1] == TILE_WALL {
                continue;
            }

            let new_cost = cost_so_far[&current] + 1; // Assuming each step costs 1

            if !cost_so_far.contains_key(next) || new_cost < *cost_so_far.get(next).unwrap() {
                cost_so_far.insert(*next, new_cost);
                let priority = new_cost + heuristic(*next, goal);
                frontier.push(State {
                    cost: priority,
                    position: *next,
                });
                came_from.insert(*next, current);
            }
        }
    }

    None // If the goal was not reached
}
