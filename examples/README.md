# Examples

Proof-of-concept implementations and feature demonstrations.

## Game Feature Demos

### `pathfinder-demo/`
Tank navigation with A* pathfinding.
- Click anywhere to move the tank
- Pathfinding around walls and water obstacles
- Visual path markers

```bash
cargo run -p pathfinder-demo
```

### `shooting-demo/`
Tank combat mechanics.
- Player tank vs enemy tank and building
- Press SPACE to cycle through targets
- Automatic shooting with cooldown
- Explosion animations and sounds
- Health bars and entity destruction

```bash
cargo run -p shooting-demo
```

### `construction-demo/`
Building construction system.
- Press 1-4 to select building types
- Click to place buildings on the grid
- Construction progress with cost deduction
- Placement validation (no overlaps)

```bash
cargo run -p construction-demo
```

### `harvester-demo/`
Harvester state machine demonstration.
- Automatic harvest cycle: search -> move -> collect -> return -> unload
- Right-click for manual control
- Resumes automatic behavior after manual command
- Gold collection and money tracking

```bash
cargo run -p harvester-demo
```

### `stress-test-demo/`
Performance testing under load.
- Spawns new tank every second
- All tanks shoot at central target
- FPS and entity count monitoring
- SPACE to pause/resume, R to reset

```bash
cargo run -p stress-test-demo
```

## Technical Examples

### `sandbox/`
Animation and sprite sheet testing.
- Explosion animation system
- Sprite sheet handling
- Keyboard-triggered animations

```bash
cargo run -p sandbox
```

### `custom-asset/`
Custom asset loading patterns.
- Loading non-standard file formats
- Asset loader implementation
- Tiled JSON file parsing

```bash
cargo run -p custom-asset
```

### `example-ui/`
UI component experiments.
- Custom fonts
- UI layouts
- Button interactions

```bash
cargo run -p example-ui
```

### `tilemap-generator/`
Tilemap generation utilities.
- Procedural map generation
- Tile placement algorithms

```bash
cargo run -p tilemap-generator
```

## Converting Examples to Plugins

These examples can be refactored into plugins for the main game:

1. Identify reusable components/systems
2. Create a new feature directory in `apps/tank-game/src/features/`
3. Implement the `Plugin` trait
4. Move shared code, keeping examples as demonstrations

## Running Examples

From the workspace root:
```bash
cargo run -p <example-name>
```

Or from the example directory:
```bash
cd examples/<example-name>
cargo run
```

