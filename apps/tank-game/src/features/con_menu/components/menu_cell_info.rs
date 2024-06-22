use bevy::prelude::Component;

#[derive(Debug, Clone, Component)]
pub struct MenuCellInfo {
    pub name: String,
    pub price: f32,
}

impl MenuCellInfo {
    pub fn new(name: String, price: f32) -> Self {
        Self {
            name,
            price,
        }
    }
}