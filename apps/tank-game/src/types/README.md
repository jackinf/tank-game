# Types

Core type definitions and data structures for the game.

## Type Categories

### Asset Types
Types for parsing Tiled2D asset files:

- `AssetTile` - Individual tile definition from tileset
- `AssetTileId` - Unique tile identifier
- `AssetTileType` - Tile category (ground, building, unit)
- `AssetTileSubType` - Specific type (grass, tank, base, etc.)
- `AssetImagePath` - Path to tile sprite

### Mission Types
Types for parsing Tiled2D map files:

- `MissionInfo` - Complete mission data with all layers
- `MissionLayer` - Single layer of the map
- `RawMission` / `RawMissionLayer` - Deserialized JSON structures
- `PlayersLayer` - Player spawn positions (P1/P2 markers)

### Game Types
Core game type definitions:

- `Player` - Player enumeration (P1, P2)
- `MainAssetInfo` - Parsed asset information
- `MainAssetInfoResource` - Bevy resource holding asset data

## Tiled2D Integration

The types are designed to work with Tiled2D map editor:

```
.tsj files (Tileset JSON) → AssetTile types
.tmj files (Tilemap JSON) → Mission types
```

## Layer Types

Maps contain multiple layers:
- `ground` - Terrain tiles (grass, water, walls)
- `resources` - Resource tiles (gold)
- `players` - Player ownership markers
- `buildings` - Building placements
- `units` - Unit spawn positions

