//! Path-following movement for units.

use crate::components::*;
use crate::grid::GameMap;
use crate::state::GameState;
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_path.run_if(in_state(GameState::Playing)));
    }
}

fn follow_path(
    time: Res<Time>,
    map: Option<Res<GameMap>>,
    mut movers: Query<(&mut Transform, &mut Mover)>,
) {
    let dt = time.delta_secs();
    let bounds = map.map(|m| m.world_bounds());

    for (mut transform, mut mover) in &mut movers {
        let speed = mover.speed;
        let Some(&target) = mover.path.front() else {
            continue;
        };
        let pos = transform.translation.truncate();
        let to = target - pos;
        let dist = to.length();
        let step = speed * dt;

        if dist <= step.max(2.0) {
            transform.translation.x = target.x;
            transform.translation.y = target.y;
            mover.path.pop_front();
        } else {
            let delta = to / dist * step;
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
        }

        if let Some((min, max)) = bounds {
            transform.translation.x = transform.translation.x.clamp(min.x, max.x);
            transform.translation.y = transform.translation.y.clamp(min.y, max.y);
        }
    }
}
