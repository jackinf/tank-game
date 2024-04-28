use bevy::math::Vec2;
use bevy::prelude::Resource;

#[derive(Resource, Debug)]
pub struct ClickInfo {
    translation: Option<Vec2>,
}

impl ClickInfo {
    pub fn new(translation: Option<Vec2>) -> Self {
        ClickInfo { translation }
    }

    pub fn set_translation(&mut self, translation: Option<Vec2>) {
        self.translation = translation;
    }

    pub fn get_translation(&self) -> Option<Vec2> {
        self.translation
    }
}
