# Examples

Proof-of-concept implementations and experimental features.

## Available Examples

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

