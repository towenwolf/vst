#pragma once

#include <juce_audio_processors/juce_audio_processors.h>
#include <juce_gui_basics/juce_gui_basics.h>
#include "PluginProcessor.h"

//==============================================================================
class GenXDelayEditor : public juce::AudioProcessorEditor
{
public:
    explicit GenXDelayEditor(GenXDelayProcessor&);
    ~GenXDelayEditor() override;

    void paint(juce::Graphics&) override;
    void resized() override;

private:
    GenXDelayProcessor& processor;

    // Helper to create and attach sliders
    using SliderAttachment = juce::AudioProcessorValueTreeState::SliderAttachment;
    using ButtonAttachment = juce::AudioProcessorValueTreeState::ButtonAttachment;
    using ComboBoxAttachment = juce::AudioProcessorValueTreeState::ComboBoxAttachment;

    // TIME section
    juce::Slider delayTimeSlider;
    juce::Label delayTimeLabel;
    std::unique_ptr<SliderAttachment> delayTimeAttachment;

    juce::ToggleButton reverseButton{"Reverse"};
    std::unique_ptr<ButtonAttachment> reverseAttachment;

    juce::ToggleButton tempoSyncButton{"Tempo Sync"};
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

    juce::ComboBox modeBox;
    juce::Label modeLabel;
    std::unique_ptr<ComboBoxAttachment> modeAttachment;

    // STEREO section
    juce::ToggleButton pingPongButton{"Ping Pong"};
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

    // Section labels
    juce::Label timeSectionLabel{"", "TIME"};
    juce::Label mainSectionLabel{"", "MAIN"};
    juce::Label stereoSectionLabel{"", "STEREO"};
    juce::Label toneSectionLabel{"", "TONE"};
    juce::Label modSectionLabel{"", "MODULATION"};
    juce::Label duckSectionLabel{"", "DUCK"};

    void setupSlider(juce::Slider& slider, juce::Label& label, const juce::String& text,
                     const juce::String& suffix = "");
    void setupSectionLabel(juce::Label& label);

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(GenXDelayEditor)
};
