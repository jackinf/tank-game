use crate::constants::{GridSize, TileCoord};
use petgraph::Graph;
use std::collections::HashSet;

pub fn calculate_astar_path(
    grid_size: GridSize,
    start: TileCoord,
    goal: TileCoord,
    blocking_cells: &HashSet<TileCoord>,
) -> Vec<TileCoord> {
    let (grid_width, grid_height) = grid_size;
    let mut graph = Graph::<TileCoord, ()>::new();

    // Create a 2D vector to store node indices
    let mut node_indices = vec![vec![None; grid_width]; grid_height];

    // Add nodes to the graph
    for y in 0..grid_height {
        for x in 0..grid_width {
            if !blocking_cells.contains(&(x, y)) || (x, y) == start || (x, y) == goal {
                let node = graph.add_node((x, y));
                node_indices[y][x] = Some(node);
            }
        }
    }

    // Add edges between adjacent nodes
    for y in 0..grid_height {
        for x in 0..grid_width {
            if let Some(node) = node_indices[y][x] {
                // neighbors
                let left = (x.wrapping_sub(1), y);
                let right = (x + 1, y);
                let up = (x, y.wrapping_sub(1));
                let down = (x, y + 1);

                for &(nx, ny) in &[left, right, up, down] {
                    if nx < grid_width && ny < grid_height {
                        if let Some(neighbor_node) = node_indices[ny][nx] {
                            graph.add_edge(node, neighbor_node, ());
                        }
                    }
                }
            }
        }
    }

    if let (Some(start_node), Some(goal_node)) =
        (node_indices[start.1][start.0], node_indices[goal.1][goal.0])
    {
        let result = petgraph::algo::astar(
            &graph,
            start_node,
            |finish| finish == goal_node,
            |_| 1,
            |_| 0,
        );

        let path_coords = result.map(|(_cost, path)| {
            path.into_iter()
                .map(|node| *graph.node_weight(node).unwrap())
                .collect::<Vec<TileCoord>>()
        });

        if let Some(coords) = path_coords {
            return coords;
        } else {
            return vec![];
        }
    } else {
        println!("Invalid start or goal");
        return vec![];
    }
}
