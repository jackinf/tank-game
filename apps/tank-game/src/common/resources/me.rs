use crate::common::constants::Player;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Me {
    money: u32,
    energy: u32,
    player: Player,
}

impl Me {
    pub fn new(player: Player) -> Self {
        Me {
            money: 1000,
            energy: 100,
            player,
        }
    }

    pub fn get_money(&self) -> u32 {
        self.money
    }

    pub fn get_energy(&self) -> u32 {
        self.energy
    }

    pub fn get_player(&self) -> Player {
        self.player.clone()
    }

    pub fn add_money(&mut self, amount: u32) {
        self.money += amount;
    }

    pub fn add_energy(&mut self, amount: u32) {
        self.energy += amount;
    }

    pub fn subtract_money(&mut self, amount: u32) {
        self.money -= amount;
    }

    pub fn subtract_energy(&mut self, amount: u32) {
        self.energy -= amount;
    }
}
