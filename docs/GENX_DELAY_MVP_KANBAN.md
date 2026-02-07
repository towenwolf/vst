# GenX Delay MVP Kanban Board

Source snapshot: `plugins/genx_delay/src/lib.rs` + `plugins/genx_delay/src/editor.rs` + `docs/GENX_DELAY_GUI_DESIGN.md` + `docs/VST_GUI_DEVELOPMENT_STANDARDS.md`

## MVP Definition

Ship a stable VST3/CLAP plugin with:
- Full planned GenX control surface
- Host-safe automation behavior
- Basic host smoke-test coverage

## Completed

- Core DSP path: delay, reverse, feedback filtering, modulation, saturation, ducking.
- Parameter surface implemented in `GenXDelayParams` (16 user-facing params).
- Tempo-sync engine + note-division math.
- GUI/editor integration live and opening in hosts.
- Full control wiring in GUI (`TIME/MAIN/STEREO/TONE/MODULATION/DUCK`).
- Mode gating (`MODULATION` disabled in `Digital`, enabled in `Analog`).
- Resize/layout robustness:
  - Full-root panel rendering (`CentralPanel`)
  - Adaptive section grid (3/2/1 columns)
  - Scale contracts and tests
- Metadata/support links filled (`URL`, `EMAIL`, `CLAP_MANUAL_URL`, `CLAP_SUPPORT_URL`).
- Ping-pong behavior updated to strict crossfeed alternation path with stereo-offset neutralization in ping-pong mode.
- Safety precautions added:
  - Feedback injection cap
  - Stereo-linked output safety limiter + hard clamp
- Usability/tests:
  - GUI contract tests
  - Ping-pong behavior tests
  - Safety limiter tests

## De-Scoped From Runtime UI

- `GDX-04` decorative polish layer
- `GDX-09` icon/motif runtime integration

These were removed from active runtime rendering to keep the UI function-first and readable.

## In Progress

- `GDX-06` Host smoke test matrix (manual) — report template exists at `docs/GDX_06_SMOKE_TEST_REPORT.md`.

## Remaining MVP Work

| ID | Priority | Task | Why open | Definition of Done |
|---|---|---|---|---|
| GDX-06 | P0 | Host smoke test pass (manual) | Release matrix not fully recorded | Smoke checks recorded for Ableton Live, REAPER, Bitwig, and one additional host: insert/open GUI, automate 3+ params, save/reload, repeated open/close, resize/HiDPI |

## Suggested Execution Order

1. GDX-06
