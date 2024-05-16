use bevy::prelude::Component;
use std::hash::{Hash, Hasher};

#[derive(Component, Debug, Clone)]
pub struct UnitId(pub usize);

impl PartialEq for UnitId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for UnitId {}

impl Hash for UnitId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
