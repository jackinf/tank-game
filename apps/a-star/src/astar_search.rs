use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::utils::common::{find_coord, Vec3};
use crate::utils::constants::{FINISH, START, WALL};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize, // This is the f score: g + h
    position: (usize, usize),
}

// The priority queue relies on `Ord` for ordering. States will be ordered by cost in ascending order.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the order is reversed; we want to pop smallest costs first
        other.cost.cmp(&self.cost)
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
    if pos.0 > 0 { result.push((pos.0 - 1, pos.1)); }
    if pos.1 > 0 { result.push((pos.0, pos.1 - 1)); }
    if pos.0 < bounds.0 - 1 { result.push((pos.0 + 1, pos.1)); }
    if pos.1 < bounds.1 - 1 { result.push((pos.0, pos.1 + 1)); }
    result
}

fn a_star(grid: &Vec<Vec<usize>>, start: (usize, usize), goal: (usize, usize)) -> Option<(usize, HashMap<(usize, usize), (usize, usize)>)> {
    let mut frontier = BinaryHeap::new();
    frontier.push(State { cost: 0, position: start });

    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut cost_so_far: HashMap<(usize, usize), usize> = HashMap::new();

    came_from.insert(start, start);
    cost_so_far.insert(start, 0);

    let bounds = (grid.len(), grid[0].len());

    while let Some(State { cost: _, position: current }) = frontier.pop() {
        if current == goal {
            return Some((*cost_so_far.get(&current).unwrap(), came_from));
        }

        for next in neighbors(current, bounds).iter() {
            if grid[next.0][next.1] == WALL {
                continue;
            }

            let new_cost = cost_so_far[&current] + 1; // Assuming each step costs 1

            if !cost_so_far.contains_key(next) || new_cost < *cost_so_far.get(next).unwrap() {
                cost_so_far.insert(*next, new_cost);
                let priority = new_cost + heuristic(*next, goal);
                frontier.push(State { cost: priority, position: *next });
                came_from.insert(*next, current);
            }
        }
    }

    None // If the goal was not reached
}

pub fn main() {
    let grid: Vec<Vec<usize>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 9, 0],
        vec![0, 0, 0, 0, 1, 0, 0],
        vec![0, 2, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
    ];

    let start_vec = find_coord(&grid, START).unwrap();
    let goal_vec = find_coord(&grid, FINISH).unwrap();

    let start = (start_vec.0, start_vec.1);
    let goal = (goal_vec.0, goal_vec.1);
    match a_star(&grid, start, goal) {
        Some((cost, came_from)) => {
            println!("Cost to reach {:?} from {:?}: {}", goal, start, cost);
            println!("Path: {:?}", came_from);

            // draw a path from goal to start
            let mut current = goal;
            let mut path = HashSet::new();
            while current != start {
                path.insert(current);
                current = came_from[&current];
            }
            path.insert(start);

            // print it
            for (i, row) in grid.iter().enumerate() {
                for (j, _) in row.iter().enumerate() {
                    if path.contains(&(i, j)) {
                        print!("* ");
                    } else {
                        print!("{} ", grid[i][j]);
                    }
                }
                println!();
            }
        },
        None => println!("No path found from {:?} to {:?}", start, goal),
    }
}