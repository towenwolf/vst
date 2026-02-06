# GenX Delay GUI Design

## Overview

The GenX Delay is a VST3/CLAP delay plugin emulating the warm, atmospheric delay sounds popular in late 90s/00s alternative rock — specifically inspired by **Incubus** and the **Line 6 DL4** era.

The GUI theme is **Woodstock 99** — the iconic (and infamous) festival where Incubus performed. The visual aesthetic captures the dusty, sun-bleached festival atmosphere with late-90s design elements.

---

## Design Direction

### Style Keywords
- **Subdued/dusty** — sun-bleached posters, worn asphalt, desert heat
- **Clean with grunge accents** — modern layout with era-appropriate touches
- **Minimal/flat controls** — simple arc-style knobs with 90s graphic design
- **Dove motif** — Woodstock's iconic peace dove symbol
- **Barbed wire & tribal tattoos** — the edgier, industrial side of the era

---

## Color Palette

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **BG Main** | `#EBE4D7` | `235, 228, 215` | Main background — dusty off-white/cream |
| **BG Panel** | `#D7CDBE` | `215, 205, 190` | Panel backgrounds — darker cream |
| **Accent Warm** | `#B45F41` | `180, 95, 65` | Terracotta — Time controls, active toggles |
| **Accent Olive** | `#697350` | `105, 115, 80` | Dusty olive — Main/Feedback controls |
| **Accent Navy** | `#3C465A` | `60, 70, 90` | Worn denim — Stereo controls |
| **Text Dark** | `#2D2823` | `45, 40, 35` | Charcoal — primary text, outlines |
| **Tribal Brown** | `#4B3728` | `75, 55, 40` | Faded brown — tribal elements, borders |
| **Rust** | `#8C4B32` | `140, 75, 50` | Rust — barbed wire, modulation section |
| **Dove Gold** | `#AF9B64` | `175, 155, 100` | Muted gold — dove accent, ducking section |

---

## Window Dimensions

- **Width:** 600px
- **Height:** 420px
- **Style:** Standard size, comfortable for most DAW workflows

---

## Layout

```
┌─────────────────────────────────────────────────────────────────┐
│  ════════════════ BARBED WIRE ════════════════      🕊️ (dove)   │
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
│  ════════════════ BARBED WIRE ════════════════                  │
└─────────────────────────────────────────────────────────────────┘
```

### Sections

1. **TIME** (left column)
   - Delay knob (terracotta accent)
   - Sync toggle button
   - Note Division selector (when synced)

2. **MAIN** (center column)
   - Feedback knob (olive accent)
   - Mix knob (olive accent)
   - Mode selector: Digital | Analog

3. **STEREO** (right column)
   - Ping Pong toggle
   - Stereo Offset knob (navy accent)

4. **TONE** (bottom left)
   - High-Pass knob (tribal brown)
   - Low-Pass knob (tribal brown)

5. **MODULATION** (bottom center)
   - Rate knob (rust accent)
   - Depth knob (rust accent)
   - Drive knob (rust accent)
   - *Grayed out when in Digital mode*

6. **DUCK** (bottom right)
   - Amount knob (dove gold)
   - Threshold knob (dove gold)

---

## Visual Elements

### Barbed Wire Border
- Horizontal lines at top and bottom of the window
- Rust-colored (`#8C4B32`)
- Small X-shaped barbs every ~25px with circular wraps
- Provides the industrial/aggressive festival edge

### Tribal Corner Decorations
- Angular, swooping lines in each corner
- Inspired by late-90s tribal tattoo aesthetics
- Tribal brown color (`#4B3728`)
- Inner accent lines at 60% opacity for depth

### Peace Dove
- Positioned top-right area
- Simplified, iconic Woodstock dove silhouette
- Dove gold color (`#AF9B64`)
- Includes small olive branch with leaves
- Wing curves suggesting flight

---

## Control Styles

### Knobs
- **Style:** Minimal arc-based
- **Size:** 55px diameter
- **Track:** 270° arc (from 7 o'clock to 5 o'clock)
- **Indicator:** Line from center to current position
- **Center dot:** 6px filled circle
- **Interaction:** Vertical drag, double-click to reset
- **Labels:** 9pt text below knob, value above

### Toggle Buttons
- **Size:** 70×22px
- **Style:** Rounded rectangle (3px radius)
- **Off state:** Panel background with brown border
- **On state:** Accent color fill with light text
- **Text:** 10pt centered

### Enum Selectors
- **Style:** Horizontal button group
- **Button size:** 55×20px each
- **Selected:** Olive fill with light text
- **Unselected:** Panel background with brown border

---

## Typography

- **Title:** 32pt proportional, bold, charcoal
- **Subtitle:** 11pt proportional, tribal brown
- **Section labels:** 10pt proportional, charcoal
- **Knob labels:** 9pt proportional
- **Values:** 9pt proportional

---

## Interaction States

### Enabled Controls
- Full color
- Responsive to drag/click
- Cursor changes on hover (if supported)

### Disabled Controls (Modulation in Digital mode)
- Muted/grayed colors (`#969187` range)
- Non-interactive
- Visual indication that feature is mode-dependent

---

## Implementation Notes

- Built with **nih_plug_egui** (egui GUI framework for nih-plug)
- Custom painting using egui's Painter API
- Window size: 600×420 via `EguiState::from_size()`
- All decorative elements drawn procedurally (no image assets)

### Key Files
- `plugins/genx_delay/src/editor.rs` — GUI implementation
- `plugins/genx_delay/src/lib.rs` — Plugin with editor integration

### Dependencies
```toml
nih_plug_egui = { git = "https://github.com/robbert-vdh/nih-plug.git" }
```

---

## Future Enhancements

- [ ] Add subtle texture overlay for more authentic dusty feel
- [ ] Animated dove wing flutter on audio input
- [ ] VU meter or delay time visualization
- [ ] Preset system with era-appropriate names
- [ ] Dark mode variant (night stage lighting aesthetic)
