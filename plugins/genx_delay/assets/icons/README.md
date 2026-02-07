# GenX Delay Icon Pack (Woodstock 99 Theme)

This folder contains starter SVG assets for the GenX Delay GUI.

## Style goals
- Dusty and distressed
- Warm earth tones from the existing GUI palette
- Subtle late-90s tribal and industrial hints
- Decorative "easter eggs", not dominant UI elements

## Files
- `dove_mark.svg`: small peace-dove emblem for header corner
- `barbed_wire_strip.svg`: horizontal barbed-wire divider
- `tribal_corner.svg`: corner ornament (rotate/flip for all corners)
- `rust_stamp.svg`: distressed circular stamp motif
- `grunge_speckle_overlay.svg`: low-opacity texture overlay

## Usage guidance
- Prefer decorative placement in header/footer and corners.
- Keep icon opacity low for overlays (6% to 18%).
- Do not place icons behind active text labels with low contrast.
- Scale by UI size factor so design remains readable at all window sizes.

## Integration note for egui
`egui` does not natively render SVG files directly without extra tooling.
For implementation, choose one:
1. Pre-rasterize to PNG at 1x/2x/3x and load as textures.
2. Recreate simple motifs procedurally with `egui::Painter`.
3. Add an SVG rasterization step (build-time or runtime) and upload textures.
