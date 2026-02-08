# Rust VST GUI Development Standards

These standards define how we build and ship reliable plugin GUIs in this repository.
They apply to all plugins, including `genx_delay`.

## 1. Platform and Framework Baseline

1. Target modern formats first: VST3 and CLAP.
2. Use `nih_plug` for plugin lifecycle/parameter integration.
3. Use `nih_plug_egui` for GUI unless a plugin has a documented reason to use another UI backend.
4. Pin framework dependencies to a known good commit and upgrade intentionally.

## 2. Reliability Requirements

1. GUI code must never run on the audio thread.
2. Audio processing code must not depend on GUI state.
3. Parameter automation must be host-safe.
4. Use host-visible parameter IDs and avoid changing IDs after release.
5. Keep gesture semantics correct for draggable controls (begin/edit/end).
6. GUI open/close cycles must be leak-free and crash-free across repeated host actions.
7. State restore must survive DAW session reloads with GUI closed or open.

## 3. Threading and Real-Time Safety

1. No blocking operations in the process callback.
2. No filesystem/network access in audio callbacks.
3. No allocation-heavy or lock-contention patterns on real-time paths.
4. UI-to-audio communication must use parameter/state paths designed for real-time safety.
5. Any shared state between UI and DSP must be lock-free or strictly bounded and proven safe.

## 4. Parameter and UI Contract

1. Every user-facing control maps to exactly one stable parameter or an explicitly documented composite behavior.
2. Display formatting and parsing must match parameter units and ranges.
3. Smoothing belongs in DSP-side parameter handling; the GUI should not fake smoothing that diverges from audio behavior.
4. Disabled controls must be visually distinct and non-interactive.
5. Control defaults in UI must match `Params` defaults exactly.

## 5. Windowing, Resize, and HiDPI

1. Declare a deterministic default size using `EguiState::from_size(...)`.
2. Resize behavior must be deterministic and preserve layout integrity.
3. GUI must remain usable at common scale factors (100%, 125%, 150%, 200%).
4. Text and controls must not clip at minimum supported window dimensions.
5. Any host-specific resize workaround must be documented in plugin docs.

## 6. Host Compatibility Matrix

Every release candidate must pass manual smoke tests in at least:

1. REAPER (macOS/Windows)
2. Ableton Live (macOS/Windows)
3. Bitwig Studio (macOS/Windows/Linux where applicable)
4. One additional host relevant to the plugin's target users

Minimum smoke test actions per host:

1. Insert plugin and open GUI.
2. Automate at least three parameters while playing audio.
3. Save project, close host, reopen, verify state.
4. Toggle GUI open/close repeatedly during playback.
5. Validate resizing and HiDPI scaling behavior.

## 7. Validation and CI Gates

1. Build all plugin crates in release mode for target platforms.
2. Run static checks (`cargo fmt`, `cargo clippy`) for changed crates.
3. Run unit/integration tests for DSP and parameter behavior.
4. Run plugin validation tooling (for example `pluginval`) where available in CI.
5. A GUI-affecting PR is not releasable unless host smoke tests are recorded in the PR description.

## 8. Observability and Debugging

1. Keep deterministic reproduction steps for each known GUI/host issue.
2. Log debug information only behind debug flags; do not spam host logs in release builds.
3. Track host/version/plugin-format for every GUI bug report.

## 9. Change Management

1. Any change to parameter IDs, ranges, or semantic meaning requires migration notes.
2. Any GUI framework upgrade requires a host regression pass.
3. Any host-specific workaround must include host name/version, format (VST3/CLAP), exact symptom, and verification steps.

## 10. Repository-Specific Conventions

1. GUI entry point lives in each plugin crate's `src/editor.rs`.
2. Persist editor state through plugin params (example: `#[persist = "editor-state"]`).
3. Keep GUI design docs separate from reliability standards.
4. Use `docs/*_GUI_DESIGN.md` for visual design.
5. Use this document for engineering constraints and release criteria.

## 11. Pre-Release Checklist

1. Parameter IDs unchanged or migration documented.
2. GUI opens, redraws, and closes cleanly across host matrix.
3. Automation playback and write modes verified.
4. Project save/restore verified.
5. HiDPI and resize verified.
6. Validation checks and CI gates pass.
7. Release notes include any known host-specific limitations.
