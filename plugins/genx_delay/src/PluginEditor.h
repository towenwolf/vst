#pragma once

#include <juce_audio_processors/juce_audio_processors.h>
#include <juce_gui_basics/juce_gui_basics.h>
#include "PluginProcessor.h"

//==============================================================================
// Pioneer VFD receiver color palette — amber VFD on matte black chassis
//==============================================================================
namespace PioneerColors
{
    inline const juce::Colour bgBlack        {  10,  10,  12 };  // deep black chassis body
    inline const juce::Colour bgPanel        {  18,  18,  22 };  // slightly lighter panel areas
    inline const juce::Colour displayBg      {   5,   5,   8 };  // VFD display window background
    inline const juce::Colour displayFrame   {  30,  30,  35 };  // recessed display border
    inline const juce::Colour vfdAmber       { 255, 176,   0 };  // primary VFD amber glow
    inline const juce::Colour vfdAmberDim    { 140,  95,   0 };  // dimmed / inactive VFD segments
    inline const juce::Colour vfdAmberGlow   { 255, 200,  50 };  // hot glow center for bloom
    inline const juce::Colour chassisGrey    {  45,  45,  50 };  // brushed metal / chassis accents
    inline const juce::Colour knobBody       {  25,  25,  28 };  // dark knob body
    inline const juce::Colour knobEdge       {  50,  50,  55 };  // knob edge / ring
    inline const juce::Colour indicatorRed   { 200,  40,  30 };  // red indicator dot on knob
}

//==============================================================================
// Custom LookAndFeel — Pioneer VFD receiver theme
//==============================================================================
class PioneerLookAndFeel : public juce::LookAndFeel_V4
{
public:
    PioneerLookAndFeel();

    void drawRotarySlider(juce::Graphics& g, int x, int y, int width, int height,
                          float sliderPos, float rotaryStartAngle, float rotaryEndAngle,
                          juce::Slider& slider) override;

    void drawToggleButton(juce::Graphics& g, juce::ToggleButton& button,
                          bool shouldDrawButtonAsHighlighted,
                          bool shouldDrawButtonAsDown) override;

    void drawComboBox(juce::Graphics& g, int width, int height, bool isButtonDown,
                      int buttonX, int buttonY, int buttonW, int buttonH,
                      juce::ComboBox& box) override;

    void drawPopupMenuBackground(juce::Graphics& g, int width, int height) override;

    void drawPopupMenuItem(juce::Graphics& g, const juce::Rectangle<int>& area,
                           bool isSeparator, bool isActive, bool isHighlighted,
                           bool isTicked, bool hasSubMenu,
                           const juce::String& text, const juce::String& shortcutKeyText,
                           const juce::Drawable* icon, const juce::Colour* textColour) override;

    void drawLabel(juce::Graphics& g, juce::Label& label) override;

    void drawButtonText(juce::Graphics& g, juce::TextButton& button,
                        bool shouldDrawButtonAsHighlighted,
                        bool shouldDrawButtonAsDown) override;

    juce::Font getComboBoxFont(juce::ComboBox& box) override;
    juce::Font getPopupMenuFont() override;

    void setFonts(const juce::Font& segFont, const juce::Font& numFont)
    {
        segmentFont = segFont;
        numericFont = numFont;
    }

    juce::Colour sectionColor { PioneerColors::vfdAmber };

private:
    juce::Font segmentFont { juce::FontOptions{} };
    juce::Font numericFont { juce::FontOptions{} };
};

//==============================================================================
class GenXDelayEditor : public juce::AudioProcessorEditor,
                         private juce::AudioProcessorValueTreeState::Listener
{
public:
    explicit GenXDelayEditor(GenXDelayProcessor&);
    ~GenXDelayEditor() override;

    void paint(juce::Graphics&) override;
    void resized() override;

private:
    GenXDelayProcessor& processorRef;
    PioneerLookAndFeel pioneerLnf;

    // Custom fonts
    juce::Font titleFont  { juce::FontOptions{} };
    juce::Font headerFont { juce::FontOptions{} };
    juce::Font bodyFont   { juce::FontOptions{} };
    juce::Font numFont    { juce::FontOptions{} };

    // Attachments
    using SliderAttachment = juce::AudioProcessorValueTreeState::SliderAttachment;
    using ButtonAttachment = juce::AudioProcessorValueTreeState::ButtonAttachment;
    using ComboBoxAttachment = juce::AudioProcessorValueTreeState::ComboBoxAttachment;

    // TIME section
    juce::Slider delayTimeSlider;
    juce::Label delayTimeLabel;
    std::unique_ptr<SliderAttachment> delayTimeAttachment;

    juce::ToggleButton reverseButton{ "Rev" };
    std::unique_ptr<ButtonAttachment> reverseAttachment;

    juce::ToggleButton tempoSyncButton{ "Sync" };
    std::unique_ptr<ButtonAttachment> tempoSyncAttachment;

    juce::ComboBox noteDivisionBox;
    juce::Label noteDivisionLabel;
    std::unique_ptr<ComboBoxAttachment> noteDivisionAttachment;

    // MAIN section
    juce::Slider feedbackSlider;
    juce::Label feedbackLabel;
    std::unique_ptr<SliderAttachment> feedbackAttachment;

    juce::Slider mixSlider;
    juce::Label mixLabel;
    std::unique_ptr<SliderAttachment> mixAttachment;

    juce::Slider trimSlider;
    juce::Label trimLabel;
    std::unique_ptr<SliderAttachment> trimAttachment;

    juce::TextButton digitalModeButton{ "Digital" };
    juce::TextButton analogModeButton{ "Analog" };

    // STEREO section
    juce::ToggleButton pingPongButton{ "Ping Pong" };
    std::unique_ptr<ButtonAttachment> pingPongAttachment;

    juce::Slider stereoOffsetSlider;
    juce::Label stereoOffsetLabel;
    std::unique_ptr<SliderAttachment> stereoOffsetAttachment;

    // TONE section
    juce::Slider highPassSlider;
    juce::Label highPassLabel;
    std::unique_ptr<SliderAttachment> highPassAttachment;

    juce::Slider lowPassSlider;
    juce::Label lowPassLabel;
    std::unique_ptr<SliderAttachment> lowPassAttachment;

    // MODULATION section
    juce::Slider modRateSlider;
    juce::Label modRateLabel;
    std::unique_ptr<SliderAttachment> modRateAttachment;

    juce::Slider modDepthSlider;
    juce::Label modDepthLabel;
    std::unique_ptr<SliderAttachment> modDepthAttachment;

    juce::Slider driveSlider;
    juce::Label driveLabel;
    std::unique_ptr<SliderAttachment> driveAttachment;

    // DUCK section
    juce::Slider duckAmountSlider;
    juce::Label duckAmountLabel;
    std::unique_ptr<SliderAttachment> duckAmountAttachment;

    juce::Slider duckThresholdSlider;
    juce::Label duckThresholdLabel;
    std::unique_ptr<SliderAttachment> duckThresholdAttachment;

    // Mode tracking
    bool isAnalogMode = false;

    // Setup helpers
    void setupSlider(juce::Slider& slider, juce::Label& label, const juce::String& text);
    void updateModeButtons();
    void updateModulationEnabled();

    // VFD glow text helper
    void drawVFDText(juce::Graphics& g, const juce::String& text,
                     juce::Rectangle<int> area, const juce::Font& font,
                     juce::Justification just = juce::Justification::centred,
                     float glowIntensity = 1.0f);

    // AudioProcessorValueTreeState::Listener
    void parameterChanged(const juce::String& parameterID, float newValue) override;

    // Layout helpers
    struct SectionBounds {
        juce::Rectangle<int> bounds;
        int sectionIndex;
    };
    std::vector<SectionBounds> calculateSectionBounds(juce::Rectangle<int> area, int columns) const;
    void layoutSection(int sectionIndex, juce::Rectangle<int> area);

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(GenXDelayEditor)
};
