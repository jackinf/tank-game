use crate::constants::TileCoord;
use crate::features::tile::GroundTileType;
use std::collections::VecDeque;

pub fn find_first_gold(grid: &Vec<Vec<i32>>, start: TileCoord) -> Option<TileCoord> {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // Up, down, left, right
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    // Start BFS from the starting position
    queue.push_back(start);
    visited[start.0][start.1] = true;

    while let Some((x, y)) = queue.pop_front() {
        // Check if current cell contains gold
        if grid[x][y] == GroundTileType::Gold as i32 {
            return Some((x, y));
        }

        // Explore the four possible directions
        for &(dx, dy) in &directions {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if !visited[nx][ny]
                    && grid[nx][ny] != GroundTileType::Wall as i32
                    && grid[nx][ny] != GroundTileType::Water as i32
                {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    None
}
