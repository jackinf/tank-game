use std::collections::{BinaryHeap, HashSet};
use crate::utils::common::{find_coord, Vec3};
use crate::utils::constants::{FINISH, START, WALL};

/// Finds a value using MinMax Heap
pub fn main() {
    let grid: Vec<Vec<usize>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 9, 0],
        vec![0, 0, 0, 0, 1, 0, 0],
        vec![0, 2, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
    ];

    let start = find_coord(&grid, START).unwrap();
    let mut queue: BinaryHeap<Vec3> = BinaryHeap::new();
    queue.push(Vec3(0, start.0, start.1));

    let mut cache: HashSet<(usize, usize)> = HashSet::new();
    let mut scanned = 0;

    while !queue.is_empty() {
        let curr = &queue.pop().unwrap();
        let depth = curr.0;
        let row = curr.1;
        let col = curr.2;
        scanned += 1;

        if !(row < grid.len() && col < grid[0].len()) {
            continue;
        }

        if cache.contains(&(row, col)) {
            continue;
        }
        cache.insert((row, col));

        if grid[row][col] == WALL {
            continue
        }

        if grid[row][col] == FINISH {
            println!("FOUND IT: [{}, {}]. Scanned: {}", row, col, scanned);
            return;
        }

        let new_depth = depth + 1;
        queue.push(Vec3(new_depth, row + 1, col));
        if row - 1 > 0 {
            queue.push(Vec3(new_depth, row - 1, col));
        }
        queue.push(Vec3(new_depth, row, col + 1));
        if col - 1 > 0 {
            queue.push(Vec3(new_depth, row, col - 1));
        }
    }
}
