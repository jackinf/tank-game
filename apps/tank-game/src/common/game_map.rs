use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameMap(pub Vec<Vec<usize>>);
