use std::collections::VecDeque;

use bevy::math::Vec2;
use bevy::prelude::{Color, Component, Mut, Sprite};

use crate::constants::TILE_SIZE;
use crate::features::unit::UnitId;
use crate::resources::me::Me;
use crate::types::player::Player;

#[derive(Component)]
pub struct Tank {
    // TODO: remove pubs, use getters
    pub id: UnitId,
    pub selected: bool,
    pub health: u32,
    pub target_position: Vec2, // TODO: Option
    pub speed: f32,            // Units per second
    pub moving: bool,
    pub movement_path: VecDeque<(f32, f32)>,
    pub player: Option<Player>,
    target: Option<UnitId>,
    cooldown_seconds: f64,
    last_shot_timestamp: f64,
    stop_when_target_in_range: bool,
}

impl Tank {
    pub fn new(id: usize, target_position: Vec2, player: Option<Player>) -> Self {
        Tank {
            id: UnitId(id),
            selected: false,
            health: 100,
            target_position,
            speed: 500.0,
            moving: false,
            movement_path: VecDeque::new(),
            player,
            target: None,
            cooldown_seconds: 1.0,
            last_shot_timestamp: 0.0,
            stop_when_target_in_range: false,
        }
    }

    pub fn get_id(&self) -> UnitId {
        self.id.clone()
    }

    pub fn get_radius(&self) -> f32 {
        250.0
    }

    pub fn get_player(&self) -> Option<Player> {
        self.player.clone()
    }

    pub fn get_default_color(&self) -> Color {
        let brightness = 2.;
        match self.player {
            Some(Player::P1) => Color::rgb(0.3, 0.3, 0.7) * brightness,
            Some(Player::P2) => Color::rgb(0.8, 0.2, 0.2) * brightness,
            _ => Color::rgb(0.5, 0.5, 0.5) * brightness,
        }
    }

    pub fn set_stop_when_target_in_range(&mut self, stop: bool) {
        self.stop_when_target_in_range = stop;
    }

    pub fn get_stop_when_target_in_range(&self) -> bool {
        self.stop_when_target_in_range
    }

    pub fn start_moving_to(&mut self, target_position: Vec2) {
        self.target_position = target_position;
        self.moving = true;
    }

    pub fn set_movement_path(&mut self, path: VecDeque<(f32, f32)>) {
        self.movement_path = path;
        self.moving = true;
    }

    pub fn is_moving(&self) -> bool {
        self.moving
    }

    pub fn is_mine(&self, me: &Me) -> bool {
        if let Some(player) = self.player.clone() {
            player == me.get_player()
        } else {
            false
        }
    }

    pub fn try_take_next_position_in_path(&mut self) {
        if self.movement_path.len() > 0 {
            let (x, y) = self.movement_path.pop_front().unwrap();
            self.target_position = Vec2::new(x, y);
        } else {
            self.stop();
        }
    }

    pub fn stop(&mut self) {
        println!("STOP!");

        self.moving = false;
        self.set_stop_when_target_in_range(false);
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

    pub fn set_target(&mut self, target: Option<UnitId>) {
        self.target = target;
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    pub fn get_target(&self) -> Option<UnitId> {
        self.target.clone()
    }

    pub fn is_cooling_down(&self, elapsed_seconds: f64) -> bool {
        let time_since_last_shot = elapsed_seconds - self.last_shot_timestamp;
        time_since_last_shot < self.cooldown_seconds
    }

    pub fn start_cooling_down(&mut self, elapsed_seconds: f64) {
        self.last_shot_timestamp = elapsed_seconds;
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
        let color = self.get_default_color();

        // Make the selected tank brighter by moving closer to white (max value=(1.0, 1.0, 1.0))
        sprite.color = Color::rgb(color.r() + 0.3, color.r() + 0.3, color.b() + 0.3);
    }

    pub fn deselect(&mut self, sprite: &mut Mut<Sprite>) {
        self.selected = false;
        sprite.color = self.get_default_color();
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
