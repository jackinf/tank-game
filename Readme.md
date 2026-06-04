<div align="center">

# Tank Game

### A Red Alert / Dune style real-time strategy game, built in Rust with Bevy

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Bevy](https://img.shields.io/badge/Bevy%200.18-232326?style=for-the-badge&logo=bevy&logoColor=white)](https://bevyengine.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)
[![Repo](https://img.shields.io/badge/GitHub-jackinf%2Ftank--game-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/jackinf/tank-game)

</div>

## Overview

Tank Game is a playable top-down real-time strategy game in the spirit of **Command & Conquer: Red Alert** and the old **Dune** games, written in Rust on top of the [Bevy](https://bevyengine.org/) engine (latest stable, **0.18**).

You start with a small base and have to grow an economy, build a tech tree, produce an army, and crush an AI opponent that is doing exactly the same thing. Everything is rendered with **simple coloured shapes** (sprites + gizmos) rather than art assets, so the focus is entirely on the simulation — sprites can be dropped in later without touching the game logic.

## Features

- **Tile-based maps** with grass, water, mountains/rock and harvestable **ore** fields, loaded from compact ASCII map definitions (two maps bundled).
- **Economy**: credits earned by harvesters, plus a **power** system where low power slows production.
- **Harvesters** with a full state machine: seek the nearest ore, mine it, return to a refinery, deposit for credits, repeat.
- **Buildings** with footprints, health and a real **tech tree**: Construction Yard → Power Plant → Refinery → Barracks / War Factory, plus defensive Gun Turrets.
- **Units**: Soldiers, Tanks and Harvesters, each with their own stats, speed and weapons.
- **Combat**: target acquisition, homing projectiles, area explosions, health bars and death — including attack-move and direct attack orders.
- **A\*** grid pathfinding (8-directional, no corner cutting) for routing units around terrain and buildings.
- **Build menu & construction queue** UI with three lanes (structures / infantry / vehicles), live progress, costs and prerequisite gating.
- **Building placement** mode with a green/red validity preview; structures must be placed near your existing base.
- **Selection**: click, drag-box, formation move orders, rally points for production buildings.
- A **simple AI opponent** that builds an economy, manages power, trains an army and periodically launches attack waves.
- **Win / lose** detection with a game-over screen and instant restart.

## Controls

| Input | Action |
| --- | --- |
| **Left click** | Select a unit / building |
| **Left click + drag** | Box-select your units |
| **Right click** | Move selected units (or attack an enemy under the cursor) |
| **Ctrl + Right click** | Attack-move (engage enemies on the way) |
| **Right click** (building selected) | Set the rally point |
| **Sidebar buttons** | Queue a building or unit |
| **Left click** (after a structure finishes) | Place the new building |
| **W A S D / Arrows / screen edges** | Pan the camera |
| **`+` / `-`** | Zoom in / out |
| **R** | Restart after a win/loss |

## Tech Stack

| Area | Technology |
| --- | --- |
| Language | Rust (edition 2021) |
| Engine | [Bevy](https://bevyengine.org/) 0.18 (latest stable) |
| Rendering | Bevy sprites + gizmos (simple shapes) |
| Pathfinding | Custom A\* over the tile grid |
| RNG | `rand` |
| Web target | WebAssembly (`wasm32-unknown-unknown`) + wasm-bindgen |
| Deploy | Docker, Google Cloud Run, Cloud Storage |

## Getting Started

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) (stable toolchain)
- On Linux, the usual Bevy system dependencies (e.g. `libwayland-dev`, `libxkbcommon-dev`, `libx11-dev`, `libasound2-dev`, `libudev-dev`). See the [Bevy setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/).

### Running

```bash
cargo run -p tank-game --release
```

(`--release` is recommended — Bevy is much smoother optimized. Dependency optimization is also enabled in debug via the workspace `Cargo.toml` profiles.)

### Tests

The core simulation (pathfinding, map loading, economy, tech tree) has headless logic tests that run without a window:

```bash
cargo test -p tank-game
```

## Project Structure

The game is organised as a set of focused Bevy plugins, one per subsystem:

```
apps/tank-game/src/
├── main.rs          # App + plugin wiring
├── config.rs        # Tunable constants (tile size, colours, costs, UI layout)
├── state.rs         # GameState (Loading / Playing / GameOver) + result
├── defs.rs          # Data-driven building & unit stats, weapons, tech tree
├── grid.rs          # Tile map resource, coordinate maths, A* pathfinding
├── maps.rs          # ASCII pre-made maps + loader
├── components.rs    # Shared gameplay components (Health, Mover, Order, ...)
├── faction.rs       # Player / Enemy / Neutral factions
├── spawn.rs         # Building & unit spawn helpers, placement validation
├── setup.rs         # Match setup / restart (loads map, builds both bases)
├── terrain.rs       # Tile + ore rendering
├── camera.rs        # RTS camera (pan / zoom)
├── cursor.rs        # Cursor → world position tracking
├── economy.rs       # Credits & power per faction
├── movement.rs      # Path following
├── combat.rs        # Targeting, weapons, projectiles, damage
├── harvester.rs     # Harvester AI / resource gathering
├── selection.rs     # Selection + move/attack orders
├── production.rs    # Build queues, prerequisites, placement
├── ai.rs            # Enemy AI (build order + attack waves)
├── health.rs        # Health bars, death, win/lose
├── fx.rs            # Explosions & weapon barrels
└── ui.rs            # HUD, build menu, game-over screen
```

The repository is a Cargo workspace; the `examples/` and `apps/tiled/` crates are earlier experiments / demos kept for reference.

## Roadmap

- Sprite art to replace the placeholder shapes
- More unit/building types and abilities
- Fog of war and minimap
- Map selection screen and more bundled maps
- Sound effects and music
- WebAssembly polish
