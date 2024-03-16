use std::collections::{HashSet, VecDeque};
use std::process::exit;
use crate::utils::common::{find_coord, Vec2};
use crate::utils::constants::{FINISH, START};

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
    println!("start: {:?}", start);
    let mut queue: VecDeque<Vec2> = VecDeque::new();
    queue.push_back(start);

    let mut hash: HashSet<(usize, usize)> = HashSet::new();

    while !queue.is_empty() {
        let curr = &queue.pop_front().unwrap();
        if !(0 <= curr.0 && curr.0 < grid.len() && 0 <= curr.1 && curr.1 < grid[0].len()) {
            println!("oob");
            continue;
        }

        if hash.contains(&(curr.0, curr.1)) {
            continue;
        }
        hash.insert((curr.0, curr.1));

        if grid[curr.0][curr.1] == FINISH {
            println!("FOUND IT: [{}, {}]", curr.0, curr.1);
            exit(0);
        }

        queue.push_back(Vec2(curr.0 + 1, curr.1));
        if curr.0 - 1 > 0 {
            queue.push_back(Vec2(curr.0 - 1, curr.1));
        }
        queue.push_back(Vec2(curr.0, curr.1 + 1));
        if curr.1 - 1 > 0 {
            queue.push_back(Vec2(curr.0, curr.1 - 1));
        }
    }
}