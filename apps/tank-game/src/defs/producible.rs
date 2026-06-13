//! A unified handle for anything that can sit in a build queue or menu, so the
//! production and UI code can treat buildings and units the same way.

use super::{BuildingKind, UnitKind};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Producible {
    Building(BuildingKind),
    Unit(UnitKind),
}

impl Producible {
    pub fn name(self) -> String {
        match self {
            Producible::Building(b) => b.name().to_string(),
            Producible::Unit(u) => u.name(),
        }
    }

    pub fn short(self) -> String {
        match self {
            Producible::Building(b) => b.short().to_string(),
            Producible::Unit(u) => u.short(),
        }
    }

    pub fn cost(self) -> i64 {
        match self {
            Producible::Building(b) => b.cost(),
            Producible::Unit(u) => u.cost(),
        }
    }

    pub fn build_time(self) -> f32 {
        match self {
            Producible::Building(b) => b.build_time(),
            Producible::Unit(u) => u.build_time(),
        }
    }

    pub fn is_building(self) -> bool {
        matches!(self, Producible::Building(_))
    }

    pub fn description(self) -> String {
        match self {
            Producible::Building(b) => b.description().to_string(),
            Producible::Unit(u) => u.description(),
        }
    }
}
