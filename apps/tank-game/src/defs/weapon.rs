//! Weapon stats and the projectiles they fire.

use super::Role;
use bevy::prelude::Color;

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
