use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct UnitSelectionRect {
    start: Option<Vec2>,
}

impl UnitSelectionRect {
    pub fn new() -> Self {
        UnitSelectionRect { start: None }
    }

    pub fn is_visible(&self) -> bool {
        self.start.is_some()
    }

    pub fn set_start(&mut self, start: Option<Vec2>) {
        self.start = start;
    }

    pub fn start(&mut self) -> Option<Vec2> {
        self.start
    }
}
