use crate::features::building::types::building_tile::BuildingTile;
use bevy::prelude::Component;

struct BuildingQueueItem {
    building_type: BuildingTile,
    progress: f32,
}

impl BuildingQueueItem {
    pub fn new(building_type: BuildingTile) -> Self {
        BuildingQueueItem {
            progress: 0.0,
            building_type,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn update(&mut self, delta: f32) {
        self.progress += delta;
    }

    pub fn get_progress(&self) -> f32 {
        self.progress
    }

    pub fn get_building_type(&self) -> BuildingTile {
        self.building_type.clone()
    }
}

#[derive(Component)]
pub struct BuildingQueue {
    items: Vec<BuildingQueueItem>,
}

impl BuildingQueue {
    pub fn new() -> Self {
        BuildingQueue { items: Vec::new() }
    }

    pub fn push(&mut self, building_type: BuildingTile) {
        self.items.push(BuildingQueueItem::new(building_type));
    }

    pub fn update(&mut self, delta: f32) {
        for item in self.items.iter_mut() {
            item.update(delta);
        }
    }

    pub fn pop(&mut self) -> Option<BuildingQueueItem> {
        self.items.pop()
    }
}
