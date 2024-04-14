use crate::common::constants::TileCoord;
use crate::common::player::Player;
use bevy::prelude::Component;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum HarvesterState {
    Idle,
    Harvesting,
    Returning,
    ForcedByPlayer,
}

#[derive(Component, Clone, Debug)]
pub struct Harvester {
    id: usize,
    player: Player,
    state: HarvesterState,
    return_to_tile: Option<TileCoord>,
    gold_current_capacity: u32,
    gold_max_capacity: u32,
    harvesting_speed: u32, // how many gold per harvesting_cooldown_seconds
    harvesting_cooldown_seconds: f64,
    last_harvest_timestamp: f64,

    // TODO: unity with Tank using Unit or VehicleUnit trait
    movement_path: VecDeque<TileCoord>,
    health: u32,
    selected: bool,
}

impl Harvester {
    pub fn new(player: Player, id: usize) -> Self {
        Self {
            id,
            player,
            state: HarvesterState::Idle,
            return_to_tile: None,
            gold_current_capacity: 0,
            gold_max_capacity: 1000,
            harvesting_speed: 100,
            harvesting_cooldown_seconds: 1.0,
            last_harvest_timestamp: 0.0,

            movement_path: VecDeque::new(),
            health: 100,
            selected: false,
        }
    }
}

impl Harvester {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_player(&self) -> Player {
        self.player.clone()
    }

    pub fn set_health(&mut self, health: u32) {
        self.health = health;
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn get_selected(&self) -> bool {
        self.selected
    }

    pub fn get_state(&self) -> HarvesterState {
        self.state.clone()
    }

    pub fn get_return_to_tile(&self) -> Option<TileCoord> {
        self.return_to_tile.clone()
    }

    pub fn set_movement_path(&mut self, path: VecDeque<TileCoord>) {
        self.movement_path = path;
    }

    pub fn has_movement_path(&self) -> bool {
        !self.movement_path.is_empty()
    }

    pub fn get_movement_path(&self) -> VecDeque<TileCoord> {
        self.movement_path.clone()
    }

    pub fn is_cooling_down_to_harvest(&self, elapsed_seconds: f64) -> bool {
        elapsed_seconds - self.last_harvest_timestamp < self.harvesting_cooldown_seconds
    }

    pub fn mine_gold(&mut self, gold: u32, elapsed_seconds: f64) {
        self.gold_current_capacity += gold;
        self.last_harvest_timestamp = elapsed_seconds;
    }

    pub fn is_full(&self) -> bool {
        self.gold_current_capacity >= self.gold_max_capacity
    }

    pub fn get_speed(&self) -> f32 {
        100.0
    }

    pub fn try_take_next_position_in_path(&mut self) {
        self.movement_path.pop_front();
    }

    /*
       State Machine logic
    */

    pub fn set_idle(&mut self) {
        self.state = HarvesterState::Idle;
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.state, HarvesterState::Idle)
    }

    pub fn set_harvesting(&mut self) {
        self.state = HarvesterState::Harvesting;
    }

    pub fn is_harvesting(&self) -> bool {
        matches!(self.state, HarvesterState::Harvesting)
    }

    pub fn set_returning(&mut self) {
        self.state = HarvesterState::Returning;
    }

    pub fn is_returning(&self) -> bool {
        matches!(self.state, HarvesterState::Returning)
    }

    pub fn set_forced_by_player(&mut self) {
        self.state = HarvesterState::ForcedByPlayer;
    }

    pub fn is_forced_by_player(&self) -> bool {
        matches!(self.state, HarvesterState::ForcedByPlayer)
    }
}
