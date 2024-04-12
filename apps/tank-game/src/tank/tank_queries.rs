use crate::common::resources::me::Me;
use crate::tank::components::tank::Tank;
use bevy::prelude::{Mut, Query, Sprite, With};

pub struct TankQueries;

impl TankQueries {
    pub fn find_selected<'a>(
        query: &'a mut Query<'a, 'a, (&'a mut Tank, &'a mut Sprite), With<Tank>>,
    ) -> Vec<Mut<'a, Tank>> {
        query
            .iter_mut()
            .filter(|(tank, _)| tank.selected)
            .map(|tank| tank.0)
            .collect()
    }

    pub fn deselect_all_my_units(query: &mut Query<(&mut Tank, &mut Sprite), With<Tank>>, me: &Me) {
        query
            .iter_mut()
            .filter(|(tank, _)| tank.is_mine(&me))
            .for_each(|(mut tank, mut sprite)| {
                tank.deselect(&mut sprite);
            });
    }
}
