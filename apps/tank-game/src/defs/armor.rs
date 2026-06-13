//! The armour / weapon-role "rock-paper-scissors" system shared by units and
//! turrets: a weapon's [`Role`] (warhead) scaled against a target's
//! [`ArmorKind`], plus the [`ArmorWeight`] that makes a unit tougher and slower.

use bevy::prelude::Color;

/// Whether a combat unit is foot infantry or an armoured vehicle.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum UnitClass {
    Infantry,
    Vehicle,
}

/// What a weapon is specialised against (its "warhead").
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[allow(clippy::enum_variant_names)] // "Anti-" reads naturally for warheads.
pub enum Role {
    AntiInfantry,
    AntiTank,
    AntiBuilding,
}

impl Role {
    pub const ALL: [Role; 3] = [Role::AntiInfantry, Role::AntiTank, Role::AntiBuilding];

    pub fn short(self) -> &'static str {
        match self {
            Role::AntiInfantry => "AI",
            Role::AntiTank => "AT",
            Role::AntiBuilding => "AB",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Role::AntiInfantry => "Anti-Infantry",
            Role::AntiTank => "Anti-Tank",
            Role::AntiBuilding => "Anti-Building",
        }
    }

    /// Badge colour shown on the unit and the build button.
    pub fn color(self) -> Color {
        match self {
            Role::AntiInfantry => Color::srgb(1.0, 0.85, 0.2),
            Role::AntiTank => Color::srgb(0.9, 0.2, 0.2),
            Role::AntiBuilding => Color::srgb(1.0, 0.55, 0.1),
        }
    }
}

/// Armour weight class: heavier = tougher and slower.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ArmorWeight {
    Light,
    Medium,
    Heavy,
}

impl ArmorWeight {
    pub const ALL: [ArmorWeight; 3] = [ArmorWeight::Light, ArmorWeight::Medium, ArmorWeight::Heavy];

    pub fn short(self) -> &'static str {
        match self {
            ArmorWeight::Light => "L",
            ArmorWeight::Medium => "M",
            ArmorWeight::Heavy => "H",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ArmorWeight::Light => "Light",
            ArmorWeight::Medium => "Medium",
            ArmorWeight::Heavy => "Heavy",
        }
    }

    /// Badge colour shown on the unit and the build button.
    pub fn color(self) -> Color {
        match self {
            ArmorWeight::Light => Color::srgb(0.85, 0.85, 0.9),
            ArmorWeight::Medium => Color::srgb(0.5, 0.5, 0.55),
            ArmorWeight::Heavy => Color::srgb(0.12, 0.12, 0.14),
        }
    }

    pub(super) fn health_mult(self) -> f32 {
        match self {
            ArmorWeight::Light => 1.0,
            ArmorWeight::Medium => 1.6,
            ArmorWeight::Heavy => 2.5,
        }
    }
}

/// The armour category a target presents to incoming weapons. The damage a
/// shot deals is `weapon.damage * damage_multiplier(weapon.role, armor_kind)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ArmorKind {
    Infantry,
    Vehicle,
    Building,
}

/// Rock-paper-scissors damage table. Small arms shred infantry but barely
/// dent armour or buildings; AT rounds wreck vehicles but glance off infantry;
/// demolition warheads flatten buildings.
pub fn damage_multiplier(role: Role, armor: ArmorKind) -> f32 {
    use ArmorKind::*;
    use Role::*;
    match (role, armor) {
        (AntiInfantry, Infantry) => 1.0,
        (AntiInfantry, Vehicle) => 0.25,
        (AntiInfantry, Building) => 0.30,
        (AntiTank, Infantry) => 0.35,
        (AntiTank, Vehicle) => 1.0,
        (AntiTank, Building) => 0.70,
        (AntiBuilding, Infantry) => 0.40,
        (AntiBuilding, Vehicle) => 0.55,
        (AntiBuilding, Building) => 1.50,
    }
}
