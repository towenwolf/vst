# GenX Delay GUI Design

## Overview

The GenX Delay is a VST3/CLAP delay plugin emulating the warm, atmospheric delay sounds popular in late 90s/00s alternative rock — specifically inspired by **Incubus** and the **Line 6 DL4** era.

The GUI theme is **Woodstock 99** with a visual aesthetic inspired by the **original 1969 Woodstock poster** by Arnold Skolnick — deep crimson red background, white/cream graphics, dove-on-guitar iconography, organic flowing shapes, and the spirit of peace, love, and music.

---

## Design Direction

### Style Keywords
- **Bold crimson** — the iconic poster red, commanding and warm
- **Clean with organic accents** — modern layout with 1960s counterculture touches
- **Minimal/flat controls** — simple slider-based controls with cream-on-red contrast
- **Dove-on-guitar motif** — the iconic 1969 Woodstock dove perched on a guitar neck
- **Peace symbols & vine borders** — organic, flowing, nature-inspired decoration

---

## Color Palette

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **BG Crimson** | `#B71C1C` | `183, 28, 28` | Main background — deep poster red |
| **BG Panel Dark** | `#991717` | `153, 23, 23` | Panel group backgrounds — darker crimson |
| **Text Cream** | `#FFF8EB` | `255, 248, 235` | Primary text — warm cream white |
| **Poster White** | `#F0EBE1` | `240, 235, 225` | Decorative lines, borders, subtitle text |
| **Accent Coral** | `#FFA782` | `255, 167, 130` | TIME section accent |
| **Accent Sage** | `#B4D2A0` | `180, 210, 160` | MAIN section, slider selection, active widgets |
| **Accent Sky** | `#A0C3E6` | `160, 195, 230` | STEREO section accent |
| **Accent Amber** | `#FFC878` | `255, 200, 120` | MODULATION section accent |
| **Dove White** | `#FFFCF5` | `255, 252, 245` | Dove motif, DUCK section accent |
| **Accent Tone** | `#DCBEAA` | `220, 190, 170` | TONE section accent |
| **Highlight Gold** | `#FFD78C` | `255, 215, 140` | Hover/active widget highlight |

---

## Window Dimensions

- **Width:** 600px
- **Height:** 420px
- **Style:** Standard size, comfortable for most DAW workflows

---

## Layout

```
┌─────────────────────────────────────────────────────────────────┐
│  ~~~~~~~~~~~~~ VINE BORDER ~~~~~~~~~~~~~~  🕊️🎸 (dove/guitar)  │
│                                                                 │
│                        GENX DELAY                               │
│                     — WOODSTOCK 99 —                            │
│                                                                 │
│  ┌─────────────┐  ┌───────────────────┐  ┌─────────────────┐   │
│  │    TIME     │  │       MAIN        │  │     STEREO      │   │
│  │             │  │                   │  │                 │   │
│  │   [DELAY]   │  │ [FEEDBACK] [MIX]  │  │  [PING PONG]    │   │
│  │             │  │                   │  │                 │   │
│  │   [SYNC]    │  │ [Digital|Analog]  │  │   [OFFSET]      │   │
│  │   [DIV]     │  │                   │  │                 │   │
│  └─────────────┘  └───────────────────┘  └─────────────────┘   │
│                                                                 │
│  ┌─────────────┐  ┌───────────────────┐  ┌─────────────────┐   │
│  │    TONE     │  │    MODULATION     │  │      DUCK       │   │
│  │             │  │   (Analog only)   │  │                 │   │
│  │ [HP]  [LP]  │  │ [RATE][DEPTH][DRV]│  │  [AMT] [THRESH] │   │
│  └─────────────┘  └───────────────────┘  └─────────────────┘   │
│                                                                 │
│  ☮ (peace)   ~~~~~~~~~~~~~ VINE BORDER ~~~~~~~~~~~~~~          │
└─────────────────────────────────────────────────────────────────┘
```

### Sections

1. **TIME** (left column)
   - Delay slider (coral accent)
   - Reverse checkbox
   - Sync checkbox
   - Note Division selector (when synced)

2. **MAIN** (center column)
   - Feedback slider (sage accent)
   - Mix slider (sage accent)
   - Mode selector: Digital | Analog

3. **STEREO** (right column)
   - Ping Pong checkbox
   - Stereo Offset slider (sky accent)

4. **TONE** (bottom left)
   - High-Pass slider (tone beige accent)
   - Low-Pass slider (tone beige accent)

5. **MODULATION** (bottom center)
   - Rate slider (amber accent)
   - Depth slider (amber accent)
   - Drive slider (amber accent)
   - *Visually muted when in Digital mode*

6. **DUCK** (bottom right)
   - Amount slider (dove white)
   - Threshold slider (dove white)

---

## Visual Elements

### Vine Border
- Organic, undulating vine lines at top and bottom of the window
- Poster white color with low opacity
- Small leaf shapes at regular intervals along the vine
- Sine-wave stem with leaf veins for organic feel

### Flower Corner Decorations
- Organic flower clusters in each corner
- Curved stem from corner with S-curve shape
- 5-petal flower at stem end with filled center
- Small buds along the stem
- Poster white color at low opacity

### Dove on Guitar
- Positioned top-right area
- Iconic 1969 Woodstock poster silhouette: dove perched on guitar neck
- Horizontal guitar neck with fret lines and headstock
- Dove body, raised wing, head, beak, tail feathers
- Dove white color at low opacity

### Peace Symbol
- Positioned bottom-left area
- Classic peace sign: circle with internal lines
- Poster white color at low opacity

### Starfield
- Scattered stars and dots across the background
- Mix of tiny dot-stars (75%) and 4-pointed cross-stars (25%)
- Cosmic festival night sky feel
- Deterministic placement via hash function

---

## Control Styles

### Sliders
- **Style:** Horizontal slider with value display
- **Track width:** 100px (scales with window)
- **Labels:** Parameter name and value displayed above slider
- **Interaction:** Horizontal drag, host-safe gestures

### Checkboxes
- **Style:** Standard egui checkbox with label
- **Text:** Cream colored on crimson background

### Enum Selectors (Buttons)
- **Style:** Horizontal button group
- **Selected:** Sage green fill with crimson text
- **Unselected:** Dark crimson panel with cream text
- **Hover:** Gold highlight

### Enum Selectors (Dropdown)
- **Style:** ComboBox dropdown
- **Selected text:** Cream colored

---

## Typography

- **Title:** 24pt proportional, cream white
- **Subtitle:** 9pt proportional, poster white
- **Section labels:** 9pt proportional, strong, section accent color
- **Slider labels:** 9pt proportional, cream
- **Values:** 9pt monospace, poster white

---

## Interaction States

### Enabled Controls
- Full color (cream text, accent-colored labels)
- Responsive to drag/click
- Gold highlight on hover

### Disabled Controls (Modulation in Digital mode)
- Muted amber at 45% opacity for label
- Non-interactive via `add_enabled_ui(false, ...)`
- Visual indication that feature is mode-dependent

---

## Implementation Notes

- Built with **nih_plug_egui** (egui GUI framework for nih-plug)
- Custom painting using egui's Painter API
- Window size: 600x420 via `EguiState::from_size()`
- All decorative elements drawn procedurally (no image assets)
- Theme starts from `Visuals::dark()` base for crimson background

### Key Files
- `plugins/genx_delay/src/editor.rs` — GUI implementation
- `plugins/genx_delay/src/lib.rs` — Plugin with editor integration

### Dependencies
```toml
nih_plug_egui = { git = "https://github.com/robbert-vdh/nih-plug.git" }
```

---

## Future Enhancements

- [ ] Add subtle texture overlay for vintage poster feel
- [ ] Animated dove wing flutter on audio input
- [ ] VU meter or delay time visualization
- [ ] Preset system with era-appropriate names
