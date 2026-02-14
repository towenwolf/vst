# Figma-to-JUCE Plugin GUI Workflow Guide

A practical, end-to-end guide for designing audio plugin GUIs in Figma and translating them into JUCE C++ code. This guide uses the "Reference Only" approach — Figma is your design tool, JUCE is your implementation tool, and you manually translate between them.

---

## Table of Contents

1. [Overview](#overview)
2. [Setting Up Your Figma File](#setting-up-your-figma-file)
3. [Designing Your Plugin GUI](#designing-your-plugin-gui)
4. [Extracting Design Values](#extracting-design-values)
5. [Mapping Figma to JUCE Code](#mapping-figma-to-juce-code)
6. [The Development Loop](#the-development-loop)
7. [Organizing Your Figma File](#organizing-your-figma-file)
8. [Common Pitfalls](#common-pitfalls)
9. [Quick Reference Cheat Sheet](#quick-reference-cheat-sheet)

---

## Overview

### What This Workflow Does

- You design the plugin GUI visually in Figma
- You inspect the design to extract exact values (colors, sizes, positions, spacing)
- You write JUCE C++ code that reproduces the design
- You iterate: tweak in Figma first, then update the code

### Why This Works Well for JUCE

JUCE plugin GUIs are built with code — there's no drag-and-drop layout editor (Projucer's GUI editor is limited and rarely used for production plugins). Figma gives you the visual iteration speed that JUCE lacks, without adding build complexity.

### What You Need

- A free Figma account (figma.com)
- Your JUCE project set up and building
- Any custom fonts you're using (TTF/OTF files)

---

## Setting Up Your Figma File

### Step 1: Create a New Figma File

1. Go to figma.com and create a new design file
2. Name it to match your plugin: e.g., "GenX Delay - GUI Design"

### Step 2: Set Up Your Frame

Your frame represents the plugin window at its default size.

1. Press **F** to create a frame
2. In the right panel, set the exact dimensions to match your plugin's `setSize()` call
   - Example: **660 x 580** (matching `setSize(660, 580)` in your JUCE code)
3. Name the frame: "Plugin Window - Default Size"

**Tip:** If your plugin is resizable, create additional frames at your min and max sizes to test how the layout scales. For example:
- "Plugin Window - Min (530 x 460)"
- "Plugin Window - Max (1250 x 1100)"

### Step 3: Upload Custom Fonts

If your plugin uses custom fonts (like DSEG for VFD displays):

1. Download the font files (TTF/OTF) — the same ones in your `assets/fonts/` directory
2. In Figma, select any text element
3. In the font picker, click the **...** menu and choose "Upload font"
4. Upload each font file

Now you can use the exact same fonts in Figma that your plugin uses.

### Step 4: Create Your Color Palette as Styles

This is critical — it ensures your Figma colors exactly match your C++ color constants.

1. Draw a small rectangle
2. Set its fill to your first color (e.g., `rgb(10, 10, 12)` for `bgBlack`)
3. Click the **4 dots** icon next to the fill color → "Create style"
4. Name it to match your code: `bgBlack (10, 10, 12)`
5. Repeat for every color in your palette

Example color styles to create (matching `PioneerColors` namespace):

| Style Name | RGB Value | Purpose |
|---|---|---|
| `bgBlack` | `(10, 10, 12)` | Chassis body |
| `bgPanel` | `(18, 18, 22)` | Panel areas |
| `displayBg` | `(5, 5, 8)` | VFD display background |
| `displayFrame` | `(30, 30, 35)` | Display border |
| `vfdAmber` | `(255, 176, 0)` | Primary VFD amber |
| `vfdAmberDim` | `(140, 95, 0)` | Dimmed VFD segments |
| `vfdAmberGlow` | `(255, 200, 50)` | Hot glow center |
| `chassisGrey` | `(45, 45, 50)` | Metal accents |
| `knobBody` | `(25, 25, 28)` | Knob body |
| `knobEdge` | `(50, 50, 55)` | Knob edge ring |
| `indicatorRed` | `(200, 40, 30)` | Indicator dot |

**Why this matters:** When you change a color style in Figma, it updates everywhere. When you change a color in your C++ namespace, it also updates everywhere. Keeping the names aligned means you always know which Figma style maps to which C++ constant.

---

## Designing Your Plugin GUI

### Step 5: Block Out the Layout

Start with rectangles representing your major layout regions. Don't worry about details yet.

1. **Background**: Fill the frame with your `bgBlack` color
2. **Display Window**: Draw a rectangle for the VFD display area
   - Position it using the right panel (X, Y, W, H values)
   - Note these values — they'll become your layout constants in code
3. **Section Panels**: Draw rectangles for each parameter section (TIME, MAIN, STEREO, etc.)
4. **Divider Lines**: Draw thin lines between regions

**Record every measurement.** In Figma's right panel, you'll see:
- **X, Y**: Position from the top-left of the parent frame
- **W, H**: Width and height
- **Padding/Spacing**: If using Auto Layout

These numbers translate directly to your JUCE layout code.

### Step 6: Design Individual Components

For each UI component, create a Figma **component** (select the elements → right-click → "Create component"). This lets you reuse them.

#### Rotary Knob Component

1. Draw a circle for the knob body (fill: `knobBody`)
2. Draw a slightly larger circle behind it for the edge ring (stroke: `knobEdge`)
3. Draw an arc for the value indicator (stroke: `vfdAmber`)
4. Draw a line for the pointer (stroke: `indicatorRed`)
5. Add a text element below for the value readout
6. Add a text element below that for the label

Measure and note:
- Knob diameter (e.g., 52px → `knobSize = (int)(52.0f * scale)`)
- Value text height (e.g., 14px → `valueH`)
- Label text height (e.g., 14px → `labelH`)
- Total knob+text height (this becomes your `knobRowH`)

#### Toggle Button Component

1. Draw a rectangle for the button body (28 x 14px)
2. Create two variants: "Off" (dark, `displayBg`) and "On" (lit, `vfdAmber`)
3. Add text next to it for the label

#### Section Panel Component

1. Draw a rounded rectangle with a thin stroke (`vfdAmber` at 25% opacity)
2. Add a text element for the section header ("TIME", "MAIN", etc.)
3. Use your VFD font for the header text

### Step 7: Assemble the Full Layout

Place your components into the section panels according to your grid:

- 3 columns at full width
- Match the spacing you want between sections
- Align knobs within each section

**Use Figma's measurement tools:** Hold **Alt/Option** and hover between elements to see spacing. These spacing values become your `pad`, `gap`, `colGap`, and `margin` constants in code.

---

## Extracting Design Values

### Step 8: Use the Inspect Panel

This is where Figma pays off. Select any element and look at the right panel:

#### Position and Size
```
X: 10    →  const int margin = 10;
Y: 8     →  const int displayTop = 8;
W: 640   →  (int)w - margin * 2
H: 87    →  const int displayHeight = (int)(h * 0.15f);
```

#### Colors
Click any color swatch to see its exact RGB values:
```
Fill: (255, 176, 0) at 25% opacity
→ vfdAmber.withAlpha(0.25f)
```

#### Typography
Select text to see:
```
Font: DSEG14Classic-Regular
Size: 24px    →  titleFont.withHeight(24.0f * scale)
Color: (255, 200, 50) at 90%  →  vfdAmberGlow.withAlpha(0.9f)
```

#### Spacing
Hold Alt/Option between elements:
```
Gap between sections: 6px  →  const int gap = (int)(6.0f * scale);
Padding inside section: 4px  →  area = area.reduced((int)(4.0f * scale), 0);
```

#### Border Radius
```
Corner radius: 2px  →  g.drawRoundedRectangle(sb, 2.0f, 0.5f);
```

#### Stroke
```
Stroke: 0.5px, (255, 176, 0) at 25%
→ g.setColour(vfdAmber.withAlpha(0.25f));
  g.drawRoundedRectangle(sb, 2.0f, 0.5f);
```

### Step 9: Create a Design Spec Document

In Figma, create a separate page called "Spec" where you annotate key measurements. This becomes your reference sheet when coding.

Add text annotations like:
```
Display Window: top=8, height=15%, margin=10
Section Grid: 3 columns, gap=6, colGap=6
Knob: diameter=52, valueText=14, label=14
Toggle: 28x14
Header font: DSEG14, 10pt
Title font: DSEG14, 24pt
```

---

## Mapping Figma to JUCE Code

### The Translation Table

Here's how Figma concepts map to JUCE code:

| Figma Concept | JUCE Equivalent |
|---|---|
| Frame dimensions | `setSize(width, height)` |
| Rectangle fill | `g.setColour(color); g.fillRect(bounds);` |
| Rectangle stroke | `g.setColour(color); g.drawRect(bounds, thickness);` |
| Rounded rectangle | `g.fillRoundedRectangle(bounds, radius);` |
| Circle | `g.fillEllipse(x, y, w, h);` |
| Line | `g.drawLine(x1, y1, x2, y2, thickness);` |
| Text | `g.setFont(font); g.drawText(text, bounds, justification);` |
| Drop shadow | Draw a darker shape offset by a few pixels behind |
| Gradient fill | `juce::ColourGradient` |
| Opacity | `color.withAlpha(0.5f)` |
| Group position (X, Y) | `component.setBounds(x, y, w, h)` |
| Auto Layout gap | Your `gap`, `pad`, `colGap` constants |
| Component variant | Different code paths (e.g., `if (isOn)` in `drawToggleButton`) |

### Scaling: Figma Pixels to JUCE Scaled Values

Your plugin is resizable, so you use a scale factor. In Figma, design at 1x (your default size). In code, multiply by scale:

```
Figma: knob is 52px wide
JUCE:  const int knobSize = (int)(52.0f * scale);
       where scale = std::min(w / 660.0f, h / 580.0f);
```

Every pixel value from Figma gets this treatment:
```cpp
// Figma says: margin is 10px, gap is 6px, header is 18px tall
const int margin = (int)(10.0f * scale);
const int gap = (int)(6.0f * scale);
const int headerH = (int)(18.0f * scale);
```

### Example: Figma Rectangle to JUCE paint()

**In Figma:**
- Rectangle at X:10, Y:8, W:640, H:87
- Fill: `displayBg` (5, 5, 8)
- Stroke: 1px, black at 60%

**In JUCE:**
```cpp
const int displayTop = (int)(8.0f * scale);
const int displayHeight = (int)(h * 0.15f);  // or fixed: (int)(87.0f * scale)
auto displayRect = juce::Rectangle<int>(margin, displayTop,
                                         (int)w - margin * 2, displayHeight);

g.setColour(displayBg);
g.fillRect(displayRect);

g.setColour(juce::Colours::black.withAlpha(0.6f));
g.drawRect(displayRect, 1);
```

### Example: Figma Text to JUCE drawText()

**In Figma:**
- Text: "GENX DELAY"
- Font: DSEG14Classic-Regular, 24px
- Color: `vfdAmberGlow` (255, 200, 50) at 90%
- Alignment: Center
- Position: centered in the display rectangle

**In JUCE:**
```cpp
g.setFont(titleFont.withHeight(24.0f * scale));
g.setColour(vfdAmberGlow.withAlpha(0.9f));
g.drawText("GENX DELAY", titleArea, juce::Justification::centred, true);
```

---

## The Development Loop

This is your day-to-day workflow once everything is set up.

### The Iteration Cycle

```
1. DESIGN in Figma
   ↓
2. INSPECT values (colors, sizes, positions)
   ↓
3. CODE in JUCE (update paint/resized/LookAndFeel)
   ↓
4. BUILD and run the plugin
   ↓
5. COMPARE side-by-side (Figma vs. running plugin)
   ↓
6. ADJUST — go back to step 1 or 3
```

### Step-by-Step: Making a Design Change

**Example: "I want to make the knobs bigger"**

1. **Figma**: Select the knob component, resize from 52px to 64px diameter
2. **Figma**: Check that the section panels still fit — adjust spacing if needed
3. **Figma**: Note the new value: knob diameter = 64px
4. **Code**: Update `const int knobSize = (int)(64.0f * scale);`
5. **Build**: `cmake --build build`
6. **Compare**: Open both Figma and the plugin, check they match
7. **Adjust**: If the sections are now too tall, adjust `estimateHeight()` or reduce padding

### Side-by-Side Comparison Tips

- Run your plugin in Standalone mode for quick iteration (no DAW needed)
- Put Figma on one half of your screen, the plugin on the other
- Use Figma's zoom to match the plugin's actual pixel size (View → Zoom to 100%)
- Screenshot the plugin and paste it into Figma as an image layer for direct overlay comparison

### When to Design First vs. Code First

| Situation | Approach |
|---|---|
| New plugin from scratch | Design first in Figma |
| Major visual redesign | Design first in Figma |
| Small tweak (change a color) | Code first, update Figma later |
| New component (meter, button) | Design first in Figma |
| Layout change (move sections) | Design first in Figma |
| Bug fix (overlapping elements) | Code first, verify against Figma |

---

## Organizing Your Figma File

### Recommended Page Structure

Create these pages in your Figma file:

1. **Design** — The main plugin design at default size
2. **Responsive** — The plugin at min/max sizes to verify scaling
3. **Components** — Individual UI components (knobs, buttons, toggles) as reusable Figma components
4. **States** — Different states of the plugin (e.g., Digital mode vs. Analog mode, hover states)
5. **Color Palette** — Visual reference of all colors with their names and RGB values
6. **Spec** — Annotated measurements and spacing reference
7. **Archive** — Old designs and iterations you want to keep for reference

### Naming Conventions

Keep Figma layer names aligned with your C++ variable names:

| Figma Layer Name | C++ Variable |
|---|---|
| `delayTimeSlider` | `delayTimeSlider` |
| `feedbackSlider` | `feedbackSlider` |
| `Section: TIME` | `case 0: // TIME` |
| `Section: MAIN` | `case 1: // MAIN` |
| `displayRect` | `displayRect` |

This makes it trivial to find the code that corresponds to any Figma element.

---

## Common Pitfalls

### 1. Figma Uses CSS Colors, JUCE Uses 0-255 RGB

Figma shows hex colors like `#FFB000`. JUCE uses `juce::Colour(255, 176, 0)`.

Quick conversion:
- `#FF` = 255, `#B0` = 176, `#00` = 0
- Or use Figma's color picker which shows RGB values directly (switch to RGB mode in the color picker)

### 2. Figma Opacity vs. JUCE Alpha

Figma shows opacity as a percentage (25%). JUCE uses a float (0.25f).
```
Figma: 25% opacity  →  JUCE: .withAlpha(0.25f)
```

### 3. Figma's Y-Axis is Top-Down (Same as JUCE)

Both Figma and JUCE use top-left origin with Y increasing downward. No conversion needed.

### 4. Figma Blur/Shadow Effects Don't Translate Directly

Figma has built-in drop shadows and blurs. JUCE doesn't have these — you need to fake them:

- **Drop shadow**: Draw a darker, slightly offset rectangle behind the element
- **Blur/glow**: Draw the element multiple times at increasing sizes with decreasing alpha (like the `drawVFDText` method)
- **Inner shadow**: Draw dark edges on the top-left, lighter edges on the bottom-right

### 5. Figma Auto Layout ≠ JUCE Layout

Figma's Auto Layout is CSS Flexbox. JUCE uses manual `setBounds()` calls or the `juce::FlexBox`/`juce::Grid` classes. The simplest approach:
- Use Figma Auto Layout for visual prototyping
- Read the resulting positions and sizes
- Translate to manual `setBounds()` or `removeFromTop()`/`removeFromLeft()` calls

### 6. Font Rendering Differences

Fonts may render slightly differently between Figma and JUCE (different anti-aliasing, hinting). Don't try to get pixel-perfect text matching — aim for "close enough" and adjust font sizes by ±1px if needed.

### 7. Keep Figma Updated

When you change code without updating Figma first, your design file becomes stale. Set a rule: **if you change a layout value in code, update Figma within the same work session.** A stale Figma file is worse than no Figma file.

---

## Quick Reference Cheat Sheet

### Figma Shortcuts You'll Use Constantly

| Action | Shortcut (Mac) |
|---|---|
| Measure distance between elements | Hold **Option** + hover |
| Zoom to 100% | **Cmd + 0** |
| Zoom to fit | **Cmd + 1** |
| Create frame | **F** |
| Create rectangle | **R** |
| Create ellipse | **O** |
| Create text | **T** |
| Create component | **Cmd + Option + K** |
| Copy CSS (for color values) | Right-click → "Copy as CSS" |
| Toggle Dev Mode / Inspect | **Shift + D** |

### JUCE Drawing Cheat Sheet

```cpp
// Fill a rectangle
g.setColour(myColor);
g.fillRect(bounds);

// Draw a rectangle outline
g.drawRect(bounds, lineThickness);

// Rounded rectangle
g.fillRoundedRectangle(bounds.toFloat(), cornerRadius);
g.drawRoundedRectangle(bounds.toFloat(), cornerRadius, lineThickness);

// Circle/Ellipse
g.fillEllipse(x, y, width, height);
g.drawEllipse(x, y, width, height, lineThickness);

// Line
g.drawLine(x1, y1, x2, y2, thickness);

// Text
g.setFont(myFont.withHeight(size));
g.setColour(textColor);
g.drawText("text", bounds, juce::Justification::centred, true);

// Gradient
juce::ColourGradient grad(color1, x1, y1, color2, x2, y2, isRadial);
g.setGradientFill(grad);
g.fillRect(bounds);

// Arc (for knob indicators)
juce::Path arc;
arc.addCentredArc(cx, cy, radius, radius, 0, startAngle, endAngle, true);
g.strokePath(arc, juce::PathStrokeType(thickness));

// Alpha/Opacity
g.setColour(myColor.withAlpha(0.5f));

// Save/restore graphics state
juce::Graphics::ScopedSaveState saveState(g);
```

### Color Conversion Reference

```
Figma hex → JUCE:
  #0A0A0C → juce::Colour(10, 10, 12)
  #FFB000 → juce::Colour(255, 176, 0)

Figma % opacity → JUCE:
  100% → 1.0f
   90% → 0.9f
   50% → 0.5f
   25% → 0.25f
   10% → 0.1f
```

---

## Summary

1. **Set up Figma** with your exact plugin dimensions, colors, and fonts
2. **Design visually** — block out layout, then design individual components
3. **Inspect values** — use Figma's inspect panel to get exact numbers
4. **Write JUCE code** — translate Figma values to `paint()`, `resized()`, and LookAndFeel methods
5. **Compare side-by-side** — run the plugin and compare against Figma
6. **Iterate** — adjust in Figma first for visual changes, then update code
7. **Keep Figma in sync** — update Figma whenever you change layout values in code

The goal isn't pixel-perfect reproduction — it's having a fast visual iteration loop so you spend less time recompiling and more time designing.
