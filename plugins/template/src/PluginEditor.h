#pragma once

#include <juce_audio_processors/juce_audio_processors.h>
#include <juce_gui_basics/juce_gui_basics.h>
#include "PluginProcessor.h"

//==============================================================================
// GenX Design System — Color Tokens
//==============================================================================
namespace GenXColors
{
    // Backgrounds — layered dark surfaces
    inline const juce::Colour bg0             {  16,  16,  20 };  // deepest background
    inline const juce::Colour bg1             {  24,  24,  28 };  // section card surface
    inline const juce::Colour bg2             {  32,  33,  38 };  // elevated / hover surface
    inline const juce::Colour bgOverlay       {  40,  42,  48 };  // popup / modal overlay

    // Text hierarchy
    inline const juce::Colour textPrimary     { 220, 222, 228 };  // high-contrast primary
    inline const juce::Colour textSecondary   { 140, 144, 155 };  // labels, dimmed
    inline const juce::Colour textDisabled    {  75,  78,  88 };  // disabled state

    // Borders and dividers
    inline const juce::Colour border          {  52,  54,  62 };  // subtle panel borders
    inline const juce::Colour borderLight     {  68,  70,  78 };  // hover / focus borders

    // Knob hardware colors
    inline const juce::Colour knobSurface     {  38,  40,  46 };  // matte knob body
    inline const juce::Colour knobEdge        {  58,  60,  68 };  // machined edge ring
    inline const juce::Colour knobGrip        { 255, 255, 255 };  // grip lines (use at low alpha)
    inline const juce::Colour knobShadow      {   0,   0,   0 };  // drop shadow

    // Inactive arc on knobs
    inline const juce::Colour track           {  48,  50,  58 };  // background arc
}

//==============================================================================
// Per-Plugin Accent Presets — pick one for your plugin
//==============================================================================
namespace GenXAccent
{
    inline const juce::Colour delay           {  80, 160, 255 };  // cool blue
    inline const juce::Colour reverb          { 160, 100, 240 };  // purple
    inline const juce::Colour compressor      { 255, 140,  50 };  // warm orange
    inline const juce::Colour eq              {  80, 220, 160 };  // teal / green
    inline const juce::Colour filter          { 240,  80, 130 };  // pink / magenta
    inline const juce::Colour utility         { 200, 200, 210 };  // neutral silver
}

//==============================================================================
// Design Tokens — Spacing, Typography, Knob Rendering
//==============================================================================
namespace GenXSpace
{
    inline constexpr float margin             = 12.0f;   // outer edge padding
    inline constexpr float sectionGap         =  6.0f;   // between section cards
    inline constexpr float columnGap          =  6.0f;   // between grid columns
    inline constexpr float cardPadding        = 10.0f;   // inside a section card
    inline constexpr float controlGap         =  6.0f;   // between controls in a section
    inline constexpr float knobSize           = 56.0f;   // rotary knob diameter
    inline constexpr float valueHeight        = 16.0f;   // text box below knob
    inline constexpr float labelHeight        = 14.0f;   // label below value
    inline constexpr float headerHeight       = 20.0f;   // section header strip
    inline constexpr float toggleHeight       = 24.0f;   // toggle button row
    inline constexpr float headerBarHeight    = 40.0f;   // top branding bar
}

namespace GenXType
{
    inline constexpr float titleSize          = 28.0f;   // plugin name in header bar (Bebas Neue)
    inline constexpr float sectionHeader      = 20.0f;   // "TIME", "MAIN", etc. (Bebas Neue)
    inline constexpr float controlLabel       = 11.0f;   // knob labels (Inter)
    inline constexpr float controlValue       = 14.0f;   // numeric value readouts (JetBrains Mono)
    inline constexpr float buttonText         = 11.0f;   // toggle / button labels (Inter)
}

namespace GenXKnob
{
    inline constexpr float gripLineCount      = 20.0f;
    inline constexpr float gripAlpha          = 0.04f;
    inline constexpr float edgeThickness      = 1.2f;
    inline constexpr float arcThickness       = 2.5f;
    inline constexpr float arcGlowExtra       = 4.0f;
    inline constexpr float pointerInner       = 0.3f;    // as fraction of radius
    inline constexpr float pointerOuter       = 0.72f;
    inline constexpr float pointerThickness   = 2.0f;
    inline constexpr float hoverGlowAlpha     = 0.10f;
    inline constexpr float shadowOffset       = 1.5f;
    inline constexpr float shadowAlpha        = 0.4f;
}

namespace GenXRadius
{
    inline constexpr float card               = 4.0f;
    inline constexpr float button             = 3.0f;
    inline constexpr float popup              = 4.0f;
}

//==============================================================================
// Theme — per-plugin accent configuration
//==============================================================================
struct GenXTheme
{
    juce::Colour accent;
    juce::Colour accentDim;
    juce::Colour accentGlow;
    juce::String displayName;
    juce::String shortName;

    static GenXTheme create(const juce::Colour& accentColor,
                            const juce::String& displayName,
                            const juce::String& shortName)
    {
        return {
            accentColor,
            accentColor.withBrightness(accentColor.getBrightness() * 0.5f),
            accentColor.withBrightness(
                std::min(1.0f, accentColor.getBrightness() * 1.3f))
                .withSaturation(accentColor.getSaturation() * 0.7f),
            displayName,
            shortName
        };
    }
};

//==============================================================================
// GenX LookAndFeel — shared rendering for knobs, toggles, combos, menus
//==============================================================================
class GenXLookAndFeel : public juce::LookAndFeel_V4
{
public:
    explicit GenXLookAndFeel(const GenXTheme& theme);

    const GenXTheme& getTheme() const { return theme; }

    void drawRotarySlider(juce::Graphics& g, int x, int y, int width, int height,
                          float sliderPos, float startAngle, float endAngle,
                          juce::Slider& slider) override;

    void drawToggleButton(juce::Graphics& g, juce::ToggleButton& button,
                          bool shouldDrawButtonAsHighlighted,
                          bool shouldDrawButtonAsDown) override;

    void drawComboBox(juce::Graphics& g, int width, int height, bool isButtonDown,
                      int bx, int by, int bw, int bh,
                      juce::ComboBox& box) override;

    void drawPopupMenuBackground(juce::Graphics& g, int width, int height) override;

    void drawPopupMenuItem(juce::Graphics& g, const juce::Rectangle<int>& area,
                           bool isSeparator, bool isActive, bool isHighlighted,
                           bool isTicked, bool hasSubMenu,
                           const juce::String& text, const juce::String& shortcutKeyText,
                           const juce::Drawable* icon, const juce::Colour* textColour) override;

    void drawLabel(juce::Graphics& g, juce::Label& label) override;

    void drawLinearSlider(juce::Graphics& g, int x, int y, int width, int height,
                          float sliderPos, float minSliderPos, float maxSliderPos,
                          juce::Slider::SliderStyle style, juce::Slider& slider) override;

    juce::Font getComboBoxFont(juce::ComboBox& box) override;
    juce::Font getPopupMenuFont() override;

    // Fonts — set by the editor after loading from binary data
    juce::Font headerFont  { juce::FontOptions{} };
    juce::Font labelFont   { juce::FontOptions{} };
    juce::Font valueFont   { juce::FontOptions{} };
    juce::Font titleFont   { juce::FontOptions{} };

private:
    GenXTheme theme;
};

//==============================================================================
// Window Size Constants
//==============================================================================
namespace GenXWindow
{
    // Default window size (design reference)
    inline constexpr int defaultWidth         = 600;
    inline constexpr int defaultHeight        = 435;

    // Minimum window size
    inline constexpr int minWidth             = 600;
    inline constexpr int minHeight            = 435;

    // Maximum window size
    inline constexpr int maxWidth             = 1600;
    inline constexpr int maxHeight            = 1160;

    // Aspect ratio (width / height) — enforced on resize
    inline constexpr double aspectRatio       = 800.0 / 580.0;  // ~1.379:1
}

//==============================================================================
// Layout Context — computed from window dimensions for responsive scaling
//==============================================================================
struct GenXLayoutContext
{
    float scale;
    int columns;
    int margin;
    int sectionGap;
    int columnGap;

    static GenXLayoutContext compute(int windowWidth, int windowHeight)
    {
        float w = (float)windowWidth;
        float h = (float)windowHeight;
        float s = std::min(w / (float)GenXWindow::defaultWidth,
                           h / (float)GenXWindow::defaultHeight);

        int cols = 1;
        if (w >= 560.0f) cols = 3;
        else if (w >= 380.0f) cols = 2;

        return {
            s,
            cols,
            (int)(GenXSpace::margin * s),
            (int)(GenXSpace::sectionGap * s),
            (int)(GenXSpace::columnGap * s)
        };
    }
};

//==============================================================================
// GenX Template Editor
//
// TEMPLATE INSTRUCTIONS:
//   1. Rename this class to match your plugin
//   2. Set your accent color in the constructor (GenXAccent::delay, etc.)
//   3. Update displayName and shortName for the header bar
//   4. Add your controls (knobs, sliders, toggles, combos) as members
//   5. Wire them to APVTS parameters with attachments
//   6. Define your sections and layout in resized()
//==============================================================================
class GenXTemplateEditor : public juce::AudioProcessorEditor
{
public:
    explicit GenXTemplateEditor(GenXTemplateProcessor&);
    ~GenXTemplateEditor() override;

    void paint(juce::Graphics&) override;
    void resized() override;

private:
    GenXTemplateProcessor& processorRef;
    GenXLookAndFeel lnf;

    // Fonts loaded from binary data
    juce::Font titleFont   { juce::FontOptions{} };
    juce::Font headerFont  { juce::FontOptions{} };
    juce::Font labelFont   { juce::FontOptions{} };
    juce::Font valueFont   { juce::FontOptions{} };

    // ── Attachments ─────────────────────────────────────────────────────
    using SliderAttachment  = juce::AudioProcessorValueTreeState::SliderAttachment;
    using ButtonAttachment  = juce::AudioProcessorValueTreeState::ButtonAttachment;
    using ComboBoxAttachment = juce::AudioProcessorValueTreeState::ComboBoxAttachment;

    // ── MAIN section — example knobs ────────────────────────────────────
    juce::Slider gainSlider;
    juce::Label  gainLabel;
    std::unique_ptr<SliderAttachment> gainAttachment;

    juce::Slider mixSlider;
    juce::Label  mixLabel;
    std::unique_ptr<SliderAttachment> mixAttachment;

    // ── CONTROLS section — example toggle ───────────────────────────────
    juce::ToggleButton bypassButton { "BYPASS" };
    std::unique_ptr<ButtonAttachment> bypassAttachment;

    // ── Setup helpers ───────────────────────────────────────────────────
    void setupKnob(juce::Slider& slider, juce::Label& label, const juce::String& text);
    void setupToggle(juce::ToggleButton& toggle);

    // ── Layout helpers ──────────────────────────────────────────────────
    void drawHeaderBar(juce::Graphics& g, juce::Rectangle<int> area, float scale);
    void drawSectionCard(juce::Graphics& g, juce::Rectangle<int> area,
                         const juce::String& title, float scale);
    void placeKnob(juce::Slider& slider, juce::Label& label,
                   juce::Rectangle<int> area, float scale);

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(GenXTemplateEditor)
};
