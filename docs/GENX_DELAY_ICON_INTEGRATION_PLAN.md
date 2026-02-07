# GenX Delay Icon Integration Plan (Archived)

## Status

Archived / inactive.

The icon/decorative motif integration work is currently **de-scoped from runtime UI**.
The plugin now prioritizes functional readability and control usability, and decorative rendering is not part of the active GUI path.

## Why This Was Archived

- Decorative layers did not align with current product goals for a stable, clear, production-ready UI.
- Runtime GUI now intentionally avoids non-functional visual elements.
- Documentation and implementation focus has shifted to DSP reliability, host safety, and control ergonomics.

## Current Source of Truth

- Runtime GUI behavior: `docs/GENX_DELAY_GUI_DESIGN.md`
- MVP/release status: `docs/GENX_DELAY_MVP_KANBAN.md`
- Engineering constraints: `docs/VST_GUI_DEVELOPMENT_STANDARDS.md`

## Note on Existing Code

Some decoration helper code/metrics may remain test-gated for historical regression context, but it is not part of the active runtime render path.
