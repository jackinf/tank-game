use crate::utils::common::{find_coord, Vec2};
use crate::utils::constants::{FINISH, START, WALL};
use std::collections::{HashSet, VecDeque};

pub fn main() {
    println!("Breadth-first search");

    let grid: Vec<Vec<usize>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 9, 0],
        vec![0, 0, 0, 0, 1, 0, 0],
        vec![0, 2, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
    ];

    let start = find_coord(&grid, START).unwrap();
    let mut queue: VecDeque<Vec2> = VecDeque::new();
    queue.push_back(start);

    let mut cache: HashSet<(usize, usize)> = HashSet::new();
    let mut scanned = 0;

    while !queue.is_empty() {
        let curr = &queue.pop_front().unwrap();
        let row = curr.0;
        let col = curr.1;

        scanned += 1;
        if !(row < grid.len() && col < grid[0].len()) {
            continue;
        }

        if cache.contains(&(row, col)) {
            continue;
        }
        cache.insert((row, col));

        if grid[row][col] == WALL {
            continue;
        }

        if grid[row][col] == FINISH {
            println!("FOUND IT: [{}, {}]. Scanned: {}", row, curr.1, scanned);
            return;
        }

        queue.push_back(Vec2(row + 1, curr.1));
        if row - 1 > 0 {
            queue.push_back(Vec2(row - 1, curr.1));
        }
        queue.push_back(Vec2(row, curr.1 + 1));
        if curr.1 - 1 > 0 {
            queue.push_back(Vec2(row, curr.1 - 1));
        }
    }
}
