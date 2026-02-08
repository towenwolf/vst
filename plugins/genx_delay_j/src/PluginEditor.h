#pragma once

#include <juce_audio_processors/juce_audio_processors.h>
#include <juce_gui_basics/juce_gui_basics.h>
#include "PluginProcessor.h"

//==============================================================================
// 1969 Woodstock poster color palette
//==============================================================================
namespace WoodstockColors
{
    inline const juce::Colour bgCrimson       { 183,  28,  28 };
    inline const juce::Colour bgPanelDark     { 153,  23,  23 };
    inline const juce::Colour textCream       { 255, 248, 235 };
    inline const juce::Colour posterWhite     { 240, 235, 225 };
    inline const juce::Colour accentCoral     { 255, 167, 130 };
    inline const juce::Colour accentSage      { 180, 210, 160 };
    inline const juce::Colour accentSky       { 160, 195, 230 };
    inline const juce::Colour accentAmber     { 255, 200, 120 };
    inline const juce::Colour accentTone      { 220, 190, 170 };
    inline const juce::Colour doveWhite       { 255, 252, 245 };
    inline const juce::Colour highlightGold   { 255, 215, 140 };
    inline const juce::Colour windowBorder    { 120,  18,  18 };
}

//==============================================================================
// Custom LookAndFeel for Woodstock 1969 theme
//==============================================================================
class WoodstockLookAndFeel : public juce::LookAndFeel_V4
{
public:
    WoodstockLookAndFeel();

    void drawLinearSlider(juce::Graphics& g, int x, int y, int width, int height,
                          float sliderPos, float minSliderPos, float maxSliderPos,
                          juce::Slider::SliderStyle style, juce::Slider& slider) override;

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

    juce::Font getComboBoxFont(juce::ComboBox& box) override;
    juce::Font getPopupMenuFont() override;

    void setBodyFont(const juce::Font& font) { bodyFont = font; }

private:
    juce::Font bodyFont { juce::FontOptions{} };
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
    WoodstockLookAndFeel woodstockLnf;

    // Custom fonts
    juce::Font righteousFont { juce::FontOptions{} };
    juce::Font josefinFont { juce::FontOptions{} };

    // Attachments
    using SliderAttachment = juce::AudioProcessorValueTreeState::SliderAttachment;
    using ButtonAttachment = juce::AudioProcessorValueTreeState::ButtonAttachment;
    using ComboBoxAttachment = juce::AudioProcessorValueTreeState::ComboBoxAttachment;

    // TIME section
    juce::Slider delayTimeSlider;
    juce::Label delayTimeLabel;
    std::unique_ptr<SliderAttachment> delayTimeAttachment;

    juce::ToggleButton reverseButton{ "Reverse" };
    std::unique_ptr<ButtonAttachment> reverseAttachment;

    juce::ToggleButton tempoSyncButton{ "Tempo Sync" };
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
    void setupSlider(juce::Slider& slider, juce::Label& label, const juce::String& text,
                     const juce::String& suffix = "");
    void updateModeButtons();
    void updateModulationEnabled();

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
