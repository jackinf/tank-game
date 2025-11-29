# Actions

Pure functions that perform game logic without Bevy system dependencies.

## Purpose

Actions are **pure functions** that:
- Take input data and return results
- Have no side effects
- Don't interact with Bevy's ECS directly
- Are easily testable

## Available Actions

| Action | Description |
|--------|-------------|
| `calculate_astar_path` | A* pathfinding between two points |
| `calculate_bfs` | Breadth-first search for tile finding |
| `calculate_bfs_simple` | Simplified BFS implementation |
| `calculate_tile_world_position` | Convert tile coords to world coords |
| `find_first_gold` | Find nearest gold resource |
| `get_all_blocking_cells` | Get cells blocked by buildings/units |
| `load_mission` | Parse Tiled JSON mission files |
| `read_main_assets` | Parse Tiled tileset definitions |

## Usage

Actions are called from systems when game logic needs to be performed:

```rust
use crate::actions::calculate_astar_path::find_path;

fn my_system(/* ... */) {
    let path = find_path(&grid, start, goal);
    // Use the path...
}
```

## Design Philosophy

By separating pure logic from Bevy systems:
- Logic can be unit tested without mocking Bevy
- Code is more portable and reusable
- Systems remain thin orchestration layers

