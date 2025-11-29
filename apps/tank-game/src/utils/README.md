# Utils

Utility functions and helpers used across the game.

## Modules

### `astar.rs` - A* Pathfinding

Implementation of the A* pathfinding algorithm for unit movement.

```rust
use crate::utils::astar::find_path;

let path = find_path(&grid, (start_x, start_y), (goal_x, goal_y));
// Returns VecDeque<(usize, usize)> of tile coordinates
```

Features:
- Manhattan distance heuristic
- Respects wall and water tiles as obstacles
- Returns empty path if no route exists

### `common_helpers.rs` - Common Helpers

General utility functions:

- `calculate_random_layer()` - Generate random Z-layer for sprites
- Other helper functions for common operations

### `logger.rs` - Logging

Logging utilities for debug output.

## Design Notes

- Utils should be **stateless** functions
- No Bevy dependencies (pure Rust)
- Can be unit tested independently
- Shared across all features

