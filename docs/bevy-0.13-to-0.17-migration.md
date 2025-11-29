# Bevy 0.13 to 0.17 Migration Guide

This document outlines all changes made to migrate the tank-game project from Bevy 0.13 to Bevy 0.17.

## Dependency Updates

### Main Game (`apps/tank-game/Cargo.toml`)

| Dependency | Before | After | Notes |
|------------|--------|-------|-------|
| bevy | 0.13.0 | 0.17.0 | Core engine |
| bevy_prototype_lyon | 0.11.0 | *removed* | Compatibility issues with lyon crate |
| bevy_rapier2d | 0.25.0 | 0.32.0 | Physics engine |
| petgraph | 0.6 | 0.8.3 | Graph algorithms for pathfinding |
| rand | 0.8.5 | 0.9.2 | Random number generation |
| leafwing-input-manager | 0.13.3 | 0.16.0 | Input handling |
| iyes_perf_ui | 0.2.3 | *removed* | Deprecated, use Bevy's built-in diagnostics |
| thiserror | 1.x | 2.0 | Error handling |

### Examples and Apps

All examples updated to `bevy = "0.17.0"` and added `thiserror = "2.0"` where needed.

---

## API Changes

### 1. Time API

**Before (0.13):**
```rust
let dt = time.delta_seconds();
let elapsed = time.elapsed_seconds_f64();
```

**After (0.17):**
```rust
let dt = time.delta_secs();
let elapsed = time.elapsed_secs_f64();
```

**Files affected:** All systems using `Res<Time>`

---

### 2. Event System

**Before (0.13):**
```rust
fn my_system(mut writer: EventWriter<MyEvent>) {
    writer.send(MyEvent { ... });
}
```

**After (0.17):**
```rust
fn my_system(mut writer: MessageWriter<MyEvent>) {
    writer.write(MyEvent { ... });
}
```

**Note:** `EventWriter` is now `MessageWriter`, and `send()` is now `write()`.

**Files affected:**
- `features/tank/systems/sys_move_bullets.rs`
- `features/monitoring/systems/sys_execute_current_tank_strategy.rs`
- `features/con_menu/systems/construction_process.rs`
- `features/monitoring/systems/sys_spawn_enemy_units.rs`

---

### 3. Query Single Methods

**Before (0.13):**
```rust
let entity = query.single_mut();
let entity = query.single();
```

**After (0.17):**
```rust
let entity = query.single_mut().unwrap();
let entity = query.single().unwrap();
```

**Note:** `single()` and `single_mut()` now return `Result<T, QuerySingleError>`.

**Files affected:** 24+ files with single queries

---

### 4. Entity Despawn

**Before (0.13):**
```rust
commands.entity(id).despawn_recursive();
```

**After (0.17):**
```rust
commands.entity(id).despawn();
```

**Note:** `despawn()` is now recursive by default.

---

### 5. Color API

**Before (0.13):**
```rust
Color::rgba(r, g, b, a)
Color::rgb(r, g, b)
color.set_alpha(0.5);
color.set_a(0.5);
```

**After (0.17):**
```rust
Color::srgba(r, g, b, a)
Color::srgb(r, g, b)
color = color.with_alpha(0.5);  // requires `use bevy::color::Alpha;`
```

**Note:** Named colors like `Color::BLUE` are now accessed via `Color::from(bevy::color::palettes::css::BLUE)`.

**Files affected:**
- `constants.rs`
- `features/tank/components/tank.rs`
- `features/building/systems/sys_draw_construction_tiles.rs`
- `features/unit/systems/display_selection_rect.rs`
- `features/con_menu/actions/*.rs`
- `features/con_menu/systems/*.rs`

---

### 6. Camera Bundle

**Before (0.13):**
```rust
commands.spawn(Camera2dBundle::default());
```

**After (0.17):**
```rust
commands.spawn(Camera2d);
// or with transform:
commands.spawn((Camera2d, Transform::from_xyz(...)));
```

---

### 7. Sprite and TextureAtlas

**Before (0.13):**
```rust
commands.spawn(SpriteBundle {
    texture: asset_server.load("image.png"),
    sprite: Sprite { ... },
    ..default()
});

// TextureAtlas as component
commands.spawn((
    SpriteSheetBundle { ... },
    TextureAtlas { layout, index },
));
```

**After (0.17):**
```rust
commands.spawn((
    Sprite {
        image: asset_server.load("image.png"),
        ..default()
    },
    Transform::default(),
));

// TextureAtlas embedded in Sprite
commands.spawn((
    Sprite {
        image: texture,
        texture_atlas: Some(TextureAtlas { layout, index }),
        ..default()
    },
    Transform::default(),
));
```

**Files affected:**
- `features/explosion/systems/prepare_explosion_animation.rs`
- `features/explosion/systems/play_explosion.rs`
- `features/explosion/event_handlers/trigger_explosion_animation_event_handler.rs`
- All sprite spawning code

---

### 8. Text API

**Before (0.13):**
```rust
text.sections[0].value = "new text".to_string();

// TextBundle
TextBundle::from_section("text", TextStyle { ... })
```

**After (0.17):**
```rust
**text = "new text".to_string();

// Separate components
(
    Text::new("text"),
    TextFont { font, font_size, ..default() },
    TextColor(color),
)
```

**Files affected:**
- `features/cursor/systems/show_cursor_coordinates_in_ui.rs`
- `features/con_menu/systems/update_money_text.rs`
- `features/con_menu/systems/update_power_text.rs`
- All UI text spawning code

---

### 9. Window Configuration

**Before (0.13):**
```rust
Window {
    resolution: WindowResolution::new(width as f32, height as f32),
    cursor: Cursor { icon: CursorIcon::Default, ..default() },
    ..default()
}
```

**After (0.17):**
```rust
Window {
    resolution: WindowResolution::new(width as u32, height as u32),  // u32 not f32
    // cursor field removed, use CursorOptions if needed
    ..default()
}
```

---

### 10. Asset Loader

**Before (0.13):**
```rust
impl AssetLoader for MyLoader {
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move { ... })
    }
}
```

**After (0.17):**
```rust
impl AssetLoader for MyLoader {
    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        // direct async, no Box::pin
    }
}
```

---

### 11. Child Spawner

**Before (0.13):**
```rust
fn spawn_child(parent: &mut ChildSpawner) { ... }
```

**After (0.17):**
```rust
fn spawn_child(parent: &mut ChildSpawnerCommands) { ... }
```

**Files affected:**
- `features/con_menu/actions/spawn_money_text.rs`
- `features/con_menu/actions/spawn_power_text.rs`
- `features/con_menu/actions/show_factory_menu_grid.rs`
- `features/con_menu/actions/show_con_base_menu_grid.rs`

---

### 12. Children Iteration

**Before (0.13):**
```rust
for &child in children.iter() { ... }
```

**After (0.17):**
```rust
for child in children.iter() { ... }
```

**Note:** `Children::iter()` now returns `Entity` directly, not `&Entity`.

---

### 13. Viewport to World

**Before (0.13):**
```rust
camera.viewport_to_world(transform, cursor)  // returns Option<Ray3d>
```

**After (0.17):**
```rust
camera.viewport_to_world(transform, cursor)  // returns Result<Ray3d, ViewportConversionError>
```

---

### 14. TextureAtlasLayout

**Before (0.13):**
```rust
TextureAtlasLayout::from_grid(Vec2::new(31., 35.), 5, 1, None, None)
```

**After (0.17):**
```rust
TextureAtlasLayout::from_grid(UVec2::new(31, 35), 5, 1, None, None)
```

---

### 15. App Run Return

**Before (0.13):**
```rust
app.run()  // returns ()
```

**After (0.17):**
```rust
app.run();  // returns AppExit, must be used or ignored
```

---

### 16. Random (rand 0.9)

**Before (0.8):**
```rust
use rand::prelude::SliceRandom;
vec.choose(&mut rand::thread_rng())
```

**After (0.9):**
```rust
use rand::prelude::IndexedRandom;  // for slices
use rand::seq::IteratorRandom;     // for iterators
vec.choose(&mut rand::rng())
```

---

### 17. UI Components

**Before (0.13):**
```rust
NodeBundle { style: Style { ... }, background_color: color.into(), ..default() }
BorderColor(color)
```

**After (0.17):**
```rust
(Node { ... }, BackgroundColor(color))
BorderColor::all(color)
```

---

### 18. Parent Access

**Before (0.13):**
```rust
Query<..., &Parent>
parent.get()
```

**After (0.17):**
```rust
Query<..., &ChildOf>
child_of.parent()
```

---

## Removed Features

### bevy_prototype_lyon
Removed due to compatibility issues with the lyon crate. Debug circles were removed from tank spawning. Consider using Bevy's built-in gizmos for debug visualization.

### iyes_perf_ui
Removed as deprecated. Replaced with Bevy's built-in diagnostics:
```rust
.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin::default())
.add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
```

---

## Migration Reference Links

- [0.13 to 0.14 Migration Guide](https://bevy.org/learn/migration-guides/0-13-to-0-14/)
- [0.14 to 0.15 Migration Guide](https://bevy.org/learn/migration-guides/0-14-to-0-15/)
- [0.15 to 0.16 Migration Guide](https://bevy.org/learn/migration-guides/0-15-to-0-16/)
- [0.16 to 0.17 Migration Guide](https://bevy.org/learn/migration-guides/0-16-to-0-17/)

---

## Post-Migration Notes

After migration, the build produces warnings but compiles successfully. Run `cargo fix` to automatically clean up:
- Unused imports
- Deprecated method warnings (e.g., `add_event` → `add_message`)
- Unnecessary parentheses

```bash
cargo fix --bin "tank-game" --allow-dirty
```

