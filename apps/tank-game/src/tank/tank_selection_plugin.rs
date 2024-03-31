use crate::cursor::cursor_coordinates::WorldCoordinates;
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

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((SpriteBundle {
            texture: asset_server.load("selection.png"),
            transform: Transform::from_xyz(0., 0., 100.),
            ..default()
        },))
        .insert(TankSelectionRect::new());
}

// while holding down left mouse button, set the start and end positions of the selection rectangle
fn calculate_selection_rect_coordinates(
    mut q_tank_selection_rect: Query<&mut TankSelectionRect, With<TankSelectionRect>>,
    mut my_world_coords: ResMut<WorldCoordinates>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    for mouse_button_input_event in mouse_button_input_events.read() {
        if let MouseButton::Left = mouse_button_input_event.button {
            let wx = my_world_coords.0.x;
            let wy = my_world_coords.0.y;

            if let ButtonState::Pressed = mouse_button_input_event.state {
                let mut tank_selection_rect = q_tank_selection_rect.single_mut();
                tank_selection_rect.start = Some(Vec2::new(wx, wy));
                println!("start: {:?}", tank_selection_rect.start);
            }

            if let ButtonState::Released = mouse_button_input_event.state {
                let mut tank_selection_rect = q_tank_selection_rect.single_mut();
                tank_selection_rect.start = None;
            }
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
