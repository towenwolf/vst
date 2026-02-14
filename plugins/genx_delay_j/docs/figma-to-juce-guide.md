# Figma to JUCE Plugin GUI Workflow

A step-by-step guide for designing your GenX Delay plugin UI in Figma and bringing it into JUCE.

---

## 1. Get Started with Figma

1. Go to **figma.com** and create a free account
2. Click **"New design file"** from your dashboard
3. You're now in the editor — it's like Photoshop but in a browser

### Key tools you'll use (left toolbar)
- **Frame tool (F)** — creates sized containers (like artboards)
- **Rectangle tool (R)** — draws rectangles
- **Ellipse tool (O)** — draws circles (your knobs)
- **Text tool (T)** — adds text
- **Move tool (V)** — selects and moves things

### Key panels
- **Left sidebar** — your layers (objects stacked on top of each other)
- **Right sidebar** — properties of whatever you've selected (color, size, effects)

---

## 2. Set Up Your Canvas

### Create the plugin frame
1. Press **F** (Frame tool)
2. Drag anywhere on the canvas
3. In the right sidebar, set the size to **660 x 470** (your default plugin size)
4. Rename it "GenX Delay - Default" in the left sidebar (double-click the name)

### Set up your color palette
Before designing, save your colors. Click the canvas background, then in the right sidebar:

| Name          | Hex Code  | What it's for            |
|---------------|-----------|--------------------------|
| bgBlack       | `#19191C` | Main body background     |
| bgRidge       | `#262423` | Section panel background |
| bgWood        | `#9B5A2A` | Walnut woodgrain         |
| bgWoodDark    | `#78411C` | Darker wood grain        |
| textSilver    | `#C8C8CD` | Chrome/silver text       |
| textDim       | `#737378` | Dimmed labels            |
| accentOrange  | `#DA822A` | Atari orange accent      |
| knobSilver    | `#96969E` | Knob body color          |
| knobDark      | `#46464B` | Knob shadow/track        |

**Tip**: Select any shape, click the color swatch in the right panel, paste the hex code. To save it for reuse, click the **"+"** next to "Local styles" in the right panel.

---

## 3. Design the Background

The background is the foundation — everything else sits on top of it.

### Step by step
1. Inside your 660x470 frame, draw a **rectangle (R)** that fills the whole frame
2. Set its color to `#19191C` (bgBlack)
3. Draw another rectangle for the **wood panel** covering the bottom ~47%
   - Position: X=0, Y=249 (roughly 53% down)
   - Size: 660 x 221
   - Color: `#9B5A2A`
4. For the **orange accent stripe** between black and wood:
   - Rectangle at Y=249, height=2, full width
   - Color: `#DA822A`

### Adding realistic textures
This is where Figma shines over code. You have several options:

**Option A — Use a real wood photo (recommended)**
1. Google "walnut wood texture seamless" and download a high-res image
2. Drag the image into Figma
3. Select your wood rectangle, go to **Fill** in the right panel
4. Click the color swatch, switch from "Solid" to **"Image"**
5. Upload your wood texture
6. Set fill mode to **"Fill"** or **"Tile"**
7. Adjust opacity/overlay to blend with your brown base color

**Option B — Use Figma's built-in noise**
1. Select the wood rectangle
2. In the right panel, click **"+"** next to **Effects**
3. This is limited, so Option A is better for realism

**For the matte black plastic:**
1. Find a "dark brushed plastic texture" or "matte black surface" image
2. Apply it to the top rectangle the same way (Image fill)
3. Set opacity low (~5-15%) so it's subtle

### Layer order (bottom to top)
```
Background (black rectangle)
Wood panel (bottom half)
Orange stripe (2px line between them)
```

---

## 4. Design the Knobs

Knobs are the most important visual element. You have two approaches:

### Approach A — Filmstrip (traditional, recommended)

A filmstrip is a single tall image containing every rotation frame of the knob stacked vertically. JUCE picks the right frame based on the parameter value.

#### Design one knob position
1. Create a new **Frame (F)** — size **52 x 52** (your knob size)
2. Draw a circle (O) centered in it, ~48px diameter
3. Style the circle:
   - **Fill**: Linear gradient from `#BABAC2` (top) to `#46464B` (bottom)
   - **Stroke**: 0.8px, color `#333338`
   - **Effects**: Drop shadow — X:0, Y:1, Blur:3, color black at 30%
   - **Inner shadow**: X:-1, Y:-1, Blur:2, color white at 15% (specular highlight)
4. Add a small **pointer dot** (circle, 4px) near the edge — this indicates the value
5. Add the **value arc** — a partial ring around the outside (use a circle with a thick stroke and mask it)

#### Generate the filmstrip
You need ~101 or 128 frames showing the knob at each rotation angle.

**Manual method** (tedious but precise):
1. Design the knob at the "minimum" position (pointer at ~7 o'clock)
2. Duplicate the frame
3. Rotate the pointer dot slightly
4. Repeat until you reach "maximum" (pointer at ~5 o'clock)
5. You'll want 101-128 frames total

**Better method — use a Figma plugin:**
1. In Figma, go to **Main menu > Plugins > Search "Rotator"** or **"Knob Generator"**
2. Some community plugins can auto-generate rotated copies
3. Alternatively, design the knob in code and just use Figma for the static background/panels

**Export the filmstrip:**
1. Select all 128 knob frames
2. Stack them vertically (each 52x52, total image = 52 x 6656)
3. Group them, then select the group
4. Bottom-right panel: click **"+"** next to **Export**
5. Choose **PNG**, set scale to **2x** (for retina/high-DPI)
6. Click **"Export"**

### Approach B — Single knob image + code rotation

Simpler: just export one knob body image and let JUCE rotate the pointer in code.

1. Design the knob body (circle with gradient/texture) WITHOUT the pointer
2. Export as PNG at 2x
3. In JUCE, draw the knob image, then draw the pointer line/dot on top using code (similar to what you already have)

This is a good middle ground — realistic knob body from Figma, dynamic pointer from code.

---

## 5. Design Section Panels

1. Draw rounded rectangles (R, then set corner radius to 4 in the right panel)
2. Color: `#262423`
3. Add effects:
   - **Drop shadow**: X:0, Y:1, Blur:2, Black at 25%
   - **Inner shadow**: X:0, Y:1, Blur:0, White at 4% (top highlight)
4. Add the orange accent line at the top (1px rectangle, orange)
5. Add header text using your fonts (Righteous for headers)

### Using your custom fonts in Figma
1. Install the **Figma desktop app** (fonts don't load from local in browser)
2. Or upload fonts: In Figma, just select the Text tool, type something, then in the right panel click the font name and search for your installed fonts
3. For fonts not on your system: go to **figma.com/fonts** or use Google Fonts versions

---

## 6. Design Toggle Buttons

1. Draw a **rounded rectangle** — 24 x 12, corner radius 6 (pill shape)
2. **OFF state**: fill `#46464B`, stroke `#737378` at 40% opacity
3. **ON state**: fill `#DA822A` (orange)
4. Add a small circle inside (the toggle thumb)
5. Create both states as separate frames named `toggle-off` and `toggle-on`

---

## 7. Exporting Your Assets

### What to export

| Asset              | Size      | Format | Scale | Filename               |
|--------------------|-----------|--------|-------|------------------------|
| Full background    | 660x470   | PNG    | 2x    | `background.png`       |
| Knob filmstrip     | 52x(52*N) | PNG    | 2x    | `knob_filmstrip.png`   |
| Knob (body only)   | 52x52     | PNG    | 2x    | `knob_body.png`        |
| Toggle ON          | 24x12     | PNG    | 2x    | `toggle_on.png`        |
| Toggle OFF         | 24x12     | PNG    | 2x    | `toggle_off.png`       |
| Button normal      | varies    | PNG    | 2x    | `button_normal.png`    |
| Button active      | varies    | PNG    | 2x    | `button_active.png`    |

### How to export
1. Select the frame/group you want to export
2. In the right panel, scroll to the bottom section labeled **"Export"**
3. Click **"+"** to add an export setting
4. Choose **PNG** format
5. Set multiplier to **2x** (exports at double resolution for sharp rendering)
6. Click the **"Export [name]"** button
7. Save to your project's assets folder

### Where to save
Put exported files in:
```
plugins/genx_delay_j/assets/images/
```

---

## 8. Bringing Assets into JUCE

### Step 1 — Register assets in CMakeLists.txt

Open `CMakeLists.txt` and add your images to the binary data target:

```cmake
juce_add_binary_data(GenXDelayJData SOURCES
    assets/fonts/BebasNeue-Regular.ttf
    assets/fonts/Righteous-Regular.ttf
    assets/fonts/JosefinSans-Variable.ttf
    assets/images/background.png       # <-- add these
    assets/images/knob_body.png
    assets/images/toggle_on.png
    assets/images/toggle_off.png
)
```

### Step 2 — Load images in your editor

In `PluginEditor.h`, add member variables:

```cpp
// Image assets
juce::Image backgroundImage;
juce::Image knobBodyImage;
```

In the `GenXDelayEditor` constructor in `PluginEditor.cpp`:

```cpp
// Load embedded images
backgroundImage = juce::ImageCache::getFromMemory(
    BinaryData::background_png, BinaryData::background_pngSize);

knobBodyImage = juce::ImageCache::getFromMemory(
    BinaryData::knob_body_png, BinaryData::knob_body_pngSize);
```

### Step 3 — Draw the background image

In `paint()`, replace all the procedural background drawing with:

```cpp
void GenXDelayEditor::paint(juce::Graphics& g)
{
    // Draw the full background image, scaled to fit
    g.drawImage(backgroundImage, getLocalBounds().toFloat(),
                juce::RectanglePlacement::stretchToFit);

    // ... keep the section panel / header text drawing if dynamic ...
}
```

### Step 4 — Use knob images in LookAndFeel

For the filmstrip approach in `drawRotarySlider()`:

```cpp
void AtariLookAndFeel::drawRotarySlider(juce::Graphics& g, int x, int y,
    int width, int height, float sliderPos, float /*rotaryStartAngle*/,
    float /*rotaryEndAngle*/, juce::Slider& /*slider*/)
{
    if (knobFilmstrip.isNull()) return;

    const int numFrames = 128;
    const int frameIndex = (int)(sliderPos * (numFrames - 1));
    const int frameH = knobFilmstrip.getHeight() / numFrames;

    // Source rectangle: pick the right frame from the vertical strip
    juce::Rectangle<int> sourceRect(0, frameIndex * frameH,
                                     knobFilmstrip.getWidth(), frameH);

    // Destination: centered in the slider bounds
    auto dest = juce::Rectangle<int>(x, y, width, height)
                    .withSizeKeepingCentre(width, width).toFloat();

    g.drawImage(knobFilmstrip, dest,
                juce::RectanglePlacement::stretchToFit, false,
                sourceRect.toFloat());
}
```

For the simpler body-image + code-pointer approach:

```cpp
void AtariLookAndFeel::drawRotarySlider(juce::Graphics& g, int x, int y,
    int width, int height, float sliderPos, float rotaryStartAngle,
    float rotaryEndAngle, juce::Slider& slider)
{
    const float diameter = (float)juce::jmin(width, height);
    const float centreX = (float)x + (float)width * 0.5f;
    const float centreY = (float)y + (float)height * 0.5f;

    // Draw the knob body image
    auto destRect = juce::Rectangle<float>(centreX - diameter * 0.5f,
                                            centreY - diameter * 0.5f,
                                            diameter, diameter);
    g.drawImage(knobBodyImage, destRect);

    // Draw the pointer/arc in code (keeps it dynamic)
    const float arcRadius = diameter * 0.5f - 4.0f;
    const float valueAngle = rotaryStartAngle
        + sliderPos * (rotaryEndAngle - rotaryStartAngle);

    // ... draw value arc and pointer dot same as current code ...
}
```

---

## 9. Practical Tips

### Resolution strategy
- Design at **1x** (660x470) in Figma for layout
- Export at **2x** for sharp rendering on retina displays
- JUCE's `drawImage` scales smoothly, so 2x assets look great at any plugin size

### Iteration workflow
1. Design/tweak in Figma
2. Export changed assets (overwrite the old files)
3. Rebuild (`cmake --build build --config Release`)
4. Test in standalone or DAW
5. Repeat

### Keep some things in code
Not everything needs to be an image. Good candidates for images vs. code:

| Use Images For               | Keep in Code                    |
|------------------------------|---------------------------------|
| Background / textures        | Section header text             |
| Knob bodies                  | Value arcs (dynamic)            |
| Toggle button states         | Parameter value text            |
| Wood grain / metal textures  | Layout / positioning            |
| Logo / branding              | Hover states / animations       |

### File size awareness
- A 2x background PNG will be ~200-500KB depending on complexity
- Knob filmstrips can be 500KB-2MB
- Total plugin binary increase: typically 1-3MB
- This is normal and acceptable for commercial plugins

### Figma keyboard shortcuts you'll use constantly
| Shortcut       | Action                     |
|----------------|----------------------------|
| **V**          | Move/select tool           |
| **R**          | Rectangle                  |
| **O**          | Ellipse (circle)           |
| **T**          | Text                       |
| **F**          | Frame                      |
| **Ctrl/Cmd+D** | Duplicate selection        |
| **Ctrl/Cmd+G** | Group selection            |
| **Ctrl/Cmd+]** | Bring forward (layer order)|
| **Ctrl/Cmd+[** | Send backward              |
| **Hold Alt**   | Show spacing/distances     |
| **Ctrl/Cmd+E** | Quick export selection     |
| **Ctrl/Cmd+Z** | Undo                       |
| **Space+drag** | Pan around canvas          |
| **Ctrl/Cmd+scroll** | Zoom in/out           |

---

## 10. Suggested Workflow for GenX Delay

1. **Start with the background** — get the wood texture and black body looking right
2. **Design one knob** — nail the look, then decide filmstrip vs body-only
3. **Design toggles and buttons** — simple, do these quick
4. **Export everything to `assets/images/`**
5. **Update `CMakeLists.txt`** to embed the new assets
6. **Swap out procedural drawing** in `paint()` and `drawRotarySlider()` one piece at a time
7. **Test at multiple sizes** — resize the window to make sure images scale well
8. **Iterate** — tweak in Figma, re-export, rebuild, test
