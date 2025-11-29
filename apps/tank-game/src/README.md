# Tank Game - Source Code

This is the main source directory for the Tank Game, an RTS game inspired by Red Alert 1.

## Directory Structure

```
src/
├── main.rs                 # Application entry point, app setup, and asset loading
├── constants.rs            # Game constants (tile size, colors, speeds)
├── features/               # Feature-based plugin modules
├── actions/                # Pure functions for game logic
├── components/             # Shared ECS components
├── resources/              # Shared Bevy resources
├── systems/                # Core game systems (setup, mission loading)
├── types/                  # Type definitions and data structures
└── utils/                  # Utility functions (A*, helpers)
```

## Architecture

The game follows Bevy's recommended **plugin-based architecture**:

- Each feature (tank, building, harvester, etc.) is encapsulated in its own plugin
- Plugins register their systems, events, and resources
- Features are self-contained with their own components, systems, and actions

## App States

```rust
enum AppState {
    Loading,                        // Loading assets (Tiled JSON files)
    PreparingUsingDynamicAssets,   // Setting up game entities
    Playing,                        // Main game loop
}
```

## Key Dependencies

- **Bevy 0.17** - Game engine
- **bevy_prototype_lyon** - Shape rendering for debug circles
- **bevy_rapier2d** - Physics (future use)
- **leafwing-input-manager** - Input handling

## Running the Game

```bash
cargo run -p tank-game
```

