use crate::features::unit::UnitId;
use bevy::prelude::Component;

#[derive(Component)]
pub struct TankGun {
    pub parent_id: UnitId,
}

impl TankGun {
    pub fn new(parent_id: UnitId) -> Self {
        TankGun { parent_id }
    }
}
