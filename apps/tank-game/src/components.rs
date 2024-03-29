use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Tank {
    pub id: Id,
    pub selected: bool,
}

#[derive(Component)]
pub struct Id(pub usize);

#[derive(Component)]
pub struct SelectedUnit(pub bool);

#[derive(Component)]
pub struct TargetPosition {
    pub position: Vec2,
    pub speed: f32, // Units per second
    pub moving: bool,
}

#[derive(Component)]
pub struct TankGun {
    pub parent_id: Id,
}

#[derive(Component, Debug)]
pub struct TilePosition {
    center: Vec2,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl TilePosition {
    pub fn new(center: Vec2, width: f32, height: f32) -> Self {
        let x1 = center.x - width / 2.0;
        let x2 = center.x + width / 2.0;
        let y1 = center.y - height / 2.0;
        let y2 = center.y + height / 2.0;
        TilePosition {
            center,
            x1,
            x2,
            y1,
            y2,
        }
    }

    pub fn in_range(&self, x: f32, y: f32) -> bool {
        let in_x = self.x1 <= x && x <= self.x2;
        let in_y = self.y1 <= y && y <= self.y2;
        in_x && in_y
    }

    pub fn get_center(&self) -> Vec2 {
        self.center
    }
}
