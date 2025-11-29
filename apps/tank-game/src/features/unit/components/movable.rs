use bevy::prelude::Component;
use std::collections::VecDeque;

use crate::actions::calculate_tile_world_position::calculate_tile_to_world_position;
use crate::constants::TileCoord;

/// Component for entities that can move along a path.
/// Provides unified movement logic for tanks, harvesters, and other mobile units.
#[derive(Component, Clone, Debug)]
pub struct Movable {
    /// Path of tile coordinates to follow
    movement_path: VecDeque<TileCoord>,
    /// Movement speed in world units per second
    speed: f32,
    /// Whether the unit is currently moving
    moving: bool,
}

impl Movable {
    pub fn new(speed: f32) -> Self {
        Self {
            movement_path: VecDeque::new(),
            speed,
            moving: false,
        }
    }

    /// Set a new movement path and start moving
    pub fn set_path(&mut self, path: VecDeque<TileCoord>) {
        self.movement_path = path;
        self.moving = !self.movement_path.is_empty();
    }

    /// Set a new movement path from a vector
    pub fn set_path_from_vec(&mut self, path: Vec<TileCoord>) {
        self.set_path(VecDeque::from(path));
    }

    /// Check if the unit has remaining waypoints
    pub fn has_path(&self) -> bool {
        !self.movement_path.is_empty()
    }

    /// Check if the unit is currently moving
    pub fn is_moving(&self) -> bool {
        self.moving
    }

    /// Get the speed
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Set the speed
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    /// Stop movement and clear path
    pub fn stop(&mut self) {
        self.moving = false;
        self.movement_path.clear();
    }

    /// Get the next waypoint without removing it
    pub fn peek_next(&self) -> Option<&TileCoord> {
        self.movement_path.front()
    }

    /// Pop and return the next waypoint
    pub fn pop_next(&mut self) -> Option<TileCoord> {
        let next = self.movement_path.pop_front();
        if self.movement_path.is_empty() {
            self.moving = false;
        }
        next
    }

    /// Get the current path length
    pub fn path_length(&self) -> usize {
        self.movement_path.len()
    }

    /// Get the entire remaining path
    pub fn get_path(&self) -> &VecDeque<TileCoord> {
        &self.movement_path
    }

    /// Get the world position of the next waypoint
    pub fn next_world_position(&self) -> Option<bevy::math::Vec2> {
        self.peek_next().map(calculate_tile_to_world_position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movable_path() {
        let mut movable = Movable::new(100.0);
        assert!(!movable.has_path());
        assert!(!movable.is_moving());

        movable.set_path_from_vec(vec![(1, 2), (3, 4), (5, 6)]);
        assert!(movable.has_path());
        assert!(movable.is_moving());
        assert_eq!(movable.path_length(), 3);

        assert_eq!(movable.pop_next(), Some((1, 2)));
        assert_eq!(movable.path_length(), 2);

        movable.stop();
        assert!(!movable.has_path());
        assert!(!movable.is_moving());
    }
}

