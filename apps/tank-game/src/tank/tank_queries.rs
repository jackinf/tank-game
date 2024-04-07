use crate::tank::tank::Tank;
use bevy::prelude::{Mut, Query, Sprite, With};

pub struct TankQueries;

impl TankQueries {
    pub fn find_selected<'a>(
        mut query: &'a mut Query<'a, 'a, (&'a mut Tank, &'a mut Sprite), With<Tank>>,
    ) -> Vec<Mut<'a, Tank>> {
        query
            .iter_mut()
            .filter(|(tank, _)| tank.selected)
            .map(|tank| tank.0)
            .collect()
    }
}
