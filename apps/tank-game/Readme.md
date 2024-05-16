# Tank Game

## Development Guidelines

Structure
```
src/
├── main.rs
├── features/
│   ├── buildings/
│   │   ├── components/
│   │   │   ├── building.rs
│   │   │   └── building_placement_tiles.rs
│   │   ├── resources/
│   │   │   ├── building_resource.rs
│   │   ├── actions/
│   │   │   ├── construct_building.rs
│   │   │   ├── interact_with_building.rs
│   │   │   └── spawn_building.rs
│   │   ├── systems/
│   │   │   ├── construction_system.rs
│   │   │   ├── interaction_system.rs
│   │   │   └── spawn_system.rs
│   │   ├── constants.rs
│   │   ├── types.rs
│   │   ├── building_plugin.rs
│   │   ├── building_tile.rs
│   │   └── mod.rs
|   ├── <feature>/...
│   └── mod.rs
├── components/
│   └── mod.rs
├── resources/
│   └── mod.rs
├── systems/
│   └── mod.rs
└── actions/
    └── mod.rs
```