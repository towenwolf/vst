#include "PluginEditor.h"

//==============================================================================
GenXDelayEditor::GenXDelayEditor(GenXDelayProcessor& p)
    : AudioProcessorEditor(&p), processor(p)
{
    auto& apvts = processor.getAPVTS();

    // Setup section labels
    setupSectionLabel(timeSectionLabel);
    setupSectionLabel(mainSectionLabel);
    setupSectionLabel(stereoSectionLabel);
    setupSectionLabel(toneSectionLabel);
    setupSectionLabel(modSectionLabel);
    setupSectionLabel(duckSectionLabel);

    // TIME section
    setupSlider(delayTimeSlider, delayTimeLabel, "Delay Time", " ms");
    delayTimeSlider.setSkewFactorFromMidPoint(300.0);
    delayTimeAttachment = std::make_unique<SliderAttachment>(apvts, "delayTime", delayTimeSlider);

    addAndMakeVisible(reverseButton);
    reverseAttachment = std::make_unique<ButtonAttachment>(apvts, "reverse", reverseButton);

    addAndMakeVisible(tempoSyncButton);
    tempoSyncAttachment = std::make_unique<ButtonAttachment>(apvts, "tempoSync", tempoSyncButton);

    noteDivisionBox.addItemList({"1/1", "1/2", "1/2D", "1/2T", "1/4", "1/4D", "1/4T",
                                  "1/8", "1/8D", "1/8T", "1/16", "1/16D", "1/16T"}, 1);
    noteDivisionLabel.setText("Note Division", juce::dontSendNotification);
    addAndMakeVisible(noteDivisionBox);
    addAndMakeVisible(noteDivisionLabel);
    noteDivisionAttachment = std::make_unique<ComboBoxAttachment>(apvts, "noteDivision", noteDivisionBox);

    // MAIN section
    setupSlider(feedbackSlider, feedbackLabel, "Feedback", "%");
    feedbackSlider.textFromValueFunction = [](double v) { return juce::String(v * 100.0, 1); };
    feedbackSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    feedbackAttachment = std::make_unique<SliderAttachment>(apvts, "feedback", feedbackSlider);

    setupSlider(mixSlider, mixLabel, "Mix", "%");
    mixSlider.textFromValueFunction = [](double v) { return juce::String(v * 100.0, 1); };
    mixSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    mixAttachment = std::make_unique<SliderAttachment>(apvts, "mix", mixSlider);

    setupSlider(trimSlider, trimLabel, "Trim", " dB");
    trimAttachment = std::make_unique<SliderAttachment>(apvts, "trim", trimSlider);

    modeBox.addItemList({"Digital", "Analog"}, 1);
    modeLabel.setText("Mode", juce::dontSendNotification);
    addAndMakeVisible(modeBox);
    addAndMakeVisible(modeLabel);
    modeAttachment = std::make_unique<ComboBoxAttachment>(apvts, "mode", modeBox);

    // STEREO section
    addAndMakeVisible(pingPongButton);
    pingPongAttachment = std::make_unique<ButtonAttachment>(apvts, "pingPong", pingPongButton);

    setupSlider(stereoOffsetSlider, stereoOffsetLabel, "Stereo Offset", " ms");
    stereoOffsetAttachment = std::make_unique<SliderAttachment>(apvts, "stereoOffset", stereoOffsetSlider);

    // TONE section
    setupSlider(highPassSlider, highPassLabel, "High-Pass", " Hz");
    highPassSlider.setSkewFactorFromMidPoint(200.0);
    highPassAttachment = std::make_unique<SliderAttachment>(apvts, "highPass", highPassSlider);

    setupSlider(lowPassSlider, lowPassLabel, "Low-Pass", " Hz");
    lowPassSlider.setSkewFactorFromMidPoint(4000.0);
    lowPassAttachment = std::make_unique<SliderAttachment>(apvts, "lowPass", lowPassSlider);

    // MODULATION section
    setupSlider(modRateSlider, modRateLabel, "Mod Rate", " Hz");
    modRateSlider.setSkewFactorFromMidPoint(1.5);
    modRateAttachment = std::make_unique<SliderAttachment>(apvts, "modRate", modRateSlider);

    setupSlider(modDepthSlider, modDepthLabel, "Mod Depth", "%");
    modDepthSlider.textFromValueFunction = [](double v) { return juce::String(v * 100.0, 1); };
    modDepthSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    modDepthAttachment = std::make_unique<SliderAttachment>(apvts, "modDepth", modDepthSlider);

    setupSlider(driveSlider, driveLabel, "Drive", "%");
    driveSlider.textFromValueFunction = [](double v) { return juce::String(v * 100.0, 1); };
    driveSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    driveAttachment = std::make_unique<SliderAttachment>(apvts, "drive", driveSlider);

    // DUCK section
    setupSlider(duckAmountSlider, duckAmountLabel, "Duck Amount", "%");
    duckAmountSlider.textFromValueFunction = [](double v) { return juce::String(v * 100.0, 1); };
    duckAmountSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    duckAmountAttachment = std::make_unique<SliderAttachment>(apvts, "duckAmount", duckAmountSlider);

    setupSlider(duckThresholdSlider, duckThresholdLabel, "Threshold", "%");
    duckThresholdSlider.textFromValueFunction = [](double v) { return juce::String(v * 100.0, 1); };
    duckThresholdSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    duckThresholdAttachment = std::make_unique<SliderAttachment>(apvts, "duckThreshold", duckThresholdSlider);

    setSize(600, 520);
    setResizable(true, true);
    setResizeLimits(400, 400, 1200, 900);
}

GenXDelayEditor::~GenXDelayEditor()
{
}

void GenXDelayEditor::setupSlider(juce::Slider& slider, juce::Label& label,
                                   const juce::String& text, const juce::String& suffix)
{
    slider.setSliderStyle(juce::Slider::LinearHorizontal);
    slider.setTextBoxStyle(juce::Slider::TextBoxRight, false, 70, 20);
    if (suffix.isNotEmpty())
        slider.setTextValueSuffix(suffix);
    addAndMakeVisible(slider);

    label.setText(text, juce::dontSendNotification);
    label.setJustificationType(juce::Justification::centredRight);
    addAndMakeVisible(label);
}

void GenXDelayEditor::setupSectionLabel(juce::Label& label)
{
    label.setFont(juce::Font(14.0f, juce::Font::bold));
    label.setColour(juce::Label::textColourId, juce::Colours::white);
    addAndMakeVisible(label);
}

void GenXDelayEditor::paint(juce::Graphics& g)
{
    g.fillAll(juce::Colour(0xff2d2d2d));

    // Title
    g.setColour(juce::Colours::white);
    g.setFont(juce::Font(24.0f, juce::Font::bold));
    g.drawText("GenX Delay", getLocalBounds().removeFromTop(45), juce::Justification::centred);

    // Section backgrounds
    auto drawSection = [&](int y, int height) {
        g.setColour(juce::Colour(0xff3a3a3a));
        g.fillRoundedRectangle(10.0f, static_cast<float>(y), getWidth() - 20.0f,
                               static_cast<float>(height), 6.0f);
    };

    int y = 50;
    drawSection(y, 95);      // TIME
    drawSection(y + 100, 95); // MAIN
    drawSection(y + 200, 65); // STEREO
    drawSection(y + 270, 65); // TONE
    drawSection(y + 340, 85); // MODULATION
    drawSection(y + 430, 65); // DUCK
}

void GenXDelayEditor::resized()
{
    auto area = getLocalBounds().reduced(15);
    area.removeFromTop(45); // Title

    const int labelWidth = 90;
    const int rowHeight = 26;
    const int sectionHeaderHeight = 22;
    const int sectionGap = 8;
    const int comboWidth = 120;

    auto layoutRow = [&](juce::Label& label, juce::Slider& slider) {
        auto row = area.removeFromTop(rowHeight);
        label.setBounds(row.removeFromLeft(labelWidth));
        slider.setBounds(row.reduced(2));
    };

    auto layoutToggle = [&](juce::ToggleButton& button) {
        auto row = area.removeFromTop(rowHeight);
        row.removeFromLeft(labelWidth);
        button.setBounds(row.reduced(2));
    };

    auto layoutCombo = [&](juce::Label& label, juce::ComboBox& combo) {
        auto row = area.removeFromTop(rowHeight);
        label.setBounds(row.removeFromLeft(labelWidth));
        combo.setBounds(row.removeFromLeft(comboWidth).reduced(2));
    };

    // TIME section
    timeSectionLabel.setBounds(area.removeFromTop(sectionHeaderHeight));
    layoutRow(delayTimeLabel, delayTimeSlider);
    {
        auto toggleRow = area.removeFromTop(rowHeight);
        toggleRow.removeFromLeft(labelWidth);
        int halfWidth = toggleRow.getWidth() / 2;
        reverseButton.setBounds(toggleRow.removeFromLeft(halfWidth).reduced(2));
        tempoSyncButton.setBounds(toggleRow.reduced(2));
    }
    layoutCombo(noteDivisionLabel, noteDivisionBox);
    area.removeFromTop(sectionGap);

    // MAIN section
    mainSectionLabel.setBounds(area.removeFromTop(sectionHeaderHeight));
    layoutRow(feedbackLabel, feedbackSlider);
    layoutRow(mixLabel, mixSlider);
    layoutRow(trimLabel, trimSlider);
    layoutCombo(modeLabel, modeBox);
    area.removeFromTop(sectionGap);

    // STEREO section
    stereoSectionLabel.setBounds(area.removeFromTop(sectionHeaderHeight));
    layoutToggle(pingPongButton);
    layoutRow(stereoOffsetLabel, stereoOffsetSlider);
    area.removeFromTop(sectionGap);

    // TONE section
    toneSectionLabel.setBounds(area.removeFromTop(sectionHeaderHeight));
    layoutRow(highPassLabel, highPassSlider);
    layoutRow(lowPassLabel, lowPassSlider);
    area.removeFromTop(sectionGap);

    // MODULATION section
    modSectionLabel.setBounds(area.removeFromTop(sectionHeaderHeight));
    layoutRow(modRateLabel, modRateSlider);
    layoutRow(modDepthLabel, modDepthSlider);
    layoutRow(driveLabel, driveSlider);
    area.removeFromTop(sectionGap);

    // DUCK section
    duckSectionLabel.setBounds(area.removeFromTop(sectionHeaderHeight));
    layoutRow(duckAmountLabel, duckAmountSlider);
    layoutRow(duckThresholdLabel, duckThresholdSlider);
}
