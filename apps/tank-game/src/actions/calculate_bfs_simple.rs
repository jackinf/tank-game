use crate::constants::{GridSize, TileCoord};
use std::collections::{HashMap, HashSet, VecDeque};

// Not used, but I will leave it here for research purposes
fn calculate_bfs_simple(
    grid_size: GridSize,
    start: TileCoord,
    goals: &HashSet<TileCoord>,
    blocking: &HashSet<TileCoord>,
) -> Option<Vec<TileCoord>> {
    if blocking.contains(&start) {
        return None;
    }

    let (width, height) = grid_size;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut predecessors = HashMap::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if goals.contains(&current) {
            let mut path = Vec::new();
            let mut step = current;
            while let Some(&pred) = predecessors.get(&step) {
                path.push(step);
                step = pred;
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        let (x, y) = current;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let next_coord = (nx as usize, ny as usize);
                if !blocking.contains(&next_coord) && !visited.contains(&next_coord) {
                    visited.insert(next_coord);
                    predecessors.insert(next_coord, current);
                    queue.push_back(next_coord);
                }
            }
        }
    }

    None
}
