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

// ---------------------------------------------------------------------------
// Armour & weapon-role system
// ---------------------------------------------------------------------------

/// Whether a combat unit is foot infantry or an armoured vehicle.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum UnitClass {
    Infantry,
    Vehicle,
}

/// What a weapon is specialised against (its "warhead").
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

    fn health_mult(self) -> f32 {
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

// ---------------------------------------------------------------------------
// Units
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum UnitKind {
    /// A combat unit: a class × weapon-role × armour-weight combination.
    Combat {
        class: UnitClass,
        role: Role,
        weight: ArmorWeight,
    },
    Harvester,
}

impl UnitKind {
    // A few named handles used by setup and the AI.
    pub const BASIC_SOLDIER: UnitKind = UnitKind::Combat {
        class: UnitClass::Infantry,
        role: Role::AntiInfantry,
        weight: ArmorWeight::Light,
    };
    pub const AT_SOLDIER: UnitKind = UnitKind::Combat {
        class: UnitClass::Infantry,
        role: Role::AntiTank,
        weight: ArmorWeight::Medium,
    };
    pub const LIGHT_TANK: UnitKind = UnitKind::Combat {
        class: UnitClass::Vehicle,
        role: Role::AntiInfantry,
        weight: ArmorWeight::Light,
    };
    pub const BASIC_TANK: UnitKind = UnitKind::Combat {
        class: UnitClass::Vehicle,
        role: Role::AntiTank,
        weight: ArmorWeight::Medium,
    };
    pub const HEAVY_TANK: UnitKind = UnitKind::Combat {
        class: UnitClass::Vehicle,
        role: Role::AntiBuilding,
        weight: ArmorWeight::Heavy,
    };

    /// All nine infantry combinations, for the Barracks build menu.
    pub fn infantry() -> Vec<UnitKind> {
        let mut v = Vec::with_capacity(9);
        for role in Role::ALL {
            for weight in ArmorWeight::ALL {
                v.push(UnitKind::Combat {
                    class: UnitClass::Infantry,
                    role,
                    weight,
                });
            }
        }
        v
    }

    /// All nine tank combinations plus the harvester, for the War Factory menu.
    pub fn vehicles() -> Vec<UnitKind> {
        let mut v = Vec::with_capacity(10);
        for role in Role::ALL {
            for weight in ArmorWeight::ALL {
                v.push(UnitKind::Combat {
                    class: UnitClass::Vehicle,
                    role,
                    weight,
                });
            }
        }
        v.push(UnitKind::Harvester);
        v
    }

    pub fn class(self) -> Option<UnitClass> {
        match self {
            UnitKind::Combat { class, .. } => Some(class),
            UnitKind::Harvester => None,
        }
    }

    pub fn role(self) -> Option<Role> {
        match self {
            UnitKind::Combat { role, .. } => Some(role),
            UnitKind::Harvester => None,
        }
    }

    pub fn weight(self) -> Option<ArmorWeight> {
        match self {
            UnitKind::Combat { weight, .. } => Some(weight),
            UnitKind::Harvester => None,
        }
    }

    /// The armour category this unit presents to incoming fire.
    pub fn armor_kind(self) -> ArmorKind {
        match self {
            UnitKind::Combat {
                class: UnitClass::Infantry,
                ..
            } => ArmorKind::Infantry,
            // Vehicles and the harvester are armoured.
            _ => ArmorKind::Vehicle,
        }
    }

    /// A characterful display name, e.g. "Heavy Tank", "Bazooka Joe".
    pub fn name(self) -> String {
        use ArmorWeight::*;
        use Role::*;
        use UnitClass::*;
        let n = match self {
            UnitKind::Harvester => "Ore Harvester",
            UnitKind::Combat { class, role, weight } => match (class, role, weight) {
                // --- Tanks ---
                (Vehicle, AntiInfantry, Light) => "Light Tank",
                (Vehicle, AntiInfantry, Medium) => "Flak Tank",
                (Vehicle, AntiInfantry, Heavy) => "Fat Bob Tank",
                (Vehicle, AntiTank, Light) => "John Tank",
                (Vehicle, AntiTank, Medium) => "Medium Tank",
                (Vehicle, AntiTank, Heavy) => "Heavy Tank",
                (Vehicle, AntiBuilding, Light) => "Sapper Tank",
                (Vehicle, AntiBuilding, Medium) => "Demo Tank",
                (Vehicle, AntiBuilding, Heavy) => "Siege Tank",
                // --- Infantry ---
                (Infantry, AntiInfantry, Light) => "Rifleman",
                (Infantry, AntiInfantry, Medium) => "Gunner",
                (Infantry, AntiInfantry, Heavy) => "Heavy Gunner",
                (Infantry, AntiTank, Light) => "Rocketeer",
                (Infantry, AntiTank, Medium) => "Bazooka Joe",
                (Infantry, AntiTank, Heavy) => "Tank Hunter",
                (Infantry, AntiBuilding, Light) => "Sapper",
                (Infantry, AntiBuilding, Medium) => "Grenadier",
                (Infantry, AntiBuilding, Heavy) => "Demolisher",
            },
        };
        n.to_string()
    }

    pub fn short(self) -> String {
        // The characterful name is short enough for the build buttons and the
        // badges beside it already encode role + armour.
        self.name()
    }

    /// A one-line summary of the unit's strengths and weaknesses, shown in the
    /// build menu so the player can weigh the trade-offs.
    pub fn description(self) -> String {
        let (class, role, weight) = match self {
            UnitKind::Harvester => {
                return "Gathers ore and hauls it back to a Refinery. Unarmed and \
                        defenceless — keep it protected."
                    .to_string()
            }
            UnitKind::Combat { class, role, weight } => (class, role, weight),
        };
        let strength = match role {
            Role::AntiInfantry => "Shreds infantry, but glances off armour and walls",
            Role::AntiTank => "Punches through vehicles, but poor against infantry",
            Role::AntiBuilding => "Levels buildings, but mediocre against units",
        };
        let body = match weight {
            ArmorWeight::Light => "Light and fast, but fragile",
            ArmorWeight::Medium => "A balance of armour and speed",
            ArmorWeight::Heavy => "Heavily armoured, but slow",
        };
        let mobility = match class {
            UnitClass::Infantry => "Foot soldier",
            UnitClass::Vehicle => "Tracked vehicle",
        };
        format!("{mobility}. {strength}. {body}.")
    }

    pub fn cost(self) -> i64 {
        match self {
            UnitKind::Harvester => 1400,
            UnitKind::Combat {
                class, role, weight, ..
            } => {
                let (base, w_add, r_add) = match class {
                    UnitClass::Infantry => (
                        80,
                        match weight {
                            ArmorWeight::Light => 0,
                            ArmorWeight::Medium => 40,
                            ArmorWeight::Heavy => 90,
                        },
                        match role {
                            Role::AntiInfantry => 0,
                            Role::AntiTank => 30,
                            Role::AntiBuilding => 50,
                        },
                    ),
                    UnitClass::Vehicle => (
                        500,
                        match weight {
                            ArmorWeight::Light => 0,
                            ArmorWeight::Medium => 150,
                            ArmorWeight::Heavy => 350,
                        },
                        match role {
                            Role::AntiInfantry => 0,
                            Role::AntiTank => 120,
                            Role::AntiBuilding => 180,
                        },
                    ),
                };
                base + w_add + r_add
            }
        }
    }

    pub fn build_time(self) -> f32 {
        match self {
            UnitKind::Harvester => 10.0,
            UnitKind::Combat { class, weight, .. } => {
                let (base, add) = match class {
                    UnitClass::Infantry => (
                        2.5,
                        match weight {
                            ArmorWeight::Light => 0.0,
                            ArmorWeight::Medium => 0.8,
                            ArmorWeight::Heavy => 1.6,
                        },
                    ),
                    UnitClass::Vehicle => (
                        6.0,
                        match weight {
                            ArmorWeight::Light => 0.0,
                            ArmorWeight::Medium => 2.0,
                            ArmorWeight::Heavy => 4.0,
                        },
                    ),
                };
                base + add
            }
        }
    }

    pub fn max_health(self) -> f32 {
        match self {
            UnitKind::Harvester => 400.0,
            UnitKind::Combat { class, weight, .. } => {
                let base = match class {
                    UnitClass::Infantry => 50.0,
                    UnitClass::Vehicle => 200.0,
                };
                base * weight.health_mult()
            }
        }
    }

    /// Movement speed in world units / second. Lighter = faster.
    pub fn speed(self) -> f32 {
        match self {
            UnitKind::Harvester => 65.0,
            UnitKind::Combat { class, weight, .. } => match (class, weight) {
                (UnitClass::Infantry, ArmorWeight::Light) => 85.0,
                (UnitClass::Infantry, ArmorWeight::Medium) => 68.0,
                (UnitClass::Infantry, ArmorWeight::Heavy) => 54.0,
                (UnitClass::Vehicle, ArmorWeight::Light) => 95.0,
                (UnitClass::Vehicle, ArmorWeight::Medium) => 75.0,
                (UnitClass::Vehicle, ArmorWeight::Heavy) => 58.0,
            },
        }
    }

    /// Visual radius in world units.
    pub fn radius(self) -> f32 {
        match self {
            UnitKind::Harvester => 14.0,
            UnitKind::Combat { class, weight, .. } => {
                let base = match class {
                    UnitClass::Infantry => 6.0,
                    UnitClass::Vehicle => 11.0,
                };
                base + match weight {
                    ArmorWeight::Light => 0.0,
                    ArmorWeight::Medium => 1.0,
                    ArmorWeight::Heavy => 2.0,
                }
            }
        }
    }

    pub fn weapon(self) -> Option<Weapon> {
        let (class, role) = match self {
            UnitKind::Combat { class, role, .. } => (class, role),
            UnitKind::Harvester => return None,
        };
        let (damage, range_tiles, reload, projectile_speed, projectile) = match class {
            UnitClass::Infantry => {
                let dmg = match role {
                    Role::AntiInfantry => 14.0,
                    Role::AntiTank => 20.0,
                    Role::AntiBuilding => 22.0,
                };
                (dmg, 4.0, 0.6, 900.0, ProjectileKind::Bullet)
            }
            UnitClass::Vehicle => {
                let dmg = match role {
                    Role::AntiInfantry => 22.0,
                    Role::AntiTank => 45.0,
                    Role::AntiBuilding => 55.0,
                };
                (dmg, 5.0, 1.2, 550.0, ProjectileKind::Shell)
            }
        };
        Some(Weapon {
            damage,
            range: range_tiles * 32.0,
            sight: (range_tiles + 1.0) * 32.0,
            reload,
            projectile_speed,
            projectile,
            role,
        })
    }

    /// Which building produces this unit.
    pub fn produced_by(self) -> BuildingKind {
        match self {
            UnitKind::Combat {
                class: UnitClass::Infantry,
                ..
            } => BuildingKind::Barracks,
            _ => BuildingKind::WarFactory,
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
    /// Warhead type, used against the target's armour to scale damage.
    pub role: Role,
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
