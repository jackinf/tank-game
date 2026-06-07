//! Data-driven definitions for every building and unit in the game.

use bevy::prelude::Color;

// ---------------------------------------------------------------------------
// Buildings
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum BuildingKind {
    ConstructionYard,
    PowerPlant,
    Refinery,
    Barracks,
    WarFactory,
    GunTurret,
}

impl BuildingKind {
    pub const ALL: [BuildingKind; 6] = [
        BuildingKind::ConstructionYard,
        BuildingKind::PowerPlant,
        BuildingKind::Refinery,
        BuildingKind::Barracks,
        BuildingKind::WarFactory,
        BuildingKind::GunTurret,
    ];

    pub fn name(self) -> &'static str {
        match self {
            BuildingKind::ConstructionYard => "Construction Yard",
            BuildingKind::PowerPlant => "Power Plant",
            BuildingKind::Refinery => "Refinery",
            BuildingKind::Barracks => "Barracks",
            BuildingKind::WarFactory => "War Factory",
            BuildingKind::GunTurret => "Gun Turret",
        }
    }

    pub fn short(self) -> &'static str {
        match self {
            BuildingKind::ConstructionYard => "Yard",
            BuildingKind::PowerPlant => "Power",
            BuildingKind::Refinery => "Refinery",
            BuildingKind::Barracks => "Barracks",
            BuildingKind::WarFactory => "Factory",
            BuildingKind::GunTurret => "Turret",
        }
    }

    pub fn cost(self) -> i64 {
        match self {
            BuildingKind::ConstructionYard => 2500,
            BuildingKind::PowerPlant => 300,
            BuildingKind::Refinery => 2000,
            BuildingKind::Barracks => 500,
            BuildingKind::WarFactory => 2000,
            BuildingKind::GunTurret => 600,
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
            BuildingKind::GunTurret => 8.0,
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
            BuildingKind::GunTurret => (1, 1),
        }
    }

    pub fn max_health(self) -> f32 {
        match self {
            BuildingKind::ConstructionYard => 1500.0,
            BuildingKind::PowerPlant => 400.0,
            BuildingKind::Refinery => 900.0,
            BuildingKind::Barracks => 600.0,
            BuildingKind::WarFactory => 1000.0,
            BuildingKind::GunTurret => 500.0,
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
            BuildingKind::GunTurret => -20,
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
            BuildingKind::GunTurret => Some(BuildingKind::PowerPlant),
        }
    }

    /// Defensive turrets get a weapon.
    pub fn weapon(self) -> Option<Weapon> {
        match self {
            BuildingKind::GunTurret => Some(Weapon {
                damage: 30.0,
                range: 6.0 * 32.0,
                sight: 7.0 * 32.0,
                reload: 0.8,
                projectile_speed: 600.0,
                projectile: ProjectileKind::Shell,
            }),
            _ => None,
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
            BuildingKind::GunTurret => Color::srgb(0.45, 0.45, 0.48),
        }
    }
}

// ---------------------------------------------------------------------------
// Units
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum UnitKind {
    Soldier,
    Tank,
    Harvester,
}

impl UnitKind {
    pub const ALL: [UnitKind; 3] = [UnitKind::Soldier, UnitKind::Tank, UnitKind::Harvester];

    pub fn name(self) -> &'static str {
        match self {
            UnitKind::Soldier => "Soldier",
            UnitKind::Tank => "Tank",
            UnitKind::Harvester => "Harvester",
        }
    }

    pub fn short(self) -> &'static str {
        self.name()
    }

    pub fn cost(self) -> i64 {
        match self {
            UnitKind::Soldier => 100,
            UnitKind::Tank => 700,
            UnitKind::Harvester => 1400,
        }
    }

    pub fn build_time(self) -> f32 {
        match self {
            UnitKind::Soldier => 3.0,
            UnitKind::Tank => 8.0,
            UnitKind::Harvester => 10.0,
        }
    }

    pub fn max_health(self) -> f32 {
        match self {
            UnitKind::Soldier => 60.0,
            UnitKind::Tank => 300.0,
            UnitKind::Harvester => 400.0,
        }
    }

    /// Movement speed in world units / second.
    pub fn speed(self) -> f32 {
        match self {
            UnitKind::Soldier => 90.0,
            UnitKind::Tank => 75.0,
            UnitKind::Harvester => 65.0,
        }
    }

    /// Visual radius in world units.
    pub fn radius(self) -> f32 {
        match self {
            UnitKind::Soldier => 7.0,
            UnitKind::Tank => 12.0,
            UnitKind::Harvester => 14.0,
        }
    }

    pub fn weapon(self) -> Option<Weapon> {
        match self {
            UnitKind::Soldier => Some(Weapon {
                damage: 12.0,
                range: 4.0 * 32.0,
                sight: 5.0 * 32.0,
                reload: 0.5,
                projectile_speed: 900.0,
                projectile: ProjectileKind::Bullet,
            }),
            UnitKind::Tank => Some(Weapon {
                damage: 40.0,
                range: 5.0 * 32.0,
                sight: 6.0 * 32.0,
                reload: 1.2,
                projectile_speed: 550.0,
                projectile: ProjectileKind::Shell,
            }),
            UnitKind::Harvester => None,
        }
    }

    /// Which building produces this unit.
    pub fn produced_by(self) -> BuildingKind {
        match self {
            UnitKind::Soldier => BuildingKind::Barracks,
            UnitKind::Tank => BuildingKind::WarFactory,
            UnitKind::Harvester => BuildingKind::WarFactory,
        }
    }

    pub fn prerequisite(self) -> BuildingKind {
        self.produced_by()
    }
}

// ---------------------------------------------------------------------------
// Weapons & projectiles
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Weapon {
    pub damage: f32,
    /// Maximum firing range, world units.
    pub range: f32,
    /// Vision range for acquiring targets, world units.
    pub sight: f32,
    /// Seconds between shots.
    pub reload: f32,
    /// Projectile travel speed (world units/sec).
    pub projectile_speed: f32,
    pub projectile: ProjectileKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProjectileKind {
    Bullet,
    Shell,
}

impl ProjectileKind {
    pub fn color(self) -> Color {
        match self {
            ProjectileKind::Bullet => Color::srgb(1.0, 0.95, 0.5),
            ProjectileKind::Shell => Color::srgb(1.0, 0.7, 0.2),
        }
    }

    pub fn radius(self) -> f32 {
        match self {
            ProjectileKind::Bullet => 2.0,
            ProjectileKind::Shell => 4.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Producible: a unified handle for anything in a build queue / menu.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Producible {
    Building(BuildingKind),
    Unit(UnitKind),
}

impl Producible {
    pub fn name(self) -> &'static str {
        match self {
            Producible::Building(b) => b.name(),
            Producible::Unit(u) => u.name(),
        }
    }

    pub fn short(self) -> &'static str {
        match self {
            Producible::Building(b) => b.short(),
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
}
