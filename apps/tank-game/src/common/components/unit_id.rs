use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct UnitId(pub usize);

impl PartialEq for UnitId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
