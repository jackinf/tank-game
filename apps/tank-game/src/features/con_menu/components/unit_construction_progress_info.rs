use crate::features::unit::UnitTile;
use bevy::prelude::{Component, Resource, State, Timer, TimerMode};
use std::time::Duration;

#[derive(PartialEq)]
enum UnitConstructionState {
    Idle,
    Unit,
    Placing,
}

#[derive(Component)]
pub struct UnitConstructionProgressInfo {
    tick_timer: Timer,
    total_ticks: u32,
    state: UnitConstructionState,
    unit_tile: Option<UnitTile>,
}

const PRICE_PER_TICK: u32 = 100;

impl UnitConstructionProgressInfo {
    pub fn new() -> Self {
        Self {
            tick_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            total_ticks: 0,
            state: UnitConstructionState::Idle,
            unit_tile: None,
        }
    }

    pub fn start_from_price(&mut self, price: u32, unit_tile: UnitTile) {
        self.total_ticks = price / PRICE_PER_TICK;
        self.state = UnitConstructionState::Unit;
        self.unit_tile = Some(unit_tile);
    }

    pub fn tick(&mut self, delta: Duration) -> bool {
        if self.tick_timer.tick(delta).just_finished() {
            self.total_ticks = self.total_ticks.saturating_sub(1);
            if self.total_ticks == 0 {
                self.state = UnitConstructionState::Placing;
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
        self.state == UnitConstructionState::Idle
    }

    pub fn is_unit(&self) -> bool {
        self.state == UnitConstructionState::Unit
    }

    pub fn is_placing(&self) -> bool {
        self.state == UnitConstructionState::Placing
    }

    pub fn reset(&mut self) {
        self.state = UnitConstructionState::Idle;
        self.total_ticks = 0;
    }

    pub fn get_unit_tile(&self) -> Option<UnitTile> {
        self.unit_tile.clone()
    }
}
