#pragma once

#include <juce_audio_processors/juce_audio_processors.h>
#include <juce_gui_basics/juce_gui_basics.h>
#include "PluginProcessor.h"

//==============================================================================
// Atari 2600 color palette — matte black, walnut woodgrain, chrome, orange
//==============================================================================
namespace AtariColors
{
    inline const juce::Colour bgBlack        {  25,  25,  28 };  // matte black plastic body
    inline const juce::Colour bgRidge        {  38,  36,  35 };  // ribbed plastic / section panels
    inline const juce::Colour bgWood         { 155,  90,  42 };  // walnut woodgrain
    inline const juce::Colour bgWoodDark     { 120,  65,  28 };  // darker wood grain lines
    inline const juce::Colour textSilver     { 200, 200, 205 };  // chrome / silver text
    inline const juce::Colour textDim        { 115, 115, 120 };  // dimmed labels
    inline const juce::Colour accentOrange   { 218, 130,  42 };  // Atari orange — the accent
    inline const juce::Colour knobSilver     { 150, 150, 158 };  // silver knob body
    inline const juce::Colour knobDark       {  70,  70,  75 };  // knob shadow / track
}

//==============================================================================
// Custom LookAndFeel for Atari 2600 theme
//==============================================================================
class AtariLookAndFeel : public juce::LookAndFeel_V4
{
public:
    AtariLookAndFeel();

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

    juce::Font getComboBoxFont(juce::ComboBox& box) override;
    juce::Font getPopupMenuFont() override;

    void setBodyFont(const juce::Font& font) { bodyFont = font; }

    juce::Colour sectionColor { AtariColors::accentOrange };

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
    AtariLookAndFeel atariLnf;

    // Custom fonts
    juce::Font titleFont { juce::FontOptions{} };    // Bebas Neue
    juce::Font headerFont { juce::FontOptions{} };   // Righteous
    juce::Font bodyFont { juce::FontOptions{} };     // Josefin Sans

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
