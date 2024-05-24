use crate::features::tank::components::Tank;
use crate::resources::me::Me;
use bevy::prelude::{Query, Sprite, With};

pub fn deselect_all_my_units(query: &mut Query<(&mut Tank, &mut Sprite), With<Tank>>, me: &Me) {
    query
        .iter_mut()
        .filter(|(tank, _)| tank.is_mine(&me))
        .for_each(|(mut tank, mut sprite)| {
            tank.deselect(&mut sprite);
        });
}
