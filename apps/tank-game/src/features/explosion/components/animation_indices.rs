use bevy::prelude::Component;

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }

    pub fn first(&self) -> usize {
        self.first
    }

    pub fn last(&self) -> usize {
        self.last
    }
}
