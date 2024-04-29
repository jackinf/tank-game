use crate::building::building_type::BuildingType;
use bevy::prelude::{Component, Query, Res, Time};

pub struct BuildingQueueItem {
    building_type: BuildingType,
    progress: f32,
}

impl BuildingQueueItem {
    pub fn new(building_type: BuildingType) -> Self {
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

    pub fn get_building_type(&self) -> BuildingType {
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

    pub fn push(&mut self, building_type: BuildingType) {
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

pub struct BuildingConstructionManager;

impl BuildingConstructionManager {
    pub fn update_building_construction(
        time: Res<Time>,
        mut q_building_queues: Query<&mut BuildingQueue>,
    ) {
        let delta = time.delta_seconds();

        q_building_queues.iter_mut().for_each(|mut building_queue| {
            building_queue.update(delta);
        });
    }
}
