use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ConState {
    #[default]
    Idle,
    Building,
    Placing,
}
