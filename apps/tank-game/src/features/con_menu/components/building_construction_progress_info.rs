use crate::features::building::types::BuildingTile;
use bevy::prelude::{Component, Resource, State, Timer, TimerMode};
use std::time::Duration;

#[derive(PartialEq)]
enum BuildingConstructionState {
    Idle,
    Constructing,
    Placing,
}

#[derive(Component)]
pub struct BuildingConstructionProgressInfo {
    tick_timer: Timer,
    total_ticks: u32,
    state: BuildingConstructionState,
    building_tile: Option<BuildingTile>,
}

const PRICE_PER_TICK: u32 = 100;

impl BuildingConstructionProgressInfo {
    pub fn new() -> Self {
        Self {
            tick_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            total_ticks: 0,
            state: BuildingConstructionState::Idle,
            building_tile: None,
        }
    }

    pub fn start_from_price(&mut self, price: u32, building_tile: BuildingTile) {
        self.total_ticks = price / PRICE_PER_TICK;
        self.state = BuildingConstructionState::Constructing;
        self.building_tile = Some(building_tile);
    }

    pub fn tick(&mut self, delta: Duration) -> bool {
        if self.tick_timer.tick(delta).just_finished() {
            self.total_ticks = self.total_ticks.saturating_sub(1);
            if self.total_ticks == 0 {
                self.state = BuildingConstructionState::Placing;
            }
            println!("Ticks left: {}", self.total_ticks);
            return true;
        }
        false
    }

    pub fn get_price_per_tick(&self) -> u32 {
        PRICE_PER_TICK
    }

    pub fn is_idle(&self) -> bool {
        self.state == BuildingConstructionState::Idle
    }

    pub fn is_constructing(&self) -> bool {
        self.state == BuildingConstructionState::Constructing
    }

    pub fn is_placing(&self) -> bool {
        self.state == BuildingConstructionState::Placing
    }

    pub fn reset(&mut self) {
        self.state = BuildingConstructionState::Idle;
        self.total_ticks = 0;
    }

    pub fn get_building_tile(&self) -> Option<BuildingTile> {
        self.building_tile.clone()
    }
}
