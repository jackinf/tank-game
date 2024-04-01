use crate::cursor::cursor_coordinates::WorldCoordinates;
use crate::tank::tank::Tank;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::sprite::*;

#[derive(Component)]
struct TankSelectionRect {
    start: Option<Vec2>,
}

impl TankSelectionRect {
    fn new() -> Self {
        TankSelectionRect { start: None }
    }

    fn is_visible(&self) -> bool {
        self.start.is_some()
    }
}

pub struct TankSelectionPlugin;

impl Plugin for TankSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, calculate_selection_rect_coordinates)
            .add_systems(FixedUpdate, display_selection_rect);
    }
}

fn setup(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    commands
        .spawn((SpriteBundle {
            texture: asset_server.load("pixels/white.png"),
            transform: Transform::from_xyz(0., 0., 100.),
            sprite: Sprite {
                color: Color::BLUE,
                ..default()
            },
            ..default()
        },))
        .insert(TankSelectionRect::new());
}

// while holding down left mouse button, set the start and end positions of the selection rectangle
fn calculate_selection_rect_coordinates(
    mut q_tank_selection_rect: Query<&mut TankSelectionRect, With<TankSelectionRect>>,
    mut my_world_coords: ResMut<WorldCoordinates>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut tank_query: Query<(&mut Tank, &mut Sprite), With<Tank>>,
) {
    for mouse_button_input_event in mouse_button_input_events.read() {
        let wx = my_world_coords.0.x;
        let wy = my_world_coords.0.y;

        let clicked_on_tank = tank_query
            .iter_mut()
            .find(|(tank, _)| tank.is_tank_clicked_on(wx, wy));

        match (
            mouse_button_input_event.button,
            mouse_button_input_event.state,
            clicked_on_tank,
        ) {
            (MouseButton::Left, ButtonState::Pressed, Some((mut tank, mut sprite))) => {
                tank.toggle(&mut sprite);
            }
            (MouseButton::Left, ButtonState::Pressed, None) => {
                tank_query.iter_mut().for_each(|(mut tank, mut sprite)| {
                    tank.deselect_tank(&mut sprite);
                });

                let mut tank_selection_rect = q_tank_selection_rect.single_mut();
                tank_selection_rect.start = Some(Vec2::new(wx, wy));
            }
            (MouseButton::Left, ButtonState::Released, _) => {
                let mut tank_selection_rect = q_tank_selection_rect.single_mut();
                if tank_selection_rect.start.is_none() {
                    continue;
                }

                let sx = tank_selection_rect.start.unwrap().x;
                let sy = tank_selection_rect.start.unwrap().y;
                tank_selection_rect.start = None;

                // finds and selects tanks within the selection rectangle
                for (mut tank, mut sprite) in &mut tank_query.iter_mut() {
                    let x1 = sx.min(wx);
                    let x2 = sx.max(wx);
                    let y1 = sy.min(wy);
                    let y2 = sy.max(wy);

                    let in_x = x1 <= tank.target_position.x && tank.target_position.x <= x2;
                    let in_y = y1 <= tank.target_position.y && tank.target_position.y <= y2;

                    if in_x && in_y {
                        tank.select_tank(&mut sprite);
                    } else {
                        tank.deselect_tank(&mut sprite);
                    }
                }
            }
            _ => {}
        }

        if mouse_button_input_event.button != MouseButton::Left {
            continue;
        }
    }
}

fn display_selection_rect(
    mut q_tank_selection_rect: Query<
        (&mut TankSelectionRect, &mut Transform, &mut Sprite),
        With<TankSelectionRect>,
    >,
    my_world_coords: ResMut<WorldCoordinates>,
) {
    let (mut tank_selection_rect, mut transform, mut sprite) = q_tank_selection_rect.single_mut();

    if tank_selection_rect.is_visible() {
        sprite.color.set_a(0.5);

        let start = tank_selection_rect.start.unwrap();
        transform.translation = {
            let x = start.x + (my_world_coords.0.x - start.x) / 2.0;
            let y = start.y + (my_world_coords.0.y - start.y) / 2.0;
            Vec3::new(x, y, 100.0)
        };

        let start = tank_selection_rect.start.unwrap();
        let end = my_world_coords.0;
        let width = end.x - start.x;
        let height = end.y - start.y;
        transform.scale = Vec3::new(width, height, 1.0);
    } else {
        sprite.color.set_a(0.0);
    }
}
