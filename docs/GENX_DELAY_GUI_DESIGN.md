# GenX Delay GUI Design (Current)

## Overview

GenX Delay ships with a function-first GUI focused on readability, automation safety, and reliable operation across DAWs.

The current UI is intentionally minimal:
- No decorative iconography or texture overlays in runtime rendering.
- Full-root rendering with `egui::CentralPanel`.
- Adaptive section layout for small/medium/large window widths.

## Runtime Design Goals

- Keep all controls readable at practical plugin sizes.
- Keep all controls operable without overlap/clipping.
- Prioritize parameter clarity over decorative styling.
- Maintain host-safe parameter gesture semantics.

## Window and Layout

- Default size: `600x420` via `EguiState::from_size(...)`.
- Root container: `egui::CentralPanel` (no inner floating editor window).
- Adaptive grid behavior:
  - 3 columns on wide widths
  - 2 columns on medium widths
  - 1 column on narrow widths

Sections:
1. `TIME`
2. `MAIN`
3. `STEREO`
4. `TONE`
5. `MODULATION` (disabled in `Digital` mode)
6. `DUCK`

## Typography

Custom fonts are loaded in `editor.rs`:
- `Righteous` for title and section/category labels
- `Josefin Sans` for body controls

Key readability choices:
- Section labels use the same font family as the main title.
- Mode selector text colors are explicit for selected/unselected contrast.
- Slider rows show label/value above the slider to avoid cramped inline text.

## Color Direction (Current)

The active palette is crimson/cream with section accents:
- `BG_CRIMSON`, `BG_PANEL_DARK`
- `TEXT_CREAM`, `POSTER_WHITE`
- `ACCENT_CORAL`, `ACCENT_SAGE`, `ACCENT_SKY`, `ACCENT_AMBER`, `ACCENT_TONE`, `DOVE_WHITE`

## Control Contract

All user-facing controls are mapped to stable plugin parameters in `GenXDelayParams`.

Notable UI behavior:
- `Mode` (`Digital`/`Analog`) gates modulation controls via `add_enabled_ui`.
- `Tempo Sync` + `Note Division` remain available in the `TIME` section.
- `Ping Pong` and `Stereo Offset` controls remain in the `STEREO` section.

## Implementation References

- GUI implementation: `plugins/genx_delay/src/editor.rs`
- DSP and parameter definitions: `plugins/genx_delay/src/lib.rs`
- Engineering standards: `docs/VST_GUI_DEVELOPMENT_STANDARDS.md`

## Out of Scope (Current Runtime)

These are intentionally not rendered in the shipped runtime GUI:
- Decorative borders (barbed wire/vine)
- Corner ornaments (tribal/flower)
- Dove/peace/stamp motifs
- Grunge/star texture overlays

If visuals are revisited later, they should be proposed as a separate optional theme pass and revalidated for readability and host behavior.
