//! Data-driven definitions for every building and unit in the game.
//!
//! These types are pure data (no ECS): given a [`BuildingKind`] or [`UnitKind`]
//! you can look up its cost, stats, weapon, colours and so on. The gameplay
//! systems read from here so balance lives in one place.
//!
//! - [`building`]: structures and their stats ([`BuildingKind`]).
//! - [`unit`]: mobile units, combat and harvester ([`UnitKind`]).
//! - [`armor`]: the role / armour damage system shared by both.
//! - [`weapon`]: weapon stats and projectiles.
//! - [`producible`]: a unified build-queue handle over buildings and units.

mod armor;
mod building;
mod producible;
mod unit;
mod weapon;

pub use armor::{damage_multiplier, ArmorKind, ArmorWeight, Role, UnitClass};
pub use building::BuildingKind;
pub use producible::Producible;
pub use unit::UnitKind;
pub use weapon::{ProjectileKind, Weapon};
