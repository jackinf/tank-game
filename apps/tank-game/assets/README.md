# Assets

Game assets including sprites, sounds, fonts, and Tiled2D map files.

## Directory Structure

```
assets/
├── main_assets.tsj      # Tiled tileset definitions
├── mission01.tmj        # First mission map
├── main.tiled-project   # Tiled project file
├── animations/          # Sprite sheet animations
├── cursors/             # Cursor sprites
├── fonts/               # Game fonts
├── maps_legacy/         # Legacy text-based maps
├── pixels/              # Simple colored sprites
├── sounds/              # Sound effects
└── sprites/             # Game sprites
    ├── tiles/           # Terrain tiles
    └── resources/       # Resource sprites
```

## Tiled2D Integration

### Tileset (`main_assets.tsj`)
Defines all available tiles with properties:
- `type`: Category (ground, building, unit, player)
- `subtype`: Specific type (grass, tank, base, p1, p2)
- `image`: Path to sprite

### Mission Maps (`*.tmj`)
Layer-based maps with:
- `ground`: Terrain (grass, water, walls)
- `resources`: Harvestable resources (gold)
- `players`: Player ownership markers
- `buildings`: Starting buildings
- `units`: Starting units

## Editing Maps

1. Open `main.tiled-project` in Tiled2D
2. Edit layers in the map
3. Save as JSON format (`.tmj`)
4. Game loads maps at runtime

## Sprite Conventions

- Tile size: 64x64 or 128x128 pixels
- Buildings: 256x256 pixels
- Units are tinted based on player (P1=blue, P2=red)

