//! Per-faction economy: credits and power.

use crate::components::Building;
use crate::config::STARTING_CREDITS;
use crate::faction::Faction;
use crate::state::GameState;
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct PlayerEconomy {
    pub credits: i64,
    pub power_produced: i32,
    pub power_consumed: i32,
}

impl Default for PlayerEconomy {
    fn default() -> Self {
        Self { credits: STARTING_CREDITS, power_produced: 0, power_consumed: 0 }
    }
}

impl PlayerEconomy {
    /// True when there is enough power to run everything.
    pub fn has_power(&self) -> bool {
        self.power_produced >= self.power_consumed
    }

    /// Production speed multiplier (slower when low on power).
    pub fn power_factor(&self) -> f32 {
        if self.has_power() {
            1.0
        } else if self.power_produced == 0 {
            0.25
        } else {
            (self.power_produced as f32 / self.power_consumed.max(1) as f32)
                .clamp(0.25, 1.0)
        }
    }

    pub fn can_afford(&self, cost: i64) -> bool {
        self.credits >= cost
    }
}

#[derive(Resource, Default)]
pub struct Economy {
    pub player: PlayerEconomy,
    pub enemy: PlayerEconomy,
}

impl Economy {
    pub fn get(&self, f: Faction) -> &PlayerEconomy {
        match f {
            Faction::Enemy => &self.enemy,
            _ => &self.player,
        }
    }
    pub fn get_mut(&mut self, f: Faction) -> &mut PlayerEconomy {
        match f {
            Faction::Enemy => &mut self.enemy,
            _ => &mut self.player,
        }
    }
}

/// Recompute power production/consumption for both factions from live buildings.
pub fn recompute_power(mut economy: ResMut<Economy>, buildings: Query<(&Building, &Faction)>) {
    economy.player.power_produced = 0;
    economy.player.power_consumed = 0;
    economy.enemy.power_produced = 0;
    economy.enemy.power_consumed = 0;

    for (building, faction) in &buildings {
        let p = building.kind.power();
        let eco = economy.get_mut(*faction);
        if p >= 0 {
            eco.power_produced += p;
        } else {
            eco.power_consumed += -p;
        }
    }
}

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Economy>()
            .add_systems(Update, recompute_power.run_if(in_state(GameState::Playing)));
    }
}
