use crate::common::player::Player;
use bevy::prelude::Component;

#[derive(Component, Clone, Debug)]
pub struct Harvester {
    pub player: Player,
    pub id: usize,
}

impl Harvester {
    pub(crate) fn new(player: Player, id: usize) -> Self {
        Self { player, id }
    }
}

impl Harvester {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_player(&self) -> Player {
        self.player.clone()
    }
}
