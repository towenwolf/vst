# GenX Delay GUI Icon Integration Plan

This plan defines how to bring Woodstock-99 themed icons and motifs into the GenX Delay GUI without hurting readability or host stability.

## Goals
- Add subtle themed decoration (dove, barbed wire, tribal accents, distressed texture).
- Preserve parameter legibility and control usability.
- Keep resizing behavior robust (aligned with `GDX-00` scaling fix).
- Avoid introducing audio-thread risk (GUI-only changes).

## Asset Inventory
Assets live in `plugins/genx_delay/assets/icons/`:
- `dove_mark.svg`
- `barbed_wire_strip.svg`
- `tribal_corner.svg`
- `rust_stamp.svg`
- `grunge_speckle_overlay.svg`

## Integration Strategy
Because `egui` does not directly render SVG without extra tooling, use a two-lane strategy:

1. Procedural lane (preferred for scalable motifs)
- Recreate simple line motifs (barbed wire, tribal corners) with `egui::Painter`.
- Benefit: naturally resolution-independent for window resizing.

2. Texture lane (for complex icons)
- Pre-rasterize selected SVGs to PNG at 1x/2x/3x.
- Load textures in GUI and draw with opacity controls.
- Keep icon textures optional and decorative only.

## Phased Implementation

### Phase 1: Scalable decoration primitives
- Add helper functions in `plugins/genx_delay/src/editor.rs`:
  - `draw_barbed_wire(painter, rect, scale)`
  - `draw_tribal_corner(painter, corner, scale)`
  - `draw_grunge_speckles(painter, rect, scale)`
- Compute `scale` from current window size vs base size (`600x420`).
- Gate opacity values to keep controls readable.

Acceptance:
- Decoration scales with window resize.
- No label clipping introduced by decorations.

### Phase 2: Dove + stamp icon textures
- Export `dove_mark.svg` and `rust_stamp.svg` to PNG variants.
- Add icon loading cache/state in editor code.
- Draw icons in low-opacity header/footer regions.

Acceptance:
- Icons render consistently in tested hosts.
- No flicker on repeated open/close.

### Phase 3: Themed placement polish
- Finalize exact placements:
  - Dove: top-right header area
  - Barbed wire: top and bottom separators
  - Tribal: corner accents
  - Grunge overlay: low-opacity full-window speckle pass
  - Rust stamp: subtle section backdrop (behind labels only if contrast passes)
- Tune z-order so controls remain highest priority.

Acceptance:
- Contrast check passes for all section labels and control text.
- Easter-egg feel is subtle, not dominant.

## Technical Tasks in `editor.rs`
- Add `UiScale` helper:
  - base: `600x420`
  - fields: `scale_x`, `scale_y`, `scale_min`
- Update `apply_theme()` to derive font sizes from `scale_min`.
- Convert hard-coded spacing constants to scaled values.
- Add a paint pass before control layout for background motifs.

## QA Checklist
- Resize window from minimum to large sizes:
  - text remains readable
  - controls remain clickable
  - decorations do not overlap critical labels
- Verify in at least Ableton + REAPER + Bitwig.
- Validate repeated GUI open/close for texture lifetime safety.

## Risks and Mitigations
- Risk: textures appear blurry at large sizes.
  - Mitigation: multi-resolution PNG variants.
- Risk: decoration reduces clarity.
  - Mitigation: opacity caps and contrast checks.
- Risk: host-specific repaint quirks.
  - Mitigation: manual smoke tests in `GDX-06`.

## Suggested Next Implementation Slice
1. Implement `UiScale` and scalable font/spacing (`GDX-00`).
2. Add procedural barbed-wire + tribal corner drawing.
3. Add dove icon texture support.
