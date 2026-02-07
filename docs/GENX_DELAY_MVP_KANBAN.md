# GenX Delay MVP Kanban Board

Source snapshot: `plugins/genx_delay/src/lib.rs` + `plugins/genx_delay/src/editor.rs` + `docs/GENX_DELAY_GUI_DESIGN.md` + `docs/VST_GUI_DEVELOPMENT_STANDARDS.md`

## MVP Definition (for this board)
- Ship a stable VST3/CLAP plugin with the full planned GenX control surface, host-safe automation behavior, and basic host smoke-test coverage.

## Done
- Core DSP path implemented: delay, reverse, feedback filtering, modulation, saturation, ducking.
- Parameters implemented in `GenXDelayParams` (16 user-facing params).
- Tempo-sync engine + note-division math implemented in DSP.
- GUI/editor integration exists and opens successfully.
- Unit/usability tests passing (`cargo test -p genx_delay`: 46 passed, 4 ignored gates).
- **GDX-01**: Full 600x420 GUI layout built — 2-row, 3-column section grid (TIME/MAIN/STEREO + TONE/MODULATION/DUCK), all design colors, GDX-01 gate test enabled and passing.
- **GDX-05**: GUI interaction test gates added for selector contract (unique/order), mode gating logic, and default UI-state parity with params.
- **GDX-08**: Warning-clean gate validated (`cargo clippy -p genx_delay --all-targets -- -D warnings` passes).
- **GDX-00**: Content scaling on resize — `content_scale()` computes uniform scale factor from current vs base window size; `apply_theme` scales font sizes (Heading/Body/Button/Small/Monospace), widget interaction sizes, slider width, item spacing, button padding, and stroke widths proportionally; all `add_space()` calls in the layout multiply by `scale`; minimum scale clamped to 0.5.
- **GDX-02**: All 16 params wired in GUI — Tempo Sync checkbox, Note Division combo (13 options), Mode horizontal buttons (Digital|Analog), Ping Pong checkbox, Stereo Offset slider, HP/LP sliders, Mod Rate/Depth/Drive sliders, Duck Amount/Threshold sliders. Generic `handle_enum_buttons` and `handle_enum_combobox` helpers added with proper host gesture semantics.

## In Progress
- None.

## To Do (Remaining MVP Work)

| ID | Priority | Task | Why it is still open | Definition of Done |
|---|---|---|---|---|
| ~~GDX-00~~ | ~~P0~~ | ~~Fix resizable-window content scaling~~ | ~~Done~~ | ~~Done~~ |
| ~~GDX-01~~ | ~~P0~~ | ~~Build full 600x420 GUI layout~~ | ~~Done~~ | ~~Done~~ |
| ~~GDX-02~~ | ~~P0~~ | ~~Wire all missing controls in GUI~~ | ~~Done~~ | ~~Done~~ |
| GDX-03 | P0 | Add mode-dependent UI states | Design requires modulation controls to be disabled in Digital mode | Mod controls are visually distinct and non-interactive when `Mode=Digital`; interactive when `Mode=Analog` |
| GDX-04 | P1 | Add design polish elements | Woodstock visuals are only partially represented | Barbed-wire separators + section accents implemented without breaking control usability |
| GDX-09 | P1 | Integrate Woodstock icon pack into GUI | Icon assets now exist but are not yet rendered in the `egui` editor | Decorative dove/barbed-wire/tribal/grunge motifs are integrated with scaled placement and low-opacity styling; no control readability regressions |
| ~~GDX-05~~ | ~~P0~~ | ~~Add GUI interaction tests for new controls~~ | ~~Done~~ | ~~Done~~ |
| GDX-06 | P0 | Host smoke test pass (manual) | Repo standards require DAW verification before release | Smoke checks recorded for at least Ableton Live, REAPER, Bitwig, and one additional host: insert/open GUI, automate 3+ params, save/reload, repeated open/close, resize/HiDPI |
| GDX-07 | P1 | Fill plugin metadata/support links | `URL`, `EMAIL`, manual/support URLs are empty/None | Metadata fields set to valid values intended for release builds |
| ~~GDX-08~~ | ~~P1~~ | ~~Clean non-critical warnings~~ | ~~Done~~ | ~~Done~~ |

## Blocked / Questions
- Confirm MVP scope for visual polish: should GDX-04 be required for MVP, or can it move to post-MVP (`v0.2`)?
- Confirm target host matrix (minimum 4th host choice): Logic via CLAP wrapper is out; likely FL Studio or Studio One.

## Suggested Execution Order
1. GDX-00
2. GDX-02
3. GDX-03
4. GDX-09
5. GDX-06
6. GDX-07
7. GDX-04 (or defer based on scope)
