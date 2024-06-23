use bevy::prelude::Resource;

#[derive(Debug, Clone, Resource)]
pub struct GlobalConInfo {
    frozen: bool,
}

impl GlobalConInfo {
    pub fn new() -> Self {
        Self { frozen: false }
    }

    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    pub fn unfreeze(&mut self) {
        self.frozen = false;
    }

    pub fn is_frozen(&self) -> bool {
        self.frozen
    }
}
