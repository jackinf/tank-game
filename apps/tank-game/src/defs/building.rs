//! Every structure the player and AI can build, with its stats.

use super::{ProjectileKind, Role, Weapon};
use bevy::prelude::Color;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum BuildingKind {
    ConstructionYard,
    PowerPlant,
    Refinery,
    Barracks,
    WarFactory,
    AntiInfantryTurret,
    AntiTankTurret,
}

impl BuildingKind {
    pub const ALL: [BuildingKind; 7] = [
        BuildingKind::ConstructionYard,
        BuildingKind::PowerPlant,
        BuildingKind::Refinery,
        BuildingKind::Barracks,
        BuildingKind::WarFactory,
        BuildingKind::AntiInfantryTurret,
        BuildingKind::AntiTankTurret,
    ];

    pub fn name(self) -> &'static str {
        match self {
            BuildingKind::ConstructionYard => "Construction Yard",
            BuildingKind::PowerPlant => "Power Plant",
            BuildingKind::Refinery => "Refinery",
            BuildingKind::Barracks => "Barracks",
            BuildingKind::WarFactory => "War Factory",
            BuildingKind::AntiInfantryTurret => "Flak Turret",
            BuildingKind::AntiTankTurret => "Cannon Turret",
        }
    }

    pub fn short(self) -> &'static str {
        match self {
            BuildingKind::ConstructionYard => "Yard",
            BuildingKind::PowerPlant => "Power",
            BuildingKind::Refinery => "Refinery",
            BuildingKind::Barracks => "Barracks",
            BuildingKind::WarFactory => "Factory",
            BuildingKind::AntiInfantryTurret => "Flak",
            BuildingKind::AntiTankTurret => "Cannon",
        }
    }

    pub fn cost(self) -> i64 {
        match self {
            BuildingKind::ConstructionYard => 2500,
            BuildingKind::PowerPlant => 300,
            BuildingKind::Refinery => 2000,
            BuildingKind::Barracks => 500,
            BuildingKind::WarFactory => 2000,
            BuildingKind::AntiInfantryTurret => 500,
            BuildingKind::AntiTankTurret => 800,
        }
    }

    /// Build time in seconds.
    pub fn build_time(self) -> f32 {
        match self {
            BuildingKind::ConstructionYard => 20.0,
            BuildingKind::PowerPlant => 6.0,
            BuildingKind::Refinery => 14.0,
            BuildingKind::Barracks => 8.0,
            BuildingKind::WarFactory => 14.0,
            BuildingKind::AntiInfantryTurret => 7.0,
            BuildingKind::AntiTankTurret => 10.0,
        }
    }

    /// Footprint in tiles (width, height).
    pub fn footprint(self) -> (i32, i32) {
        match self {
            BuildingKind::ConstructionYard => (3, 3),
            BuildingKind::PowerPlant => (2, 2),
            BuildingKind::Refinery => (3, 2),
            BuildingKind::Barracks => (2, 2),
            BuildingKind::WarFactory => (3, 2),
            BuildingKind::AntiInfantryTurret => (1, 1),
            BuildingKind::AntiTankTurret => (1, 1),
        }
    }

    pub fn max_health(self) -> f32 {
        match self {
            BuildingKind::ConstructionYard => 1500.0,
            BuildingKind::PowerPlant => 400.0,
            BuildingKind::Refinery => 900.0,
            BuildingKind::Barracks => 600.0,
            BuildingKind::WarFactory => 1000.0,
            BuildingKind::AntiInfantryTurret => 400.0,
            BuildingKind::AntiTankTurret => 550.0,
        }
    }

    /// Net power: positive = produced, negative = consumed.
    pub fn power(self) -> i32 {
        match self {
            BuildingKind::ConstructionYard => 0,
            BuildingKind::PowerPlant => 100,
            BuildingKind::Refinery => -30,
            BuildingKind::Barracks => -20,
            BuildingKind::WarFactory => -30,
            BuildingKind::AntiInfantryTurret => -15,
            BuildingKind::AntiTankTurret => -25,
        }
    }

    /// Prerequisite building required before this one can be built.
    pub fn prerequisite(self) -> Option<BuildingKind> {
        match self {
            BuildingKind::ConstructionYard => None,
            BuildingKind::PowerPlant => Some(BuildingKind::ConstructionYard),
            BuildingKind::Refinery => Some(BuildingKind::PowerPlant),
            BuildingKind::Barracks => Some(BuildingKind::PowerPlant),
            BuildingKind::WarFactory => Some(BuildingKind::Refinery),
            BuildingKind::AntiInfantryTurret => Some(BuildingKind::PowerPlant),
            BuildingKind::AntiTankTurret => Some(BuildingKind::Refinery),
        }
    }

    /// Defensive turrets get a weapon. Each turret type fires a different
    /// warhead, so its effectiveness depends on the target's armour.
    pub fn weapon(self) -> Option<Weapon> {
        match self {
            // Anti-infantry: fast, low damage, shreds soldiers, useless vs tanks.
            BuildingKind::AntiInfantryTurret => Some(Weapon {
                damage: 18.0,
                range: 6.0 * 32.0,
                sight: 7.0 * 32.0,
                reload: 0.4,
                projectile_speed: 900.0,
                projectile: ProjectileKind::Bullet,
                role: Role::AntiInfantry,
            }),
            // Anti-tank: slow, heavy punch, wrecks vehicles.
            BuildingKind::AntiTankTurret => Some(Weapon {
                damage: 50.0,
                range: 6.0 * 32.0,
                sight: 7.0 * 32.0,
                reload: 1.1,
                projectile_speed: 600.0,
                projectile: ProjectileKind::Shell,
                role: Role::AntiTank,
            }),
            _ => None,
        }
    }

    /// A one-line summary shown in the build menu.
    pub fn description(self) -> &'static str {
        match self {
            BuildingKind::ConstructionYard => {
                "The heart of your base. Builds all other structures."
            }
            BuildingKind::PowerPlant => {
                "Supplies power. Low power slows all production — build more as you expand."
            }
            BuildingKind::Refinery => {
                "Processes ore into credits. Harvesters unload at its docking bay."
            }
            BuildingKind::Barracks => "Trains infantry.",
            BuildingKind::WarFactory => "Builds tanks, vehicles and harvesters.",
            BuildingKind::AntiInfantryTurret => {
                "Rapid-fire defence. Mows down infantry, weak against tanks."
            }
            BuildingKind::AntiTankTurret => {
                "Hard-hitting defence. Wrecks vehicles, slow against infantry."
            }
        }
    }

    /// Accent colour for the building body.
    pub fn accent(self) -> Color {
        match self {
            BuildingKind::ConstructionYard => Color::srgb(0.85, 0.75, 0.30),
            BuildingKind::PowerPlant => Color::srgb(0.35, 0.65, 0.85),
            BuildingKind::Refinery => Color::srgb(0.80, 0.55, 0.25),
            BuildingKind::Barracks => Color::srgb(0.55, 0.55, 0.35),
            BuildingKind::WarFactory => Color::srgb(0.50, 0.50, 0.55),
            BuildingKind::AntiInfantryTurret => Color::srgb(0.50, 0.45, 0.30),
            BuildingKind::AntiTankTurret => Color::srgb(0.40, 0.42, 0.48),
        }
    }
}
