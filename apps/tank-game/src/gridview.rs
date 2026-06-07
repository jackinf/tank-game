//! A toggleable ground grid overlay so the player can see the tile layout
//! that units path along. Toggle with the `G` key.

use crate::config::TILE;
use crate::grid::GameMap;
use crate::state::GameState;
use bevy::prelude::*;

/// Whether the tile grid is currently drawn.
#[derive(Resource, Default)]
pub struct GridOverlay(pub bool);

pub struct GridViewPlugin;

impl Plugin for GridViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridOverlay>().add_systems(
            Update,
            (toggle_grid, draw_grid)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn toggle_grid(keys: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<GridOverlay>) {
    if keys.just_pressed(KeyCode::KeyG) {
        overlay.0 = !overlay.0;
    }
}

fn draw_grid(overlay: Res<GridOverlay>, map: Option<Res<GameMap>>, mut gizmos: Gizmos) {
    if !overlay.0 {
        return;
    }
    let Some(map) = map else { return };
    let (min, max) = map.world_bounds();
    let color = Color::srgba(1.0, 1.0, 1.0, 0.12);

    for col in 0..=map.width {
        let x = min.x + col as f32 * TILE;
        gizmos.line_2d(Vec2::new(x, min.y), Vec2::new(x, max.y), color);
    }
    for row in 0..=map.height {
        let y = min.y + row as f32 * TILE;
        gizmos.line_2d(Vec2::new(min.x, y), Vec2::new(max.x, y), color);
    }
}
