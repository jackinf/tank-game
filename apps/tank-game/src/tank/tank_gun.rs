use crate::tank::tank_id::TankId;
use bevy::prelude::Component;

#[derive(Component)]
pub struct TankGun {
    pub parent_id: TankId,
}

impl TankGun {
    pub fn new(parent_id: TankId) -> Self {
        TankGun { parent_id }
    }
}
