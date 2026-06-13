# Asset pipeline & prompts

How the game's building art was made, and the exact prompts used, so new assets
match the existing style.

## Pipeline

1. **Generate** a healthy and a damaged version of the sprite (Midjourney for the
   healthy renders, Gemini for most damaged variants). Keep both in the same
   frame (aspect ratio + camera angle) so they line up in game.
2. **Cut the background** to transparent:
   ```
   make cutbg IN=~/Downloads/render.png OUT=apps/tank-game/assets/buildings/<name>.png
   ```
   This flood-fills from the image edges, so only the background connected to the
   border is removed; look-alike pixels inside the artwork survive. See
   `tools/cutbg.py`.
3. **Wire it up** in `apps/tank-game/src/defs/building.rs`: add the file to
   `texture_path()` (healthy) and `damaged_texture_path()` (shown below 50%
   health). The `texture_buildings` system in `src/fx.rs` does the rest, including
   the subtle faction tint.

## Conventions

- **View:** 3/4 isometric (the Construction Yard is the odd one out, more flat
  top-down; regenerate it isometric if you want a perfectly consistent set).
- **Palette:** muted steel blue and orange, Red Alert / Command & Conquer style,
  bold readable silhouette.
- **Background:** plain flat off-white, subject isolated and centred.
- **Aspect ratio = footprint:** square footprints use `--ar 1:1`, the 3x2
  buildings use `--ar 3:2`. Footprints live in `building.rs::footprint()`.
- **Damaged variant:** same frame and angle as the healthy one, but cracked,
  smoking, scorched, partially collapsed.

## Building prompts (Midjourney, healthy)

Common suffix used on each: `muted steel blue and orange palette, Red Alert /
Command & Conquer style, clean readable silhouette, isolated on plain flat
off-white background --style raw`.

### Construction Yard (3x3, `--ar 1:1`)
```
top-down orthographic view of a sci-fi RTS construction yard building,
flat overhead angle, centered, crane and assembly bay, industrial
```

### Refinery (3x2, `--ar 3:2`)
```
top-down orthographic view of a sci-fi RTS ore refinery building,
flat overhead angle, centered, large central processing silo with a docking
bay on one side for harvester vehicles, conveyor and refinery pipes, industrial
```

### Power Plant (2x2, `--ar 1:1`)
```
3/4 isometric view of a sci-fi RTS power plant building, game asset,
central reactor or cooling tower with glowing energy conduits, vents and
turbine housing, pipes and transformers around the base, industrial
```

### Barracks (2x2, `--ar 1:1`)
```
3/4 isometric view of a sci-fi RTS infantry barracks building, game asset,
reinforced military structure with a large vehicle-style entrance door,
sandbags, antenna and a rooftop vent, soldiers' training facility, industrial
```

### War Factory (3x2, `--ar 3:2`)
```
3/4 isometric view of a sci-fi RTS war factory vehicle assembly building,
game asset, wide industrial hangar with a large roll-up bay door for tanks,
gantry crane, exhaust stacks and assembly arms, heavy machinery, industrial
```

### Flak Turret / anti-infantry (1x1, `--ar 1:1`)
```
3/4 isometric view of a sci-fi RTS anti-aircraft flak turret, game asset,
compact armored base with a rotating multi-barrel rapid-fire gun pointing up,
ammo drums, military, bold readable silhouette
```

### Cannon Turret / anti-tank (1x1, `--ar 1:1`)
```
3/4 isometric view of a sci-fi RTS anti-tank cannon turret, game asset,
heavy armored bunker base with a single large long-barreled tank cannon,
reinforced plating, military, bold readable silhouette
```

## Damaged variants

Generated from the healthy render (img2img / "make a destroyed, smoking,
debris-strewn version"), keeping the same aspect ratio and isometric angle.

## Ground / terrain

The ground is built from seamless tiles plus scattered detail. The smart bit is
in `terrain.rs`: every tile mirrors + brightness-jitters its texture from a hash
of its coords (kills the visible repeat with no extra art), the grass is muted
~30% (`GRASS_MUTE`) so units stay readable, and a sparse rock/dirt decal layer
adds macro variation. Trees are a blocking `Terrain::Forest`.

### Grass (seamless, `--tile`)
```
seamless tileable top-down grass terrain texture for an RTS game,
muted desaturated olive green, subtle dirt patches, very low contrast,
flat even lighting, no large features, no shadows, hand-painted game art
--tile --ar 1:1 --style raw
```
Pick the flattest-lit, lowest-contrast variant (directional lighting fights the
per-tile mirroring). Copied to `assets/terrain/grass_0.png`, `grass_1.png`.

### Ore field (seamless, `--tile`)
```
seamless tileable top-down ore mineral field texture, glittering crystalline
gold and amber crystals clustered on rock, muted, RTS game art
--tile --ar 1:1 --style raw
```
Keep it vivid (you want to spot it). `assets/terrain/ore_0.png`.

### Ground decals (sheet on plain bg)
```
top-down small scattered terrain details for an RTS, set of rocks pebbles
dry grass tufts and dirt cracks, muted desaturated, soft shadow, isolated
on plain flat off-white background --ar 1:1 --style raw
```
These come as contact sheets. Slice + recolour them to individual sprites with:
```
python tools/slice_decals.py apps/tank-game/assets/decals SHEET1.png SHEET2.png ...
```
which cuts the background, splits connected components, drops sparse/spiky tufts,
and desaturates the arid rocks so they sit on green grass.

### Trees (cluster on plain bg, `--ar 1:1`)
```
top-down view of stylized RTS trees, small cluster, dark muted green canopy
with soft drop shadow, isolated on plain flat off-white background
--ar 1:1 --style raw
```
Cut with `make cutbg`. The baked blue-grey drop shadow is knocked out afterwards
(it reads as a puddle on grass). `assets/trees/tree_0..3.png`.
