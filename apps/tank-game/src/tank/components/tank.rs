use crate::common::constants::TILE_SIZE;
use bevy::math::Vec2;
use bevy::prelude::{Color, Component, Mut, Sprite};
use std::collections::VecDeque;
use crate::common::components::unit_id::UnitId;

#[derive(Component)]
pub struct Tank {
    pub id: UnitId,
    pub selected: bool,
    pub health: u32,
    pub target_position: Vec2,
    pub speed: f32, // Units per second
    pub moving: bool,
    pub movement_path: VecDeque<(f32, f32)>,
}

impl Tank {
    pub fn new(id: usize, target_position: Vec2) -> Self {
        Tank {
            id: UnitId(id),
            selected: false,
            health: 100,
            target_position,
            speed: 500.0,
            moving: false,
            movement_path: VecDeque::new(),
        }
    }

    pub fn start_moving_to(&mut self, target_position: Vec2) {
        self.target_position = target_position;
        self.moving = true;
    }

    pub fn set_movement_path(&mut self, path: VecDeque<(f32, f32)>) {
        self.moving = true;
        self.movement_path = path;
    }

    pub fn is_moving(&self) -> bool {
        self.moving
    }

    pub fn try_take_next_position_in_path(&mut self) {
        if self.movement_path.len() > 0 {
            let (x, y) = self.movement_path.pop_front().unwrap();
            self.target_position = Vec2::new(x, y);
        } else {
            self.moving = false;
        }
    }

    pub fn stop(&mut self) {
        self.moving = false;
    }

    pub fn is_clicked_on(&self, wx: f32, wy: f32) -> bool {
        let offset = TILE_SIZE / 2.0;

        let x1 = self.target_position.x - offset;
        let x2 = self.target_position.x + TILE_SIZE - offset;
        let in_x = x1 <= wx && wx <= x2;

        let y1 = self.target_position.y - offset;
        let y2 = self.target_position.y + TILE_SIZE - offset;
        let in_y = y1 <= wy && wy <= y2;

        in_x && in_y
    }

    pub fn toggle(&mut self, sprite: &mut Mut<Sprite>) {
        if self.selected {
            self.deselect(sprite);
        } else {
            self.select(sprite);
        }
    }

    pub fn select(&mut self, sprite: &mut Mut<Sprite>) {
        self.selected = true;
        sprite.color = Color::rgb(2.0, 2.0, 2.0);
    }

    pub fn deselect(&mut self, sprite: &mut Mut<Sprite>) {
        self.selected = false;
        sprite.color = Color::WHITE;
    }

    pub fn take_damage(&mut self, damage: u32) {
        if self.health <= damage {
            self.health = 0;
        } else {
            self.health -= damage;
        }

        if self.health == 0 {
            self.moving = false;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }
}
