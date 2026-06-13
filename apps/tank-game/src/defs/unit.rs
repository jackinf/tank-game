//! Every mobile unit: combat units (a class × role × armour-weight matrix) and
//! the ore harvester.

use super::{ArmorKind, ArmorWeight, BuildingKind, ProjectileKind, Role, UnitClass, Weapon};

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
        Self::combat_matrix(UnitClass::Infantry)
    }

    /// All nine tank combinations plus the harvester, for the War Factory menu.
    pub fn vehicles() -> Vec<UnitKind> {
        let mut v = Self::combat_matrix(UnitClass::Vehicle);
        v.push(UnitKind::Harvester);
        v
    }

    /// Every role × weight combination for a given class.
    fn combat_matrix(class: UnitClass) -> Vec<UnitKind> {
        let mut v = Vec::with_capacity(Role::ALL.len() * ArmorWeight::ALL.len());
        for role in Role::ALL {
            for weight in ArmorWeight::ALL {
                v.push(UnitKind::Combat { class, role, weight });
            }
        }
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
            UnitKind::Combat { class, role, weight, .. } => {
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
