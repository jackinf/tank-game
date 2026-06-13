//! The tile map: terrain, ore, and passability, plus coordinate conversions
//! and A* pathfinding.
//!
//! - [`tile`]: the [`Tile`] coordinate type and [`Terrain`] kinds.
//! - [`map`]: the [`GameMap`] resource holding the grid and its queries.
//! - [`pathfinding`]: A* over passable tiles ([`find_path`]).

mod map;
mod pathfinding;
mod tile;

pub use map::GameMap;
pub use pathfinding::find_path;
pub use tile::{Terrain, Tile};
