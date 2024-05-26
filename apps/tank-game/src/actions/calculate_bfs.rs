use crate::constants::{GridSize, TileCoord};
use bevy::utils::petgraph::prelude::NodeIndex;
use bevy::utils::petgraph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn calculate_bfs(
    grid_size: GridSize,
    start: TileCoord,
    goals: &HashSet<TileCoord>,
    blocking: &HashSet<TileCoord>,
) -> Option<Vec<TileCoord>> {
    let (graph, nodes) = build_graph(grid_size, start, goals, &blocking);

    bfs_closest_path(&graph, start, &goals, &nodes)
}

fn build_graph(
    grid_size: GridSize,
    start: TileCoord,
    goals: &HashSet<TileCoord>,
    blocking_cells: &HashSet<TileCoord>,
) -> (Graph<(), ()>, HashMap<TileCoord, NodeIndex>) {
    let mut graph = Graph::<(), ()>::new();

    let (width, height) = grid_size;

    let mut nodes = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            if !(blocking_cells.contains(&(x, y)) && (x, y) != start) {
                let node = graph.add_node(());
                nodes.insert((x, y), node);
            }
        }
    }

    for (&(x, y), &node) in &nodes {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let next_coord = (nx as usize, ny as usize);
                if !blocking_cells.contains(&next_coord) {
                    if let Some(&neighbor) = nodes.get(&next_coord) {
                        graph.add_edge(node, neighbor, ());
                    }
                }
            }
        }
    }

    (graph, nodes)
}

fn bfs_closest_path(
    graph: &Graph<(), ()>,
    start: TileCoord,
    goals: &HashSet<TileCoord>,
    nodes: &HashMap<TileCoord, NodeIndex>,
) -> Option<Vec<TileCoord>> {
    let start_node = *nodes.get(&start).unwrap();
    let goal_nodes: HashSet<NodeIndex> = goals
        .iter()
        .filter_map(|goal| nodes.get(goal))
        .copied()
        .collect();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut predecessors = HashMap::new();

    visited.insert(start_node);
    queue.push_back(start_node);

    while let Some(node) = queue.pop_front() {
        if goal_nodes.contains(&node) {
            let mut path = Vec::new();
            let mut current = node;
            while let Some(&pred) = predecessors.get(&current) {
                path.push(current);
                current = pred;
            }
            path.push(start_node);
            path.reverse();
            return Some(
                path.into_iter()
                    .map(|n| *nodes.iter().find(|&(_, &v)| v == n).unwrap().0)
                    .collect(),
            );
        }

        for neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                predecessors.insert(neighbor, node);
                queue.push_back(neighbor);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bfs_case1() {
        /*
           sx 0  g
           0  x  0
           0  0  g
        */

        let start: TileCoord = (0, 0);
        let goals: HashSet<TileCoord> = vec![(2, 2), (0, 2)].into_iter().collect();
        let grid_size: GridSize = (3, 3);
        let blocking: HashSet<TileCoord> = vec![(1, 1), (0, 0)].into_iter().collect();

        let path = calculate_bfs(grid_size, start, &goals, &blocking).unwrap();
        assert_eq!(path, vec![(0, 0), (0, 1), (0, 2)]);
    }

    #[test]
    fn test_calculate_bfs_case2() {
        /*
           sx x  g
           0  x  0
           0  0  g
        */

        let start: TileCoord = (0, 0);
        let goals: HashSet<TileCoord> = vec![(2, 2), (0, 2)].into_iter().collect();
        let grid_size: GridSize = (3, 3);
        let blocking: HashSet<TileCoord> = vec![(1, 1), (0, 1), (0, 0)].into_iter().collect();

        let path = calculate_bfs(grid_size, start, &goals, &blocking).unwrap();
        assert_eq!(path, vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]);
    }

    #[test]
    fn test_calculate_bfs_case3() {
        /*
           g  0  g
           0  x  0
           0  0  s
        */

        let start: TileCoord = (2, 2);
        let goals: HashSet<TileCoord> = vec![(0, 0), (0, 2)].into_iter().collect();
        let grid_size: GridSize = (3, 3);
        let blocking: HashSet<TileCoord> = vec![(1, 1)].into_iter().collect();

        let path = calculate_bfs(grid_size, start, &goals, &blocking).unwrap();
        assert_eq!(path, vec![(2, 2), (1, 2), (0, 2)]);
    }
}
