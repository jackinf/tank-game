use crate::common::player::Player;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Me {
    money: u32,
    energy: i32,
    player: Player,
}

impl Me {
    pub fn new(player: Player) -> Self {
        Me {
            money: 1000,
            energy: 0,
            player,
        }
    }

    pub fn get_money(&self) -> u32 {
        self.money
    }

    pub fn get_energy(&self) -> i32 {
        self.energy
    }

    pub fn get_player(&self) -> Player {
        self.player.clone()
    }

    pub fn add_money(&mut self, amount: u32) {
        self.money += amount;
    }

    pub fn set_energy(&mut self, energy: i32) {
        self.energy = energy;
    }

    pub fn subtract_money(&mut self, amount: u32) {
        self.money -= amount;
    }
}
