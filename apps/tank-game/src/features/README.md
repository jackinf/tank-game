# Features

This directory contains all game features organized as Bevy plugins.

## Plugin Overview

| Plugin | Description | Key Functionality |
|--------|-------------|-------------------|
| `TankPlugin` | Tank units | Movement, shooting, targeting, health |
| `BuildingPlugin` | Base structures | Construction, placement, power management |
| `HarvesterPlugin` | Resource collectors | Gold collection, pathfinding to resources |
| `MenuPlugin` | Construction UI | Building/unit purchase menus, costs |
| `UnitSelectionPlugin` | Selection box | Drag-select units, click selection |
| `ExplosionPlugin` | Visual effects | Explosion animations, sprite sheets |
| `CursorPlugin` | Camera/cursor | Camera movement, world coordinates |
| `MonitoringPlugin` | AI/Performance | Enemy AI spawning, FPS monitoring |
| `DebugPlugin` | Debug tools | Debug info display |

## Feature Module Structure

Each feature follows a consistent structure:

```
<feature>/
├── mod.rs              # Public exports
├── <feature>_plugin.rs # Plugin definition
├── actions/            # Pure functions (no Bevy dependencies)
├── components/         # ECS components
├── systems/            # Bevy systems (Update, FixedUpdate, etc.)
├── resources/          # Bevy resources
├── events/             # Event definitions
├── event_handlers/     # Event handler systems
└── types/              # Type definitions
```

## Adding a New Feature

1. Create a new directory under `features/`
2. Create `mod.rs` with public exports
3. Create `<feature>_plugin.rs` implementing `Plugin`
4. Add the plugin to `main.rs` in the `add_plugins` call
5. Follow the existing structure for consistency

## System Schedules

- **PreStartup**: Initial setup (camera, debug info)
- **Startup**: One-time initialization
- **Update**: Every frame logic (input, UI)
- **FixedUpdate**: Physics and movement (deterministic)

