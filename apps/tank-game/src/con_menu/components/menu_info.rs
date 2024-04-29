use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct MenuInfo {
    money: i32,
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

    pub fn get_money(&self) -> i32 {
        self.money
    }

    pub fn add_money(&mut self, amount: i32) {
        self.money += amount;
    }

    pub fn set_money(&mut self, money: i32) {
        self.money = money;
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }
}
