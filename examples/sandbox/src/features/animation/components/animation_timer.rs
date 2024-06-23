use bevy::prelude::{Component, Deref, DerefMut, Timer};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
