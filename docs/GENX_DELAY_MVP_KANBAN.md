# GenX Delay MVP Kanban Board

Source snapshot: `plugins/genx_delay/src/lib.rs` + `plugins/genx_delay/src/editor.rs` + `docs/GENX_DELAY_GUI_DESIGN.md` + `docs/VST_GUI_DEVELOPMENT_STANDARDS.md`

## MVP Definition (for this board)
- Ship a stable VST3/CLAP plugin with the full planned GenX control surface, host-safe automation behavior, and basic host smoke-test coverage.

## Done
- Core DSP path implemented: delay, reverse, feedback filtering, modulation, saturation, ducking.
- Parameters implemented in `GenXDelayParams` (16 user-facing params).
- Tempo-sync engine + note-division math implemented in DSP.
- GUI/editor integration exists and opens successfully.
- Unit/usability tests passing (`cargo test -p genx_delay`: 40 passed).

## In Progress
- None (board initialized).

## To Do (Remaining MVP Work)

| ID | Priority | Task | Why it is still open | Definition of Done |
|---|---|---|---|---|
| GDX-01 | P0 | Build full 600x420 GUI layout | Current window is 300x200 placeholder and only partially implements design sections | `editor.rs` uses 600x420 default, full 2-row layout exists (TIME/MAIN/STEREO/TONE/MODULATION/DUCK) |
| GDX-02 | P0 | Wire all missing controls in GUI | Current GUI only exposes Delay Time, Reverse, Feedback, Mix | GUI exposes and writes: Tempo Sync, Note Division, Mode, Ping Pong, Stereo Offset, HP/LP, Mod Rate/Depth/Drive, Duck Amount/Threshold |
| GDX-03 | P0 | Add mode-dependent UI states | Design requires modulation controls to be disabled in Digital mode | Mod controls are visually distinct and non-interactive when `Mode=Digital`; interactive when `Mode=Analog` |
| GDX-04 | P1 | Add design polish elements | Woodstock visuals are only partially represented | Barbed-wire separators + section accents implemented without breaking control usability |
| GDX-05 | P0 | Add GUI interaction tests for new controls | Existing tests cover note division semantics but not full editor control presence/contract | Tests validate selector option uniqueness/order, mode gating logic, and default UI state parity with params |
| GDX-06 | P0 | Host smoke test pass (manual) | Repo standards require DAW verification before release | Smoke checks recorded for at least Ableton Live, REAPER, Bitwig, and one additional host: insert/open GUI, automate 3+ params, save/reload, repeated open/close, resize/HiDPI |
| GDX-07 | P1 | Fill plugin metadata/support links | `URL`, `EMAIL`, manual/support URLs are empty/None | Metadata fields set to valid values intended for release builds |
| GDX-08 | P1 | Clean non-critical warnings | `delay_line::process` and `ducker::process` are currently unused | No avoidable dead-code warnings in `genx_delay` (remove or use methods) |

## Blocked / Questions
- Confirm MVP scope for visual polish: should GDX-04 be required for MVP, or can it move to post-MVP (`v0.2`)?
- Confirm target host matrix (minimum 4th host choice): Logic via CLAP wrapper is out; likely FL Studio or Studio One.

## Suggested Execution Order
1. GDX-01
2. GDX-02
3. GDX-03
4. GDX-05
5. GDX-06
6. GDX-07
7. GDX-08
8. GDX-04 (or defer based on scope)
