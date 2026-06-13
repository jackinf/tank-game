//! Tile coordinates and the terrain types that fill them.

use bevy::prelude::Color;

/// A tile coordinate (column, row), with row 0 at the top of the map.
pub type Tile = (i32, i32);

/// The kind of terrain on a tile.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Terrain {
    Grass,
    Water,
    Mountain,
    /// Ore-bearing ground. Harvestable; the amount lives in `GameMap::ore`.
    Ore,
    /// Dense trees. Blocks movement and building, like a Red Alert forest.
    Forest,
}

impl Terrain {
    /// Base (no-ore) colour for rendering. Used as a fallback before the tile
    /// texture loads, and under transparent overlays (ore, trees).
    pub fn color(self) -> Color {
        match self {
            Terrain::Grass => Color::srgb(0.22, 0.42, 0.18),
            Terrain::Water => Color::srgb(0.12, 0.28, 0.55),
            Terrain::Mountain => Color::srgb(0.32, 0.30, 0.28),
            Terrain::Ore => Color::srgb(0.22, 0.42, 0.18),
            Terrain::Forest => Color::srgb(0.18, 0.34, 0.16),
        }
    }

    /// Does this tile sit on grass (and so use the grass texture as its base)?
    pub fn is_grassy(self) -> bool {
        matches!(self, Terrain::Grass | Terrain::Ore | Terrain::Forest)
    }

    /// Terrain that units/buildings can never occupy.
    pub fn is_solid(self) -> bool {
        matches!(self, Terrain::Water | Terrain::Mountain | Terrain::Forest)
    }
}
