# GenX Design System — Implementation Guide

> A comprehensive guide for building a shared JUCE/C++ design system across all GenX plugins.
> This document describes the architecture, visual specifications, and step-by-step process
> for creating and integrating the `genx_ui` shared library.

---

## Table of Contents

1. [Overview](#overview)
2. [Directory Structure](#directory-structure)
3. [CMake Architecture](#cmake-architecture)
4. [Design Tokens](#design-tokens)
5. [Theme System](#theme-system)
6. [Font System](#font-system)
7. [LookAndFeel Implementation](#lookandfeel-implementation)
8. [Layout Utilities](#layout-utilities)
9. [Shared Components](#shared-components)
10. [Per-Plugin Integration](#per-plugin-integration)
11. [Figma Workflow](#figma-workflow)
12. [Migration Guide (GenX Delay)](#migration-guide)
13. [Creating a New Plugin](#creating-a-new-plugin)
14. [Visual Reference](#visual-reference)

---

## Overview

### Goals

- **Cohesive identity**: All GenX plugins share the same knob style, typography, spacing grid, and dark-panel aesthetic
- **Per-plugin personality**: Each plugin has a unique accent color (blue for delay, purple for reverb, etc.)
- **Reusable code**: Shared C++ library eliminates duplicate rendering code across plugins
- **Design-to-code pipeline**: Figma component library maps 1:1 to code tokens and components

### Aesthetic Direction

**Hybrid style** — modern flat layout with subtle hardware nods:
- Clean dark panels (not skeuomorphic, not entirely flat)
- Matte-finish rotary knobs with machined edge rings (realistic but not photorealistic)
- Accent-colored value arcs and pointer indicators
- Clean sans-serif typography (no retro segment fonts in the shared system)
- Inspired by: Neural DSP Archetype, Slate Digital, FabFilter Pro series

---

## Directory Structure

```
vst/
├── CMakeLists.txt                        # Top-level: fetches JUCE, orchestrates all targets
├── genx_ui/                              # Shared design system library
│   ├── CMakeLists.txt                    # Static lib + binary data for fonts
│   ├── genx_ui/                          # Source directory (namespaced include path)
│   │   ├── GenXTokens.h                  # Design tokens (colors, spacing, type scale)
│   │   ├── GenXTheme.h                   # Per-plugin theme configuration
│   │   ├── GenXLookAndFeel.h             # LookAndFeel class declaration
│   │   ├── GenXLookAndFeel.cpp           # Knob, toggle, combo, menu rendering
│   │   ├── GenXLayout.h                  # Layout context + grid utilities declaration
│   │   ├── GenXLayout.cpp                # Responsive grid + placement helpers
│   │   ├── GenXComponents.h              # HeaderBar, SectionCard, setup helpers declaration
│   │   ├── GenXComponents.cpp            # Shared component implementations
│   │   ├── GenXFonts.h                   # Font registry declaration
│   │   └── GenXFonts.cpp                 # Font loading from binary data
│   └── assets/
│       └── fonts/
│           ├── Inter-Variable.ttf        # UI labels, body text
│           ├── JetBrainsMono-Regular.ttf  # Numeric value readouts
│           └── BebasNeue-Regular.ttf      # Section headers, branding
├── plugins/
│   ├── genx_delay/                       # Links to genx_ui
│   ├── genx_filter/                      # Future: links to genx_ui
│   └── ...
└── docs/
    └── DESIGN_SYSTEM.md                  # This file
```

### Include Convention

Plugins include shared headers with the `genx_ui/` prefix:

```cpp
#include <genx_ui/GenXTokens.h>
#include <genx_ui/GenXTheme.h>
#include <genx_ui/GenXLookAndFeel.h>
#include <genx_ui/GenXLayout.h>
#include <genx_ui/GenXComponents.h>
#include <genx_ui/GenXFonts.h>
```

---

## CMake Architecture

### Top-Level `vst/CMakeLists.txt`

This file fetches JUCE once for the entire workspace and wires everything together.

```cmake
cmake_minimum_required(VERSION 3.22)
project(GenXPlugins VERSION 1.0.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# ── Fetch JUCE once for the entire workspace ──────────────────────────────
include(FetchContent)
FetchContent_Declare(
    JUCE
    GIT_REPOSITORY https://github.com/juce-framework/JUCE.git
    GIT_TAG        8.0.4
    GIT_SHALLOW    TRUE
)
FetchContent_MakeAvailable(JUCE)

# ── Shared UI library ─────────────────────────────────────────────────────
add_subdirectory(genx_ui)

# ── Plugins ───────────────────────────────────────────────────────────────
add_subdirectory(plugins/genx_delay)
# add_subdirectory(plugins/genx_filter)    # future
# add_subdirectory(plugins/genx_reverb)    # future
```

### Shared Library `genx_ui/CMakeLists.txt`

```cmake
# ── Shared font binary data ──────────────────────────────────────────────
juce_add_binary_data(GenXUIData
    NAMESPACE GenXBinaryData
    SOURCES
        assets/fonts/Inter-Variable.ttf
        assets/fonts/JetBrainsMono-Regular.ttf
        assets/fonts/BebasNeue-Regular.ttf
)

# ── Static library ───────────────────────────────────────────────────────
add_library(genx_ui STATIC
    genx_ui/GenXLookAndFeel.cpp
    genx_ui/GenXLayout.cpp
    genx_ui/GenXComponents.cpp
    genx_ui/GenXFonts.cpp
)

target_include_directories(genx_ui
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(genx_ui
    PUBLIC
        GenXUIData
        juce::juce_gui_basics
        juce::juce_audio_processors
)
```

### Plugin `CMakeLists.txt` (Example: GenX Delay)

Key changes from the current standalone setup:

```cmake
# Remove the standalone FetchContent for JUCE (now inherited from top-level)
# Remove juce_add_binary_data for shared fonts (now in genx_ui)

# Keep plugin-specific binary data if needed (plugin-specific icons, etc.)
# juce_add_binary_data(GenXDelayData SOURCES assets/icons/delay_icon.svg)

juce_add_plugin(GenXDelay
    PRODUCT_NAME "GenX Delay"
    COMPANY_NAME "GenX Audio"
    PLUGIN_MANUFACTURER_CODE Trwf
    PLUGIN_CODE GxDl
    FORMATS VST3 AU Standalone
    # ... other settings
)

target_sources(GenXDelay PRIVATE
    src/PluginProcessor.cpp
    src/PluginEditor.cpp
)

target_link_libraries(GenXDelay
    PRIVATE
        genx_ui                          # <-- shared design system
        juce::juce_audio_utils
        juce::juce_dsp
    PUBLIC
        juce::juce_recommended_config_flags
)
```

---

## Design Tokens

All visual constants live in `GenXTokens.h`. This is the single source of truth for colors, spacing, typography scale, and knob rendering parameters.

### `genx_ui/genx_ui/GenXTokens.h`

```cpp
#pragma once
#include <juce_gui_basics/juce_gui_basics.h>

namespace GenX::Tokens
{

//==============================================================================
// COLORS — Base palette shared across all plugins
//==============================================================================
namespace Color
{
    // Backgrounds — layered dark surfaces
    inline const juce::Colour bg0          {  16,  16,  20 };  // deepest background
    inline const juce::Colour bg1          {  24,  24,  28 };  // section card surface
    inline const juce::Colour bg2          {  32,  33,  38 };  // elevated/hover surface
    inline const juce::Colour bgOverlay    {  40,  42,  48 };  // popup/modal overlay

    // Text hierarchy
    inline const juce::Colour textPrimary  { 220, 222, 228 };  // high-contrast primary
    inline const juce::Colour textSecondary { 140, 144, 155 };  // labels, dimmed
    inline const juce::Colour textDisabled  {  75,  78,  88 };  // disabled state

    // Borders and dividers
    inline const juce::Colour border       {  52,  54,  62 };  // subtle panel borders
    inline const juce::Colour borderLight  {  68,  70,  78 };  // hover/focus borders

    // Knob hardware colors
    inline const juce::Colour knobSurface  {  38,  40,  46 };  // matte knob body
    inline const juce::Colour knobEdge     {  58,  60,  68 };  // machined edge ring
    inline const juce::Colour knobGrip     { 255, 255, 255 };  // grip lines (use at low alpha)
    inline const juce::Colour knobShadow   {   0,   0,   0 };  // drop shadow

    // Inactive arc on knobs
    inline const juce::Colour track        {  48,  50,  58 };  // background arc
}

//==============================================================================
// PER-PLUGIN ACCENT PRESETS
//==============================================================================
namespace Accent
{
    inline const juce::Colour delay        {  80, 160, 255 };  // cool blue
    inline const juce::Colour reverb       { 160, 100, 240 };  // purple
    inline const juce::Colour compressor   { 255, 140,  50 };  // warm orange
    inline const juce::Colour eq           {  80, 220, 160 };  // teal/green
    inline const juce::Colour filter       { 240,  80, 130 };  // pink/magenta
    inline const juce::Colour utility      { 200, 200, 210 };  // neutral silver
}

//==============================================================================
// TYPOGRAPHY SCALE — heights in design pixels (multiply by scale at runtime)
//==============================================================================
namespace Type
{
    inline constexpr float titleSize       = 20.0f;  // plugin name in header bar
    inline constexpr float sectionHeader   = 11.0f;  // "TIME", "MAIN", etc.
    inline constexpr float controlLabel    = 10.0f;  // knob labels
    inline constexpr float controlValue    = 11.0f;  // numeric value readouts
    inline constexpr float buttonText      = 10.0f;  // toggle/button labels
    inline constexpr float tooltipText     =  9.0f;  // tooltips
}

//==============================================================================
// SPACING — design pixels (multiply by scale at runtime)
//==============================================================================
namespace Space
{
    inline constexpr float margin          = 12.0f;  // outer edge padding
    inline constexpr float sectionGap      =  6.0f;  // between section cards
    inline constexpr float columnGap       =  6.0f;  // between grid columns
    inline constexpr float cardPadding     = 10.0f;  // inside a section card
    inline constexpr float controlGap      =  6.0f;  // between controls in a section
    inline constexpr float knobSize        = 56.0f;  // rotary knob diameter
    inline constexpr float valueHeight     = 16.0f;  // text box below knob
    inline constexpr float labelHeight     = 14.0f;  // label below value
    inline constexpr float headerHeight    = 20.0f;  // section header strip
    inline constexpr float toggleHeight    = 24.0f;  // toggle button row
    inline constexpr float headerBarHeight = 40.0f;  // top branding bar
}

//==============================================================================
// KNOB RENDERING CONSTANTS
//==============================================================================
namespace Knob
{
    inline constexpr float gripLineCount   = 20.0f;
    inline constexpr float gripAlpha       = 0.04f;  // very subtle grip lines
    inline constexpr float edgeThickness   = 1.2f;
    inline constexpr float arcThickness    = 2.5f;
    inline constexpr float arcGlowExtra    = 4.0f;   // extra width for glow layer under arc
    inline constexpr float pointerInner    = 0.3f;   // as fraction of radius
    inline constexpr float pointerOuter    = 0.72f;
    inline constexpr float pointerThickness= 2.0f;
    inline constexpr float hoverGlowAlpha  = 0.10f;
    inline constexpr float shadowOffset    = 1.5f;
    inline constexpr float shadowAlpha     = 0.4f;
}

//==============================================================================
// CORNER RADIUS
//==============================================================================
namespace Radius
{
    inline constexpr float card            = 4.0f;   // section card corners
    inline constexpr float button          = 3.0f;   // toggle button corners
    inline constexpr float popup           = 4.0f;   // popup menu corners
}

} // namespace GenX::Tokens
```

### Token Usage Rules

1. **Never hardcode colors in plugin code** — always reference `GenX::Tokens::Color::*` or `theme.accent`
2. **Always multiply spacing/type values by scale** — e.g., `GenX::Tokens::Space::knobSize * scale`
3. **Accent colors come from the theme**, not from tokens directly — the theme wraps an accent preset with auto-derived dim/glow variants

---

## Theme System

Each plugin creates a theme that configures its accent color and display name. The `GenX::LookAndFeel` takes this theme at construction and uses it for all accent-dependent rendering.

### `genx_ui/genx_ui/GenXTheme.h`

```cpp
#pragma once
#include <juce_gui_basics/juce_gui_basics.h>

namespace GenX
{

struct Theme
{
    juce::Colour accent;        // primary accent (value arcs, active toggles, pointer)
    juce::Colour accentDim;     // dimmed variant (inactive accent elements)
    juce::Colour accentGlow;    // lightened variant (glow layers behind arcs)
    juce::String displayName;   // "GENX DELAY" — shown in header bar
    juce::String shortName;     // "DELAY" — for tight spaces

    // Factory method with auto-derived colors
    static Theme create(const juce::Colour& accentColor,
                        const juce::String& displayName,
                        const juce::String& shortName)
    {
        return {
            accentColor,
            accentColor.withBrightness(
                accentColor.getBrightness() * 0.5f),
            accentColor.withBrightness(
                std::min(1.0f, accentColor.getBrightness() * 1.3f))
                .withSaturation(accentColor.getSaturation() * 0.7f),
            displayName,
            shortName
        };
    }
};

} // namespace GenX
```

### How Theme Flows Through the System

```
Plugin constructor
    └── GenX::Theme::create(GenX::Tokens::Accent::delay, "GENX DELAY", "DELAY")
            └── GenX::LookAndFeel(theme)
                    ├── drawRotarySlider() uses theme.accent for arc + pointer
                    ├── drawToggleButton() uses theme.accent for on-state
                    ├── drawComboBox() uses theme.accent for arrow
                    └── drawPopupMenuItem() uses theme.accent for highlight
            └── GenX::HeaderBar(theme)
                    └── paint() uses theme.displayName + theme.accent for underline
```

---

## Font System

Fonts are loaded once from binary data and accessed through a singleton registry.

### `genx_ui/genx_ui/GenXFonts.h`

```cpp
#pragma once
#include <juce_gui_basics/juce_gui_basics.h>

namespace GenX
{

class Fonts
{
public:
    // Singleton access — fonts are loaded on first call
    static Fonts& instance();

    // Base fonts at design-pixel sizes (from GenX::Tokens::Type)
    juce::Font title()   const;  // BebasNeue at titleSize
    juce::Font header()  const;  // BebasNeue at sectionHeader
    juce::Font label()   const;  // Inter at controlLabel
    juce::Font value()   const;  // JetBrainsMono at controlValue
    juce::Font button()  const;  // Inter at buttonText

    // Scaled versions — multiply design size by scale factor
    juce::Font title(float scale)   const;
    juce::Font header(float scale)  const;
    juce::Font label(float scale)   const;
    juce::Font value(float scale)   const;
    juce::Font button(float scale)  const;

private:
    Fonts();  // private — loads typefaces from GenXBinaryData

    juce::Typeface::Ptr bebasTypeface;   // BebasNeue-Regular
    juce::Typeface::Ptr interTypeface;   // Inter-Variable
    juce::Typeface::Ptr monoTypeface;    // JetBrainsMono-Regular
};

} // namespace GenX
```

### `genx_ui/genx_ui/GenXFonts.cpp`

```cpp
#include "GenXFonts.h"
#include "GenXTokens.h"
#include <BinaryData.h>  // Generated by juce_add_binary_data (GenXUIData target)

namespace GenX
{

Fonts& Fonts::instance()
{
    static Fonts fonts;
    return fonts;
}

Fonts::Fonts()
{
    // Load typefaces from compiled binary data
    bebasTypeface = juce::Typeface::createSystemTypefaceFor(
        GenXBinaryData::BebasNeueRegular_ttf,
        GenXBinaryData::BebasNeueRegular_ttfSize);

    interTypeface = juce::Typeface::createSystemTypefaceFor(
        GenXBinaryData::InterVariable_ttf,
        GenXBinaryData::InterVariable_ttfSize);

    monoTypeface = juce::Typeface::createSystemTypefaceFor(
        GenXBinaryData::JetBrainsMonoRegular_ttf,
        GenXBinaryData::JetBrainsMonoRegular_ttfSize);
}

// Base fonts at design size
juce::Font Fonts::title() const
{
    return juce::Font(juce::FontOptions(bebasTypeface))
        .withHeight(Tokens::Type::titleSize);
}

juce::Font Fonts::header() const
{
    return juce::Font(juce::FontOptions(bebasTypeface))
        .withHeight(Tokens::Type::sectionHeader);
}

juce::Font Fonts::label() const
{
    return juce::Font(juce::FontOptions(interTypeface))
        .withHeight(Tokens::Type::controlLabel);
}

juce::Font Fonts::value() const
{
    return juce::Font(juce::FontOptions(monoTypeface))
        .withHeight(Tokens::Type::controlValue);
}

juce::Font Fonts::button() const
{
    return juce::Font(juce::FontOptions(interTypeface))
        .withHeight(Tokens::Type::buttonText);
}

// Scaled versions
juce::Font Fonts::title(float scale) const
{
    return juce::Font(juce::FontOptions(bebasTypeface))
        .withHeight(Tokens::Type::titleSize * scale);
}

juce::Font Fonts::header(float scale) const
{
    return juce::Font(juce::FontOptions(bebasTypeface))
        .withHeight(Tokens::Type::sectionHeader * scale);
}

juce::Font Fonts::label(float scale) const
{
    return juce::Font(juce::FontOptions(interTypeface))
        .withHeight(Tokens::Type::controlLabel * scale);
}

juce::Font Fonts::value(float scale) const
{
    return juce::Font(juce::FontOptions(monoTypeface))
        .withHeight(Tokens::Type::controlValue * scale);
}

juce::Font Fonts::button(float scale) const
{
    return juce::Font(juce::FontOptions(interTypeface))
        .withHeight(Tokens::Type::buttonText * scale);
}

} // namespace GenX
```

### Font Choices Rationale

| Font | Role | Why |
|------|------|-----|
| **Inter** | Labels, buttons, body text | Industry-standard UI font. Highly readable at small sizes. Open source (SIL OFL). |
| **JetBrains Mono** | Numeric value readouts | Clean monospaced font. Numbers align perfectly. Technical feel without being retro. |
| **BebasNeue** | Headers, branding | Already in the project. Bold condensed geometric — excellent for all-caps section headers and "GENX" branding. |

---

## LookAndFeel Implementation

The `GenX::LookAndFeel` class is the core visual engine. It extends `juce::LookAndFeel_V4` and renders all standard JUCE components in the GenX style.

### `genx_ui/genx_ui/GenXLookAndFeel.h`

```cpp
#pragma once
#include <juce_gui_basics/juce_gui_basics.h>
#include "GenXTheme.h"

namespace GenX
{

class LookAndFeel : public juce::LookAndFeel_V4
{
public:
    explicit LookAndFeel(const Theme& theme);
    ~LookAndFeel() override = default;

    const Theme& getTheme() const { return theme; }

    // ── Overrides ────────────────────────────────────────────────────────
    void drawRotarySlider(juce::Graphics& g, int x, int y, int width,
                          int height, float sliderPos, float startAngle,
                          float endAngle, juce::Slider& slider) override;

    void drawToggleButton(juce::Graphics& g, juce::ToggleButton& button,
                          bool shouldDrawButtonAsHighlighted,
                          bool shouldDrawButtonAsDown) override;

    void drawComboBox(juce::Graphics& g, int width, int height,
                      bool isButtonDown, int bx, int by, int bw, int bh,
                      juce::ComboBox& box) override;

    void drawPopupMenuBackground(juce::Graphics& g, int width,
                                 int height) override;

    void drawPopupMenuItem(juce::Graphics& g, const juce::Rectangle<int>& area,
                           bool isSeparator, bool isActive, bool isHighlighted,
                           bool isTicked, bool hasSubMenu,
                           const juce::String& text,
                           const juce::String& shortcutKeyText,
                           const juce::Drawable* icon,
                           const juce::Colour* textColour) override;

    void drawLabel(juce::Graphics& g, juce::Label& label) override;

    void drawButtonText(juce::Graphics& g, juce::TextButton& button,
                        bool shouldDrawButtonAsHighlighted,
                        bool shouldDrawButtonAsDown) override;

    juce::Font getComboBoxFont(juce::ComboBox& box) override;
    juce::Font getPopupMenuFont() override;

private:
    Theme theme;
};

} // namespace GenX
```

### Rotary Knob Rendering — Layer by Layer

The knob is the most important visual element. Here is the 9-layer rendering approach for the new aesthetic:

```cpp
void LookAndFeel::drawRotarySlider(juce::Graphics& g, int x, int y,
    int width, int height, float sliderPos, float startAngle,
    float endAngle, juce::Slider& slider)
{
    using namespace Tokens;

    const float enabledAlpha = slider.isEnabled() ? 1.0f : 0.3f;
    const auto bounds = juce::Rectangle<int>(x, y, width, height).toFloat();
    const float diameter = std::min(bounds.getWidth(), bounds.getHeight());
    const float radius = diameter * 0.5f;
    const auto centre = bounds.getCentre();

    const float angle = startAngle + sliderPos * (endAngle - startAngle);

    // ── Layer 1: Hover glow halo ─────────────────────────────────────
    // Subtle radial gradient in the accent color, visible on hover
    if (slider.isMouseOver())
    {
        juce::ColourGradient glow(
            theme.accentGlow.withAlpha(Knob::hoverGlowAlpha * enabledAlpha),
            centre,
            theme.accentGlow.withAlpha(0.0f),
            centre.translated(radius * 1.3f, 0),
            true);  // radial
        g.setGradientFill(glow);
        g.fillEllipse(bounds.expanded(radius * 0.3f));
    }

    // ── Layer 2: Drop shadow ─────────────────────────────────────────
    g.setColour(Color::knobShadow.withAlpha(Knob::shadowAlpha * enabledAlpha));
    g.fillEllipse(bounds.reduced(diameter * 0.12f)
        .translated(Knob::shadowOffset, Knob::shadowOffset));

    // ── Layer 3: Knob body (matte dark gradient) ─────────────────────
    {
        const auto knobBounds = bounds.reduced(diameter * 0.14f);
        juce::ColourGradient bodyGrad(
            Color::knobSurface.brighter(0.08f).withAlpha(enabledAlpha),
            knobBounds.getX(), knobBounds.getY(),
            Color::knobSurface.darker(0.05f).withAlpha(enabledAlpha),
            knobBounds.getRight(), knobBounds.getBottom(),
            false);
        g.setGradientFill(bodyGrad);
        g.fillEllipse(knobBounds);
    }

    // ── Layer 4: Grip lines ──────────────────────────────────────────
    {
        const float gripRadius = radius * 0.62f;
        g.setColour(Color::knobGrip.withAlpha(Knob::gripAlpha * enabledAlpha));
        for (int i = 0; i < (int)Knob::gripLineCount; ++i)
        {
            float a = (float)i / Knob::gripLineCount * juce::MathConstants<float>::twoPi;
            float inner = gripRadius * 0.7f;
            g.drawLine(
                centre.x + std::cos(a) * inner,
                centre.y + std::sin(a) * inner,
                centre.x + std::cos(a) * gripRadius,
                centre.y + std::sin(a) * gripRadius,
                0.5f);
        }
    }

    // ── Layer 5: Edge ring ───────────────────────────────────────────
    {
        const auto ringBounds = bounds.reduced(diameter * 0.14f);
        g.setColour(Color::knobEdge.withAlpha(enabledAlpha));
        g.drawEllipse(ringBounds, Knob::edgeThickness);
    }

    // ── Layer 6: Track arc (background/inactive) ─────────────────────
    {
        juce::Path trackArc;
        trackArc.addCentredArc(centre.x, centre.y, radius * 0.82f,
            radius * 0.82f, 0.0f, startAngle, endAngle, true);
        g.setColour(Color::track.withAlpha(0.4f * enabledAlpha));
        g.strokePath(trackArc,
            juce::PathStrokeType(Knob::arcThickness,
                juce::PathStrokeType::curved,
                juce::PathStrokeType::rounded));
    }

    // ── Layer 7: Value arc + glow ────────────────────────────────────
    if (sliderPos > 0.0f)
    {
        juce::Path valueArc;
        valueArc.addCentredArc(centre.x, centre.y, radius * 0.82f,
            radius * 0.82f, 0.0f, startAngle, angle, true);

        // Glow layer (wider, low alpha)
        g.setColour(theme.accentGlow.withAlpha(0.15f * enabledAlpha));
        g.strokePath(valueArc,
            juce::PathStrokeType(Knob::arcThickness + Knob::arcGlowExtra,
                juce::PathStrokeType::curved,
                juce::PathStrokeType::rounded));

        // Main arc
        g.setColour(theme.accent.withAlpha(0.9f * enabledAlpha));
        g.strokePath(valueArc,
            juce::PathStrokeType(Knob::arcThickness,
                juce::PathStrokeType::curved,
                juce::PathStrokeType::rounded));
    }

    // ── Layer 8: Pointer indicator ───────────────────────────────────
    {
        juce::Path pointer;
        float pointerInnerR = radius * Knob::pointerInner;
        float pointerOuterR = radius * Knob::pointerOuter;
        pointer.addLineSegment(juce::Line<float>(
            centre.x + std::sin(angle) * pointerInnerR,
            centre.y - std::cos(angle) * pointerInnerR,
            centre.x + std::sin(angle) * pointerOuterR,
            centre.y - std::cos(angle) * pointerOuterR),
            0.0f);
        g.setColour(theme.accent.withAlpha(enabledAlpha));
        g.strokePath(pointer,
            juce::PathStrokeType(Knob::pointerThickness,
                juce::PathStrokeType::curved,
                juce::PathStrokeType::rounded));
    }

    // ── Layer 9: Center cap ──────────────────────────────────────────
    {
        const float capSize = diameter * 0.12f;
        g.setColour(Color::bg0.withAlpha(enabledAlpha));
        g.fillEllipse(centre.x - capSize, centre.y - capSize,
            capSize * 2.0f, capSize * 2.0f);
    }
}
```

### Toggle Button Rendering

```cpp
void LookAndFeel::drawToggleButton(juce::Graphics& g,
    juce::ToggleButton& button, bool highlighted, bool down)
{
    using namespace Tokens;

    auto bounds = button.getLocalBounds().toFloat().reduced(1.0f);
    const float enabledAlpha = button.isEnabled() ? 1.0f : 0.3f;
    const bool isOn = button.getToggleState();

    // Background
    if (isOn)
    {
        g.setColour(theme.accent.withAlpha(0.85f * enabledAlpha));
        g.fillRoundedRectangle(bounds, Radius::button);
    }
    else
    {
        g.setColour(Color::bg2.withAlpha(enabledAlpha));
        g.fillRoundedRectangle(bounds, Radius::button);
        g.setColour(Color::border.withAlpha(enabledAlpha));
        g.drawRoundedRectangle(bounds, Radius::button, 1.0f);
    }

    // Hover highlight
    if (highlighted && button.isEnabled())
    {
        g.setColour(juce::Colours::white.withAlpha(0.05f));
        g.fillRoundedRectangle(bounds, Radius::button);
    }

    // Text
    g.setColour(isOn
        ? juce::Colours::white.withAlpha(enabledAlpha)
        : Color::textSecondary.withAlpha(enabledAlpha));
    g.setFont(Fonts::instance().button());
    g.drawText(button.getButtonText(), bounds, juce::Justification::centred);
}
```

### Combo Box and Popup Menu

```cpp
void LookAndFeel::drawComboBox(juce::Graphics& g, int width, int height,
    bool isButtonDown, int, int, int, int, juce::ComboBox& box)
{
    using namespace Tokens;

    auto bounds = juce::Rectangle<float>(0, 0, (float)width, (float)height);
    g.setColour(Color::bg1);
    g.fillRoundedRectangle(bounds, Radius::button);
    g.setColour(Color::border);
    g.drawRoundedRectangle(bounds, Radius::button, 1.0f);

    // Dropdown arrow in accent color
    const float arrowSize = height * 0.3f;
    const float arrowX = width - height * 0.5f;
    const float arrowY = height * 0.5f;
    juce::Path arrow;
    arrow.addTriangle(
        arrowX - arrowSize * 0.5f, arrowY - arrowSize * 0.25f,
        arrowX + arrowSize * 0.5f, arrowY - arrowSize * 0.25f,
        arrowX, arrowY + arrowSize * 0.25f);
    g.setColour(theme.accent);
    g.fillPath(arrow);
}

void LookAndFeel::drawPopupMenuBackground(juce::Graphics& g, int width, int height)
{
    g.fillAll(Tokens::Color::bgOverlay);
    g.setColour(Tokens::Color::border);
    g.drawRect(0, 0, width, height, 1);
}

void LookAndFeel::drawPopupMenuItem(juce::Graphics& g,
    const juce::Rectangle<int>& area, bool isSeparator, bool isActive,
    bool isHighlighted, bool isTicked, bool hasSubMenu,
    const juce::String& text, const juce::String&,
    const juce::Drawable*, const juce::Colour*)
{
    using namespace Tokens;

    if (isSeparator)
    {
        g.setColour(Color::border);
        g.drawHorizontalLine(area.getCentreY(), area.getX() + 4.0f,
            area.getRight() - 4.0f);
        return;
    }

    if (isHighlighted && isActive)
    {
        g.setColour(theme.accent.withAlpha(0.2f));
        g.fillRect(area);
        g.setColour(Color::textPrimary);
    }
    else
    {
        g.setColour(isActive ? Color::textPrimary : Color::textDisabled);
    }

    g.setFont(Fonts::instance().label());
    g.drawText(text, area.reduced(8, 0), juce::Justification::centredLeft);
}
```

---

## Layout Utilities

### `genx_ui/genx_ui/GenXLayout.h`

```cpp
#pragma once
#include <juce_gui_basics/juce_gui_basics.h>
#include <vector>

namespace GenX
{

//==============================================================================
// LayoutContext — computed from window dimensions
//==============================================================================
struct LayoutContext
{
    float scale;       // multiplier for all design-pixel values
    int columns;       // responsive column count (1, 2, or 3)
    int margin;        // scaled outer margin
    int sectionGap;    // scaled gap between section cards
    int columnGap;     // scaled gap between columns

    // Compute from window size and reference dimensions
    static LayoutContext compute(int windowWidth, int windowHeight,
                                 int refWidth = 800, int refHeight = 580)
    {
        float w = (float)windowWidth;
        float h = (float)windowHeight;
        float s = std::min(w / (float)refWidth, h / (float)refHeight);

        int cols = 1;
        if (w >= 560.0f) cols = 3;
        else if (w >= 380.0f) cols = 2;

        return {
            s,
            cols,
            (int)(Tokens::Space::margin * s),
            (int)(Tokens::Space::sectionGap * s),
            (int)(Tokens::Space::columnGap * s)
        };
    }
};

//==============================================================================
// SectionGrid — positions section cards in a responsive grid
//==============================================================================
class SectionGrid
{
public:
    struct Section
    {
        int index;
        juce::Rectangle<int> bounds;
    };

    // Given section height estimates, compute bounds in a multi-column grid.
    // Heights should be pre-scaled (in actual pixels, not design pixels).
    // The area is the available rectangle below the header bar.
    static std::vector<Section> layout(
        const LayoutContext& ctx,
        juce::Rectangle<int> area,
        const std::vector<int>& sectionHeights);
};

//==============================================================================
// Placement helpers
//==============================================================================
namespace Place
{
    // Centers a rotary knob + value label stack within the given area.
    // The slider gets the top portion (knobSize x knobSize),
    // the label gets the bottom portion.
    void knob(juce::Slider& slider, juce::Label& label,
              juce::Rectangle<int> area, float scale);

    // Places a toggle button in a row area.
    void toggle(juce::ToggleButton& button,
                juce::Rectangle<int> row, float scale);

    // Places a combo box in a row area.
    void comboBox(juce::ComboBox& box,
                  juce::Rectangle<int> row, float scale);
}

} // namespace GenX
```

### Grid Algorithm

The `SectionGrid::layout()` method distributes sections across columns, balancing total height:

```cpp
std::vector<SectionGrid::Section> SectionGrid::layout(
    const LayoutContext& ctx,
    juce::Rectangle<int> area,
    const std::vector<int>& sectionHeights)
{
    std::vector<Section> result;
    const int numSections = (int)sectionHeights.size();

    if (ctx.columns == 1)
    {
        // Stack vertically
        int y = area.getY();
        for (int i = 0; i < numSections; ++i)
        {
            result.push_back({
                i,
                { area.getX(), y, area.getWidth(), sectionHeights[i] }
            });
            y += sectionHeights[i] + ctx.sectionGap;
        }
    }
    else
    {
        // Distribute sections across columns, filling left-to-right, top-to-bottom
        int colWidth = (area.getWidth() - (ctx.columns - 1) * ctx.columnGap)
                       / ctx.columns;

        // Track Y position per column
        std::vector<int> colY(ctx.columns, area.getY());

        for (int i = 0; i < numSections; ++i)
        {
            // Find column with least total height (greedy balancing)
            int bestCol = 0;
            for (int c = 1; c < ctx.columns; ++c)
            {
                if (colY[c] < colY[bestCol])
                    bestCol = c;
            }

            int colX = area.getX() + bestCol * (colWidth + ctx.columnGap);
            result.push_back({
                i,
                { colX, colY[bestCol], colWidth, sectionHeights[i] }
            });
            colY[bestCol] += sectionHeights[i] + ctx.sectionGap;
        }
    }

    return result;
}
```

---

## Shared Components

### `genx_ui/genx_ui/GenXComponents.h`

```cpp
#pragma once
#include <juce_gui_basics/juce_gui_basics.h>
#include "GenXTheme.h"

namespace GenX
{

//==============================================================================
// HeaderBar — branded top strip for every plugin
//==============================================================================
class HeaderBar : public juce::Component
{
public:
    explicit HeaderBar(const Theme& theme);
    void paint(juce::Graphics& g) override;

private:
    Theme theme;
};

//==============================================================================
// SectionCard — draws bg, border, and header text for a control group
//==============================================================================
class SectionCard : public juce::Component
{
public:
    explicit SectionCard(const juce::String& title);
    void paint(juce::Graphics& g) override;

    // Returns the area below the header, inside padding, for child controls
    juce::Rectangle<int> getContentArea() const;

private:
    juce::String title;
};

//==============================================================================
// Setup helpers — configure standard JUCE components with GenX styling
//==============================================================================

// Configures a Slider as a rotary knob with a paired Label
void setupKnob(juce::Slider& slider, juce::Label& label,
               const juce::String& labelText);

// Configures a ToggleButton with GenX styling
void setupToggle(juce::ToggleButton& toggle);

// Configures a ComboBox with GenX styling
void setupComboBox(juce::ComboBox& box);

} // namespace GenX
```

### HeaderBar Implementation

```cpp
GenX::HeaderBar::HeaderBar(const Theme& t) : theme(t) {}

void GenX::HeaderBar::paint(juce::Graphics& g)
{
    auto bounds = getLocalBounds().toFloat();
    float scale = bounds.getHeight() / Tokens::Space::headerBarHeight;

    // Dark header background (slightly darker than bg0)
    g.setColour(Tokens::Color::bg0.darker(0.15f));
    g.fillRect(bounds);

    // Plugin name
    g.setFont(Fonts::instance().title(scale));
    g.setColour(Tokens::Color::textPrimary);
    g.drawText(theme.displayName,
        bounds.reduced(Tokens::Space::margin * scale, 0),
        juce::Justification::centredLeft);

    // Accent-colored underline
    float lineY = bounds.getBottom() - 2.0f;
    float nameWidth = Fonts::instance().title(scale)
        .getStringWidthFloat(theme.displayName);
    float lineX = Tokens::Space::margin * scale;
    g.setColour(theme.accent);
    g.drawLine(lineX, lineY, lineX + nameWidth, lineY, 2.0f);

    // Bottom border
    g.setColour(Tokens::Color::border);
    g.drawHorizontalLine((int)bounds.getBottom() - 1,
        bounds.getX(), bounds.getRight());
}
```

### SectionCard Implementation

```cpp
GenX::SectionCard::SectionCard(const juce::String& t) : title(t) {}

void GenX::SectionCard::paint(juce::Graphics& g)
{
    auto bounds = getLocalBounds().toFloat();

    // Card background
    g.setColour(Tokens::Color::bg1);
    g.fillRoundedRectangle(bounds, Tokens::Radius::card);

    // Card border
    g.setColour(Tokens::Color::border);
    g.drawRoundedRectangle(bounds, Tokens::Radius::card, 1.0f);

    // Section header text (uppercase)
    auto headerArea = bounds.removeFromTop(Tokens::Space::headerHeight);
    g.setFont(Fonts::instance().header());
    g.setColour(Tokens::Color::textSecondary);
    g.drawText(title.toUpperCase(),
        headerArea.reduced(Tokens::Space::cardPadding, 0),
        juce::Justification::centredLeft);
}

juce::Rectangle<int> GenX::SectionCard::getContentArea() const
{
    return getLocalBounds()
        .withTrimmedTop((int)Tokens::Space::headerHeight)
        .reduced((int)Tokens::Space::cardPadding);
}
```

### Setup Helpers

```cpp
void GenX::setupKnob(juce::Slider& slider, juce::Label& label,
                     const juce::String& labelText)
{
    slider.setSliderStyle(juce::Slider::RotaryHorizontalVerticalDrag);
    slider.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 60, 16);
    slider.setColour(juce::Slider::textBoxTextColourId,
                     Tokens::Color::textPrimary);
    slider.setColour(juce::Slider::textBoxOutlineColourId,
                     juce::Colours::transparentBlack);

    label.setText(labelText, juce::dontSendNotification);
    label.setJustificationType(juce::Justification::centred);
    label.setFont(Fonts::instance().label());
    label.setColour(juce::Label::textColourId,
                    Tokens::Color::textSecondary);
}

void GenX::setupToggle(juce::ToggleButton& toggle)
{
    toggle.setClickingTogglesState(true);
}

void GenX::setupComboBox(juce::ComboBox& box)
{
    box.setColour(juce::ComboBox::textColourId,
                  Tokens::Color::textPrimary);
    box.setColour(juce::ComboBox::backgroundColourId,
                  Tokens::Color::bg1);
    box.setColour(juce::ComboBox::outlineColourId,
                  Tokens::Color::border);
}
```

---

## Per-Plugin Integration

### How a Plugin Uses the Design System

Every GenX plugin follows this pattern:

```cpp
// PluginEditor.h
#include <genx_ui/GenXLookAndFeel.h>
#include <genx_ui/GenXTheme.h>
#include <genx_ui/GenXLayout.h>
#include <genx_ui/GenXComponents.h>

class MyPluginEditor : public juce::AudioProcessorEditor
{
public:
    MyPluginEditor(MyPluginProcessor& p);
    ~MyPluginEditor() override;

    void paint(juce::Graphics& g) override;
    void resized() override;

private:
    MyPluginProcessor& processorRef;

    // Design system
    std::unique_ptr<GenX::LookAndFeel> lnf;
    GenX::HeaderBar headerBar;
    GenX::SectionCard timeSection { "Time" };
    GenX::SectionCard mainSection { "Main" };
    // ... more sections

    // Controls
    juce::Slider delayTimeSlider;
    juce::Label  delayTimeLabel;
    // ... more controls

    // Attachments
    using SliderAttachment = juce::AudioProcessorValueTreeState::SliderAttachment;
    std::unique_ptr<SliderAttachment> delayTimeAttachment;
    // ... more attachments
};
```

```cpp
// PluginEditor.cpp
MyPluginEditor::MyPluginEditor(MyPluginProcessor& p)
    : AudioProcessorEditor(&p),
      processorRef(p),
      headerBar(GenX::Theme::create(
          GenX::Tokens::Accent::delay, "GENX DELAY", "DELAY"))
{
    // Create theme and LookAndFeel
    auto theme = GenX::Theme::create(
        GenX::Tokens::Accent::delay, "GENX DELAY", "DELAY");
    lnf = std::make_unique<GenX::LookAndFeel>(theme);
    setLookAndFeel(lnf.get());

    // Add header bar
    addAndMakeVisible(headerBar);

    // Add section cards
    addAndMakeVisible(timeSection);
    addAndMakeVisible(mainSection);

    // Setup controls using shared helpers
    GenX::setupKnob(delayTimeSlider, delayTimeLabel, "Delay Time");
    addAndMakeVisible(delayTimeSlider);
    addAndMakeVisible(delayTimeLabel);

    // Bind to parameters
    delayTimeAttachment = std::make_unique<SliderAttachment>(
        processorRef.apvts, "delayTime", delayTimeSlider);

    // Window setup
    setSize(800, 580);
    setResizable(true, true);
    if (auto* c = getConstrainer())
    {
        c->setFixedAspectRatio(800.0 / 580.0);
        setResizeLimits(760, 552, 1520, 1100);
    }
}

MyPluginEditor::~MyPluginEditor()
{
    setLookAndFeel(nullptr);
}

void MyPluginEditor::paint(juce::Graphics& g)
{
    // Clean dark background — that's it. No textures, no vignettes.
    g.fillAll(GenX::Tokens::Color::bg0);
}

void MyPluginEditor::resized()
{
    auto area = getLocalBounds();
    auto ctx = GenX::LayoutContext::compute(getWidth(), getHeight());

    // Header bar at top
    headerBar.setBounds(area.removeFromTop(
        (int)(GenX::Tokens::Space::headerBarHeight * ctx.scale)));

    // Content area below header
    area.reduce(ctx.margin, ctx.margin);

    // Compute section heights (each section estimates its own height)
    std::vector<int> heights = {
        (int)(120 * ctx.scale),  // TIME section
        (int)(180 * ctx.scale),  // MAIN section
        // ... etc
    };

    // Layout sections in responsive grid
    auto sections = GenX::SectionGrid::layout(ctx, area, heights);

    for (auto& s : sections)
    {
        switch (s.index)
        {
            case 0:
                timeSection.setBounds(s.bounds);
                // Place controls within section content area
                GenX::Place::knob(delayTimeSlider, delayTimeLabel,
                    timeSection.getContentArea(), ctx.scale);
                break;
            case 1:
                mainSection.setBounds(s.bounds);
                // ...
                break;
        }
    }
}
```

---

## Figma Workflow

### Setting Up the Figma File

1. **Create a new Figma file**: "GenX Design System"
2. **Create pages**: Tokens, Components, GenX Delay, GenX Filter (future), Responsive

### Defining Figma Variables

Use Figma's **Variables** feature (beta) to mirror the code tokens:

| Collection | Variable | Value | Maps to |
|-----------|----------|-------|---------|
| Colors/Base | `bg0` | #101014 | `GenX::Tokens::Color::bg0` |
| Colors/Base | `bg1` | #18181C | `GenX::Tokens::Color::bg1` |
| Colors/Base | `bg2` | #202126 | `GenX::Tokens::Color::bg2` |
| Colors/Base | `text-primary` | #DCDEE4 | `GenX::Tokens::Color::textPrimary` |
| Colors/Base | `text-secondary` | #8C909B | `GenX::Tokens::Color::textSecondary` |
| Colors/Base | `border` | #34363E | `GenX::Tokens::Color::border` |
| Colors/Base | `knob-surface` | #26282E | `GenX::Tokens::Color::knobSurface` |
| Colors/Base | `track` | #30323A | `GenX::Tokens::Color::track` |
| Colors/Accent | `accent` | (mode-dependent) | `theme.accent` |
| Colors/Accent | `accent-dim` | (mode-dependent) | `theme.accentDim` |
| Colors/Accent | `accent-glow` | (mode-dependent) | `theme.accentGlow` |
| Spacing | `margin` | 12 | `GenX::Tokens::Space::margin` |
| Spacing | `knob-size` | 56 | `GenX::Tokens::Space::knobSize` |
| Spacing | `section-gap` | 6 | `GenX::Tokens::Space::sectionGap` |

### Accent Color Modes

Create **modes** in the Accent color collection:

| Mode | `accent` | `accent-dim` | `accent-glow` |
|------|----------|-------------|---------------|
| Delay | #50A0FF | #285080 | #80C0FF |
| Reverb | #A064F0 | #503278 | #C090FF |
| Compressor | #FF8C32 | #804619 | #FFB070 |
| EQ | #50DCA0 | #286E50 | #80F0C0 |
| Filter | #F05082 | #782841 | #FF80A0 |

Switch a frame to "Delay" mode and all accent-bound elements update instantly.

### Building Figma Components

Create these as Figma components with auto-layout:

1. **Rotary Knob**: Circle (knob-surface fill) + arc stroke (accent) + pointer line (accent) + value text below (mono font) + label below (Inter). Use component properties for label text and value text.

2. **Toggle Button**: Rounded rectangle. Two variants: Off (bg2 fill, border stroke, text-secondary label) and On (accent fill, white label).

3. **Combo Box**: Rounded rectangle (bg1 fill, border stroke) + text (text-primary) + triangle arrow (accent).

4. **Section Card**: Rounded rectangle (bg1 fill, border stroke, card radius corners) + header text (BebasNeue, text-secondary, uppercase) + content area below.

5. **Header Bar**: Full-width rectangle (bg0 darker) + title text (BebasNeue, text-primary) + accent underline + bottom border.

### Design-to-Code Mapping Table

| Figma Element | Figma Property | Code Reference |
|--------------|----------------|----------------|
| Any background fill | `bg0` variable | `GenX::Tokens::Color::bg0` |
| Section card fill | `bg1` variable | `GenX::Tokens::Color::bg1` |
| Active arc stroke | `accent` variable | `theme.accent` |
| Knob body fill | `knob-surface` variable | `GenX::Tokens::Color::knobSurface` |
| Label text | Inter 10px, `text-secondary` | `GenX::Fonts::instance().label()` |
| Value readout text | JetBrains Mono 11px, `text-primary` | `GenX::Fonts::instance().value()` |
| Header text | BebasNeue 11px, `text-secondary` | `GenX::Fonts::instance().header()` |
| Title text | BebasNeue 20px, `text-primary` | `GenX::Fonts::instance().title()` |
| Spacing between sections | `section-gap` variable | `GenX::Tokens::Space::sectionGap` |
| Knob diameter | `knob-size` variable | `GenX::Tokens::Space::knobSize` |

### Workflow Process

1. **Design in Figma** — Create or modify mockup using the component library and variables
2. **Extract measurements** — Note any new spacing, sizes, or colors used
3. **Update tokens** — If new values are needed, update `GenXTokens.h` first
4. **Implement in code** — Build the layout using `GenX::LayoutContext`, `SectionGrid`, and `Place` helpers
5. **Visual QA** — Compare running plugin side-by-side with Figma mockup

---

## Migration Guide

### Migrating GenX Delay from PioneerLookAndFeel to GenX Design System

This is the recommended first migration to validate the design system.

#### Step 1: Create the genx_ui library

Set up the full directory structure, CMake files, and all source files as described above.

#### Step 2: Update GenX Delay CMakeLists.txt

- Remove the standalone `FetchContent` block for JUCE (use top-level)
- Remove `juce_add_binary_data(GenXDelayData ...)` for shared fonts
- Keep any plugin-specific binary data if needed
- Add `target_link_libraries(GenXDelay PRIVATE genx_ui)`

#### Step 3: Refactor PluginEditor.h

**Remove:**
- `PioneerColors` namespace (entire block, ~15 lines)
- `PioneerLookAndFeel` class declaration (entire block, ~30 lines)
- Font member variables (`titleFont`, `headerFont`, `bodyFont`, `numFont`)
- `drawVFDText()` method declaration
- `sectionColor` member

**Add:**
- `#include <genx_ui/GenXLookAndFeel.h>` and other genx_ui headers
- `std::unique_ptr<GenX::LookAndFeel> lnf;`
- `GenX::HeaderBar headerBar;`
- `GenX::SectionCard` members for each section

#### Step 4: Refactor PluginEditor.cpp

**Constructor:**
- Replace `PioneerLookAndFeel` setup with `GenX::Theme::create()` + `GenX::LookAndFeel`
- Replace font loading block with `GenX::Fonts::instance()`
- Replace `setupSlider()` calls with `GenX::setupKnob()` etc.

**paint():**
- Replace brushed metal texture with `g.fillAll(GenX::Tokens::Color::bg0)`
- Remove display window bevel rendering
- Remove VFD title text rendering (now handled by `HeaderBar`)
- Remove section border drawing (now handled by `SectionCard`)
- Remove vignette edge darkening
- The `paint()` method becomes essentially one line: `g.fillAll(bg0)`

**resized():**
- Replace manual section bounds calculation with `GenX::LayoutContext::compute()` + `GenX::SectionGrid::layout()`
- Use `GenX::Place::knob()` instead of manual `placeKnob()` lambda

**layoutSection():**
- Refactor to work with `SectionCard::getContentArea()` instead of raw bounds

#### Step 5: Build and Test

```bash
cd /Users/trwolf/Repos/vst
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel
```

Load the built VST3/AU in a DAW and verify:
- Plugin opens without crash
- All controls functional (knobs, toggles, combo)
- Parameter automation works
- Window resize respects aspect ratio
- Visual appearance matches the new aesthetic (blue accent, dark panels)

---

## Creating a New Plugin

### Checklist for a New GenX Plugin

1. **Create plugin directory**: `plugins/genx_<name>/`
2. **Create CMakeLists.txt**: Use the template above, link to `genx_ui`
3. **Choose accent color**: Pick from `GenX::Tokens::Accent::*` or define custom
4. **Create PluginProcessor**: Standard JUCE AudioProcessor with APVTS
5. **Create PluginEditor**:
   - Create `GenX::Theme` with your accent + display name
   - Create `GenX::LookAndFeel` with the theme
   - Use `GenX::HeaderBar` for branding
   - Use `GenX::SectionCard` for control groups
   - Use `GenX::setupKnob/Toggle/ComboBox` for controls
   - Use `GenX::LayoutContext` + `SectionGrid` in `resized()`
6. **Add to top-level CMakeLists.txt**: `add_subdirectory(plugins/genx_<name>)`
7. **Create Figma mockup**: Use the component library, switch to your accent mode
8. **Visual QA**: Compare running plugin with Figma mockup

### Minimal Example — GenX Filter

```cpp
// plugins/genx_filter/src/PluginEditor.cpp

GenXFilterEditor::GenXFilterEditor(GenXFilterProcessor& p)
    : AudioProcessorEditor(&p),
      processorRef(p),
      headerBar(GenX::Theme::create(
          GenX::Tokens::Accent::filter, "GENX FILTER", "FILTER")),
      filterSection("Filter"),
      outputSection("Output")
{
    auto theme = GenX::Theme::create(
        GenX::Tokens::Accent::filter, "GENX FILTER", "FILTER");
    lnf = std::make_unique<GenX::LookAndFeel>(theme);
    setLookAndFeel(lnf.get());

    addAndMakeVisible(headerBar);
    addAndMakeVisible(filterSection);
    addAndMakeVisible(outputSection);

    GenX::setupKnob(cutoffSlider, cutoffLabel, "Cutoff");
    GenX::setupKnob(resonanceSlider, resonanceLabel, "Resonance");
    GenX::setupKnob(gainSlider, gainLabel, "Gain");

    addAndMakeVisible(cutoffSlider);
    addAndMakeVisible(cutoffLabel);
    addAndMakeVisible(resonanceSlider);
    addAndMakeVisible(resonanceLabel);
    addAndMakeVisible(gainSlider);
    addAndMakeVisible(gainLabel);

    // Parameter attachments
    cutoffAttachment = std::make_unique<SliderAttachment>(
        processorRef.apvts, "cutoff", cutoffSlider);
    resonanceAttachment = std::make_unique<SliderAttachment>(
        processorRef.apvts, "resonance", resonanceSlider);
    gainAttachment = std::make_unique<SliderAttachment>(
        processorRef.apvts, "gain", gainSlider);

    setSize(500, 400);
}
```

This plugin automatically gets the GenX family look — dark panels, pink/magenta knob arcs, clean typography — with zero custom rendering code.

---

## Visual Reference

### Old vs. New Comparison

| Element | Old (Pioneer VFD) | New (GenX Design System) |
|---------|-------------------|--------------------------|
| Background | Deep black + procedural grain lines | Flat charcoal `(16,16,20)` |
| Accent color | Amber `(255,176,0)` everywhere | Per-plugin (blue, purple, pink, etc.) |
| Knob body | Dark with amber arc + red pointer | Matte dark with accent-colored arc + accent pointer |
| Toggle on | Amber-filled rectangle with glow | Accent-filled rounded rect, no glow |
| Section borders | Amber outline with VFD glow header | Subtle gray border, uppercase gray header |
| Title | VFD phosphor bloom (3-layer) | Clean BebasNeue with accent underline |
| Fonts | DSEG14/DSEG7 (segment display) | Inter (labels), JetBrains Mono (values), BebasNeue (headers) |
| Edge effects | 4-sided vignette darkening | None (clean edges) |
| Chrome/texture | Brushed metal grain | Clean flat dark panels |
| Popup menus | Amber text, amber highlight | White text, accent highlight |

### Accent Color Palette at a Glance

```
Delay       ████  #50A0FF  Cool blue
Reverb      ████  #A064F0  Purple
Compressor  ████  #FF8C32  Warm orange
EQ          ████  #50DCA0  Teal/green
Filter      ████  #F05082  Pink/magenta
Utility     ████  #C8C8D2  Neutral silver
```
