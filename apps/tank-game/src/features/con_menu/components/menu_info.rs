use crate::types::player::Player;
use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct MenuInfo {
    money: u32,
    power: i32,
    hovered: bool,
}

impl MenuInfo {
    pub fn new() -> Self {
        Self {
            money: 4000,
            power: 0,
            hovered: false,
        }
    }

    pub fn get_money(&self) -> u32 {
        self.money
    }

    pub fn add_money(&mut self, amount: u32) {
        self.money += amount;
    }

    pub fn substract_money(&mut self, amount: u32) {
        self.money = self.money.saturating_sub(amount);
    }

    pub fn set_money(&mut self, money: u32) {
        self.money = money;
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    pub fn energy(&self) -> i32 {
        self.power
    }

    pub fn player(&self) -> Player {
        return Player::P1;
    }

    pub fn set_energy(&mut self, energy: i32) {
        self.power = energy;
    }

    pub fn has_enough_money(&self, amount: u32) -> bool {
        self.money >= amount
    }

    pub fn subtract_money(&mut self, amount: u32) {
        self.money -= amount;
    }

    pub fn get_build_speed(&self) -> f32 {
        if self.power > 0 {
            1.0
        } else {
            0.5
        }
    }
}
