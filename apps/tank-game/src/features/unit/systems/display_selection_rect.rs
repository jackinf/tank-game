use crate::features::cursor::CursorCoordinates;
use crate::features::unit::components::UnitSelectionRect;
use bevy::math::Vec3;
use bevy::prelude::{Query, ResMut, Sprite, Transform, With};

pub fn display_selection_rect(
    mut q_tank_selection_rect: Query<
        (&mut UnitSelectionRect, &mut Transform, &mut Sprite),
        With<UnitSelectionRect>,
    >,
    my_world_coords: ResMut<CursorCoordinates>,
) {
    let (mut tank_selection_rect, mut transform, mut sprite) = q_tank_selection_rect.single_mut();

    if tank_selection_rect.is_visible() {
        sprite.color.set_a(0.5);
        let world_coords = my_world_coords.get_world();

        let start = tank_selection_rect.start().unwrap();
        transform.translation = {
            let x = start.x + (world_coords.x - start.x) / 2.0;
            let y = start.y + (world_coords.y - start.y) / 2.0;
            Vec3::new(x, y, 100.0)
        };

        let start = tank_selection_rect.start().unwrap();
        let end = world_coords;
        let width = end.x - start.x;
        let height = end.y - start.y;
        transform.scale = Vec3::new(width, height, 1.0);
    } else {
        sprite.color.set_a(0.0);
    }
}
