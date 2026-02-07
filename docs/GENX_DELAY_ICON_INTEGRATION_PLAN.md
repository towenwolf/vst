# GenX Delay GUI Icon Integration Plan

This plan defines how Woodstock-themed icons and motifs are integrated into the GenX Delay GUI without hurting readability or host stability. The visual aesthetic is inspired by the **original 1969 Woodstock poster** by Arnold Skolnick.

## Goals
- Add subtle themed decoration (dove-on-guitar, vine borders, flower corners, peace symbol, starfield).
- Preserve parameter legibility and control usability on crimson background.
- Keep resizing behavior robust (aligned with `GDX-00` scaling fix).
- Avoid introducing audio-thread risk (GUI-only changes).

## Asset Inventory
Reference SVGs live in `plugins/genx_delay/assets/icons/` (procedural implementations are used in production):
- `dove_mark.svg` — dove-on-guitar reference
- `barbed_wire_strip.svg` — legacy reference (replaced by vine border)
- `tribal_corner.svg` — legacy reference (replaced by flower corner)
- `rust_stamp.svg` — legacy reference (replaced by peace symbol)
- `grunge_speckle_overlay.svg` — legacy reference (replaced by starfield)

## Integration Strategy
All motifs are implemented procedurally via `egui::Painter` — no texture assets are loaded at runtime:

### Procedural Decoration Functions (in `editor.rs`)
| Function | Replaces | Description |
|----------|----------|-------------|
| `draw_vine_border()` | `draw_barbed_wire_line()` | Undulating vine with leaf shapes at intervals |
| `draw_flower_corner()` | `draw_tribal_corner()` | Curved stem with 5-petal flower and buds |
| `draw_dove_on_guitar()` | `draw_dove_mark()` | Dove perched on guitar neck (1969 poster silhouette) |
| `draw_peace_symbol()` | `draw_rust_stamp()` | Classic peace sign circle |
| `draw_starfield()` | `draw_grunge_speckles()` | Scattered dot-stars and cross-stars |

Benefits:
- Naturally resolution-independent for window resizing
- No texture loading or asset management
- No file I/O in GUI code

## Implementation (Completed)

### Phase 1: Scalable decoration primitives
- Helper functions in `plugins/genx_delay/src/editor.rs`:
  - `draw_vine_border(painter, rect, y, spacing, color, stroke_width)` — sine-wave vine with leaf convex polygons
  - `draw_flower_corner(painter, rect, corner, extent, color, scale)` — S-curve stem + 5-petal flower + buds
  - `draw_starfield(painter, rect, scale, color)` — deterministic star placement via hash function
- Scale computed from `content_scale_from_dimensions()` (window size vs base 600x420).
- Opacity values clamped to 6%–18% range per `WoodstockDecorationMetrics`.

### Phase 2: Dove + peace symbol
- `draw_dove_on_guitar(painter, top_left, size, color)` — guitar neck with frets/headstock, dove body/wing/head/beak/tail
- `draw_peace_symbol(painter, center, radius, color, scale)` — outer circle + vertical line + two angled arms

### Phase 3: Themed placement (via `draw_woodstock_decorations()`)
- Dove-on-guitar: top-right header area
- Vine borders: top and bottom edge separators
- Flower corners: all four corners
- Starfield: low-opacity full-window overlay
- Peace symbol: bottom-left area
- All decorations drawn in a background paint pass before controls render.

## Decoration Metrics

`WoodstockDecorationMetrics` struct controls all decoration sizes and opacities:
- `vine_border_opacity`, `flower_corner_opacity`, `dove_opacity`, `starfield_opacity`, `peace_symbol_opacity` — all clamped to 0.06..0.18
- `vine_spacing` — distance between leaf pairs (24px * scale)
- `corner_extent` — flower corner reach (34px * scale)
- `dove_size` — dove-on-guitar bounding box (28px * scale)

## QA Checklist
- Resize window from minimum to large sizes:
  - text remains readable on crimson background
  - controls remain clickable
  - decorations do not overlap critical labels
- Verify in at least Ableton + REAPER + Bitwig.
- Validate repeated GUI open/close for stability.

## Risks and Mitigations
- Risk: cream text on crimson reduces readability.
  - Mitigation: warm cream (#FFF8EB) not pure white, section panels slightly darker crimson.
- Risk: decoration reduces clarity.
  - Mitigation: opacity caps (6%–18%) and contrast checks via test gates.
- Risk: host-specific repaint quirks.
  - Mitigation: manual smoke tests in `GDX-06`.
