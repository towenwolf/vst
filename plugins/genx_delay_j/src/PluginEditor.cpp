#include "PluginEditor.h"
#include "BinaryData.h"

using namespace WoodstockColors;

//==============================================================================
// WoodstockLookAndFeel
//==============================================================================

WoodstockLookAndFeel::WoodstockLookAndFeel()
{
    // Base color scheme
    setColour(juce::ResizableWindow::backgroundColourId, bgCrimson);
    setColour(juce::Label::textColourId, textCream);
    setColour(juce::Slider::textBoxTextColourId, posterWhite);
    setColour(juce::Slider::textBoxBackgroundColourId, bgPanelDark);
    setColour(juce::Slider::textBoxOutlineColourId, juce::Colours::transparentBlack);
    setColour(juce::TextButton::buttonColourId, bgPanelDark);
    setColour(juce::TextButton::textColourOffId, posterWhite);
    setColour(juce::ComboBox::backgroundColourId, bgPanelDark);
    setColour(juce::ComboBox::textColourId, textCream);
    setColour(juce::ComboBox::outlineColourId, textCream.withAlpha(0.3f));
    setColour(juce::ComboBox::arrowColourId, textCream);
    setColour(juce::PopupMenu::backgroundColourId, bgPanelDark);
    setColour(juce::PopupMenu::textColourId, textCream);
    setColour(juce::PopupMenu::highlightedBackgroundColourId, highlightGold);
    setColour(juce::PopupMenu::highlightedTextColourId, bgCrimson);
    setColour(juce::ToggleButton::textColourId, textCream);
    setColour(juce::ToggleButton::tickColourId, accentSage);
    setColour(juce::ToggleButton::tickDisabledColourId, textCream.withAlpha(0.3f));
    setColour(juce::CaretComponent::caretColourId, textCream);
}

void WoodstockLookAndFeel::drawLinearSlider(juce::Graphics& g, int x, int y, int width, int height,
                                             float sliderPos, float /*minSliderPos*/, float /*maxSliderPos*/,
                                             juce::Slider::SliderStyle style, juce::Slider& slider)
{
    if (style != juce::Slider::LinearHorizontal)
    {
        LookAndFeel_V4::drawLinearSlider(g, x, y, width, height, sliderPos, 0, 0, style, slider);
        return;
    }

    const float trackHeight = 4.0f;
    const float trackY = (float)y + ((float)height - trackHeight) * 0.5f;
    const float trackX = (float)x;
    const float trackWidth = (float)width;
    const float thumbRadius = 6.0f;

    // Track background
    g.setColour(bgPanelDark);
    g.fillRoundedRectangle(trackX, trackY, trackWidth, trackHeight, trackHeight * 0.5f);

    // Filled portion (sage green)
    const float fillWidth = sliderPos - trackX;
    if (fillWidth > 0.0f)
    {
        g.setColour(slider.isEnabled() ? accentSage : accentSage.withAlpha(0.3f));
        g.fillRoundedRectangle(trackX, trackY, fillWidth, trackHeight, trackHeight * 0.5f);
    }

    // Thumb
    const bool isHovered = slider.isMouseOverOrDragging();
    const juce::Colour thumbColour = isHovered ? highlightGold : textCream;
    g.setColour(slider.isEnabled() ? thumbColour : thumbColour.withAlpha(0.3f));
    g.fillEllipse(sliderPos - thumbRadius, trackY + trackHeight * 0.5f - thumbRadius,
                  thumbRadius * 2.0f, thumbRadius * 2.0f);
}

void WoodstockLookAndFeel::drawToggleButton(juce::Graphics& g, juce::ToggleButton& button,
                                              bool shouldDrawButtonAsHighlighted, bool /*shouldDrawButtonAsDown*/)
{
    const float size = 14.0f;
    const float margin = 4.0f;
    const float boxY = ((float)button.getHeight() - size) * 0.5f;

    juce::Rectangle<float> boxBounds(margin, boxY, size, size);

    // Checkbox box
    if (button.getToggleState())
    {
        g.setColour(button.isEnabled() ? accentSage : accentSage.withAlpha(0.3f));
        g.fillRoundedRectangle(boxBounds, 2.0f);
    }
    else
    {
        g.setColour(button.isEnabled() ? textCream : textCream.withAlpha(0.3f));
        g.drawRoundedRectangle(boxBounds, 2.0f, 1.0f);
    }

    // Checkmark
    if (button.getToggleState())
    {
        g.setColour(bgCrimson);
        juce::Path tick;
        tick.startNewSubPath(boxBounds.getX() + 3.0f, boxBounds.getCentreY());
        tick.lineTo(boxBounds.getCentreX() - 1.0f, boxBounds.getBottom() - 3.0f);
        tick.lineTo(boxBounds.getRight() - 3.0f, boxBounds.getY() + 3.0f);
        g.strokePath(tick, juce::PathStrokeType(1.5f));
    }

    // Highlight on hover
    if (shouldDrawButtonAsHighlighted)
    {
        g.setColour(highlightGold.withAlpha(0.15f));
        g.fillRoundedRectangle(boxBounds.expanded(2.0f), 3.0f);
    }

    // Label text
    const float textX = boxBounds.getRight() + 6.0f;
    g.setColour(button.isEnabled() ? textCream : textCream.withAlpha(0.3f));
    g.setFont(bodyFont.withHeight(11.0f));
    g.drawText(button.getButtonText(),
               juce::Rectangle<float>(textX, 0.0f, (float)button.getWidth() - textX, (float)button.getHeight()),
               juce::Justification::centredLeft);
}

void WoodstockLookAndFeel::drawComboBox(juce::Graphics& g, int width, int height, bool /*isButtonDown*/,
                                          int /*buttonX*/, int /*buttonY*/, int /*buttonW*/, int /*buttonH*/,
                                          juce::ComboBox& box)
{
    auto bounds = juce::Rectangle<float>(0.0f, 0.0f, (float)width, (float)height);

    g.setColour(bgPanelDark);
    g.fillRoundedRectangle(bounds, 3.0f);

    g.setColour(textCream.withAlpha(0.4f));
    g.drawRoundedRectangle(bounds.reduced(0.5f), 3.0f, 1.0f);

    // Arrow
    const float arrowSize = 6.0f;
    const float arrowX = (float)width - 16.0f;
    const float arrowY = ((float)height - arrowSize * 0.5f) * 0.5f;

    juce::Path arrow;
    arrow.addTriangle(arrowX, arrowY,
                      arrowX + arrowSize, arrowY,
                      arrowX + arrowSize * 0.5f, arrowY + arrowSize * 0.5f);

    g.setColour(box.isEnabled() ? textCream : textCream.withAlpha(0.3f));
    g.fillPath(arrow);
}

void WoodstockLookAndFeel::drawPopupMenuBackground(juce::Graphics& g, int width, int height)
{
    g.fillAll(bgPanelDark);
    g.setColour(textCream.withAlpha(0.2f));
    g.drawRect(0, 0, width, height);
}

void WoodstockLookAndFeel::drawPopupMenuItem(juce::Graphics& g, const juce::Rectangle<int>& area,
                                               bool /*isSeparator*/, bool isActive, bool isHighlighted,
                                               bool isTicked, bool /*hasSubMenu*/,
                                               const juce::String& text, const juce::String& /*shortcutKeyText*/,
                                               const juce::Drawable* /*icon*/, const juce::Colour* /*textColour*/)
{
    if (isHighlighted && isActive)
    {
        g.setColour(highlightGold);
        g.fillRect(area);
        g.setColour(bgCrimson);
    }
    else
    {
        g.setColour(isActive ? textCream : textCream.withAlpha(0.4f));
    }

    auto textArea = area.reduced(8, 0);
    g.setFont(bodyFont.withHeight(11.0f));
    g.drawText(text, textArea, juce::Justification::centredLeft);

    if (isTicked)
    {
        g.setColour(isHighlighted ? bgCrimson : accentSage);
        auto tickBounds = area.withLeft(area.getRight() - area.getHeight()).reduced(5);
        g.fillEllipse(tickBounds.toFloat());
    }
}

void WoodstockLookAndFeel::drawLabel(juce::Graphics& g, juce::Label& label)
{
    g.fillAll(label.findColour(juce::Label::backgroundColourId));

    if (!label.isBeingEdited())
    {
        g.setColour(label.findColour(juce::Label::textColourId));
        g.setFont(label.getFont());
        g.drawText(label.getText(), label.getLocalBounds(),
                   label.getJustificationType(), true);
    }
}

juce::Font WoodstockLookAndFeel::getComboBoxFont(juce::ComboBox& /*box*/)
{
    return bodyFont.withHeight(11.0f);
}

juce::Font WoodstockLookAndFeel::getPopupMenuFont()
{
    return bodyFont.withHeight(11.0f);
}

//==============================================================================
// Section color helper
//==============================================================================
static juce::Colour getSectionColor(int sectionIndex)
{
    switch (sectionIndex)
    {
        case 0: return accentCoral;   // TIME
        case 1: return accentSage;    // MAIN
        case 2: return accentSky;     // STEREO
        case 3: return accentTone;    // TONE
        case 4: return accentAmber;   // MODULATION
        case 5: return doveWhite;     // DUCK
        default: return textCream;
    }
}

static const char* getSectionName(int sectionIndex)
{
    switch (sectionIndex)
    {
        case 0: return "TIME";
        case 1: return "MAIN";
        case 2: return "STEREO";
        case 3: return "TONE";
        case 4: return "MODULATION";
        case 5: return "DUCK";
        default: return "";
    }
}

//==============================================================================
// GenXDelayEditor
//==============================================================================

GenXDelayEditor::GenXDelayEditor(GenXDelayProcessor& p)
    : AudioProcessorEditor(&p), processorRef(p)
{
    // Load custom fonts from binary data
    auto righteousTypeface = juce::Typeface::createSystemTypefaceFor(
        BinaryData::RighteousRegular_ttf, BinaryData::RighteousRegular_ttfSize);
    auto josefinTypeface = juce::Typeface::createSystemTypefaceFor(
        BinaryData::JosefinSansVariable_ttf, BinaryData::JosefinSansVariable_ttfSize);

    righteousFont = juce::Font(juce::FontOptions(righteousTypeface));
    josefinFont = juce::Font(juce::FontOptions(josefinTypeface));

    woodstockLnf.setBodyFont(josefinFont);
    setLookAndFeel(&woodstockLnf);

    auto& apvts = processorRef.getAPVTS();

    // TIME section
    setupSlider(delayTimeSlider, delayTimeLabel, "Delay (ms)", " ms");
    delayTimeSlider.setSkewFactorFromMidPoint(300.0);
    delayTimeAttachment = std::make_unique<SliderAttachment>(apvts, "delayTime", delayTimeSlider);

    addAndMakeVisible(reverseButton);
    reverseAttachment = std::make_unique<ButtonAttachment>(apvts, "reverse", reverseButton);

    addAndMakeVisible(tempoSyncButton);
    tempoSyncAttachment = std::make_unique<ButtonAttachment>(apvts, "tempoSync", tempoSyncButton);

    noteDivisionBox.addItemList({ "1/1", "1/2", "1/2D", "1/2T", "1/4", "1/4D", "1/4T",
                                   "1/8", "1/8D", "1/8T", "1/16", "1/16D", "1/16T" }, 1);
    noteDivisionLabel.setText("Div", juce::dontSendNotification);
    noteDivisionLabel.setFont(josefinFont.withHeight(11.0f));
    noteDivisionLabel.setColour(juce::Label::textColourId, textCream);
    noteDivisionLabel.setJustificationType(juce::Justification::centredRight);
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

    setupSlider(trimSlider, trimLabel, "Trim (dB)", " dB");
    trimAttachment = std::make_unique<SliderAttachment>(apvts, "trim", trimSlider);

    // Mode buttons (Digital | Analog) — horizontal selectable buttons like Rust
    digitalModeButton.setClickingTogglesState(false);
    analogModeButton.setClickingTogglesState(false);
    addAndMakeVisible(digitalModeButton);
    addAndMakeVisible(analogModeButton);

    digitalModeButton.onClick = [&apvts]() {
        if (auto* param = apvts.getParameter("mode"))
        {
            param->beginChangeGesture();
            param->setValueNotifyingHost(0.0f);
            param->endChangeGesture();
        }
    };
    analogModeButton.onClick = [&apvts]() {
        if (auto* param = apvts.getParameter("mode"))
        {
            param->beginChangeGesture();
            param->setValueNotifyingHost(1.0f);
            param->endChangeGesture();
        }
    };

    // STEREO section
    addAndMakeVisible(pingPongButton);
    pingPongAttachment = std::make_unique<ButtonAttachment>(apvts, "pingPong", pingPongButton);

    setupSlider(stereoOffsetSlider, stereoOffsetLabel, "Offset (ms)", " ms");
    stereoOffsetAttachment = std::make_unique<SliderAttachment>(apvts, "stereoOffset", stereoOffsetSlider);

    // TONE section
    setupSlider(highPassSlider, highPassLabel, "HP (Hz)", " Hz");
    highPassSlider.setSkewFactorFromMidPoint(200.0);
    highPassAttachment = std::make_unique<SliderAttachment>(apvts, "highPass", highPassSlider);

    setupSlider(lowPassSlider, lowPassLabel, "LP (Hz)", " Hz");
    lowPassSlider.setSkewFactorFromMidPoint(4000.0);
    lowPassAttachment = std::make_unique<SliderAttachment>(apvts, "lowPass", lowPassSlider);

    // MODULATION section
    setupSlider(modRateSlider, modRateLabel, "Rate (Hz)", " Hz");
    modRateSlider.setSkewFactorFromMidPoint(1.5);
    modRateAttachment = std::make_unique<SliderAttachment>(apvts, "modRate", modRateSlider);

    setupSlider(modDepthSlider, modDepthLabel, "Depth", "%");
    modDepthSlider.textFromValueFunction = [](double v) { return juce::String(v * 100.0, 1); };
    modDepthSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    modDepthAttachment = std::make_unique<SliderAttachment>(apvts, "modDepth", modDepthSlider);

    setupSlider(driveSlider, driveLabel, "Drive", "%");
    driveSlider.textFromValueFunction = [](double v) { return juce::String(v * 100.0, 1); };
    driveSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    driveAttachment = std::make_unique<SliderAttachment>(apvts, "drive", driveSlider);

    // DUCK section
    setupSlider(duckAmountSlider, duckAmountLabel, "Amount", "%");
    duckAmountSlider.textFromValueFunction = [](double v) { return juce::String(v * 100.0, 1); };
    duckAmountSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    duckAmountAttachment = std::make_unique<SliderAttachment>(apvts, "duckAmount", duckAmountSlider);

    setupSlider(duckThresholdSlider, duckThresholdLabel, "Threshold", "%");
    duckThresholdSlider.textFromValueFunction = [](double v) { return juce::String(v * 100.0, 1); };
    duckThresholdSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    duckThresholdAttachment = std::make_unique<SliderAttachment>(apvts, "duckThreshold", duckThresholdSlider);

    // Listen for mode changes
    apvts.addParameterListener("mode", this);
    isAnalogMode = apvts.getRawParameterValue("mode")->load() >= 0.5f;
    updateModeButtons();
    updateModulationEnabled();

    setSize(600, 420);
    setResizable(true, true);
    setResizeLimits(400, 300, 1200, 900);
}

GenXDelayEditor::~GenXDelayEditor()
{
    processorRef.getAPVTS().removeParameterListener("mode", this);
    setLookAndFeel(nullptr);
}

void GenXDelayEditor::setupSlider(juce::Slider& slider, juce::Label& label,
                                    const juce::String& text, const juce::String& suffix)
{
    slider.setSliderStyle(juce::Slider::LinearHorizontal);
    slider.setTextBoxStyle(juce::Slider::TextBoxRight, false, 55, 16);
    if (suffix.isNotEmpty())
        slider.setTextValueSuffix(suffix);
    addAndMakeVisible(slider);

    label.setText(text, juce::dontSendNotification);
    label.setFont(josefinFont.withHeight(11.0f));
    label.setColour(juce::Label::textColourId, textCream);
    label.setJustificationType(juce::Justification::centredRight);
    addAndMakeVisible(label);
}

void GenXDelayEditor::updateModeButtons()
{
    if (isAnalogMode)
    {
        digitalModeButton.setColour(juce::TextButton::buttonColourId, bgPanelDark);
        digitalModeButton.setColour(juce::TextButton::textColourOffId, posterWhite);
        analogModeButton.setColour(juce::TextButton::buttonColourId, highlightGold);
        analogModeButton.setColour(juce::TextButton::textColourOffId, bgCrimson);
    }
    else
    {
        digitalModeButton.setColour(juce::TextButton::buttonColourId, highlightGold);
        digitalModeButton.setColour(juce::TextButton::textColourOffId, bgCrimson);
        analogModeButton.setColour(juce::TextButton::buttonColourId, bgPanelDark);
        analogModeButton.setColour(juce::TextButton::textColourOffId, posterWhite);
    }
    digitalModeButton.repaint();
    analogModeButton.repaint();
}

void GenXDelayEditor::updateModulationEnabled()
{
    modRateSlider.setEnabled(isAnalogMode);
    modRateLabel.setEnabled(isAnalogMode);
    modDepthSlider.setEnabled(isAnalogMode);
    modDepthLabel.setEnabled(isAnalogMode);
    driveSlider.setEnabled(isAnalogMode);
    driveLabel.setEnabled(isAnalogMode);

    const float alpha = isAnalogMode ? 1.0f : 0.3f;
    modRateLabel.setAlpha(alpha);
    modDepthLabel.setAlpha(alpha);
    driveLabel.setAlpha(alpha);
}

void GenXDelayEditor::parameterChanged(const juce::String& parameterID, float newValue)
{
    if (parameterID == "mode")
    {
        isAnalogMode = newValue >= 0.5f;

        juce::MessageManager::callAsync([this]()
        {
            updateModeButtons();
            updateModulationEnabled();
            repaint();
        });
    }
}

//==============================================================================
void GenXDelayEditor::paint(juce::Graphics& g)
{
    auto bounds = getLocalBounds();
    const float w = (float)bounds.getWidth();
    const float h = (float)bounds.getHeight();
    const float scale = std::min(w / 600.0f, h / 420.0f);

    // Background
    g.fillAll(bgCrimson);

    // Border
    g.setColour(windowBorder);
    g.drawRect(bounds, 1);

    // Title: "GENX DELAY"
    g.setColour(textCream);
    g.setFont(righteousFont.withHeight(24.0f * scale));
    const int titleHeight = (int)(36.0f * scale);
    g.drawText("GENX DELAY", bounds.removeFromTop(titleHeight), juce::Justification::centred);

    // Calculate section positions (same math as resized())
    auto area = getLocalBounds();
    area.removeFromTop(titleHeight);
    const int margin = (int)(8.0f * scale);
    area = area.reduced(margin);

    int columns;
    if (w >= 540.0f) columns = 3;
    else if (w >= 360.0f) columns = 2;
    else columns = 1;

    auto sections = calculateSectionBounds(area, columns);

    const int headerH = (int)(18.0f * scale);
    const float cornerRadius = 4.0f * scale;

    for (auto& section : sections)
    {
        // Section panel background
        g.setColour(bgPanelDark);
        g.fillRoundedRectangle(section.bounds.toFloat(), cornerRadius);

        // Section outline
        g.setColour(textCream.withAlpha(0.08f));
        g.drawRoundedRectangle(section.bounds.toFloat().reduced(0.5f), cornerRadius, 0.5f);

        // Section header text
        auto headerArea = section.bounds.removeFromTop(headerH);
        juce::Colour sectionColour = getSectionColor(section.sectionIndex);

        // Dim modulation section header when in digital mode
        if (section.sectionIndex == 4 && !isAnalogMode)
            sectionColour = sectionColour.withAlpha(0.45f);

        g.setColour(sectionColour);
        g.setFont(righteousFont.withHeight(13.0f * scale));
        g.drawText(getSectionName(section.sectionIndex), headerArea.reduced(4, 0),
                   juce::Justification::centredLeft);

        // "(Analog only)" subtitle for MODULATION
        if (section.sectionIndex == 4)
        {
            g.setFont(josefinFont.withHeight(9.0f * scale));
            g.setColour(sectionColour);
            auto subtitleArea = headerArea;
            subtitleArea.translate((int)(85.0f * scale), 0);
            g.drawText("(Analog only)", subtitleArea, juce::Justification::centredLeft);
        }
    }
}

void GenXDelayEditor::resized()
{
    auto area = getLocalBounds();
    const float w = (float)area.getWidth();
    const float h = (float)area.getHeight();
    const float scale = std::min(w / 600.0f, h / 420.0f);

    const int titleHeight = (int)(36.0f * scale);
    area.removeFromTop(titleHeight);

    const int margin = (int)(8.0f * scale);
    area = area.reduced(margin);

    // Determine columns
    int columns;
    if (w >= 540.0f) columns = 3;
    else if (w >= 360.0f) columns = 2;
    else columns = 1;

    // Section order: TIME(0), MAIN(1), STEREO(2), TONE(3), MODULATION(4), DUCK(5)
    const int numSections = 6;
    const int gap = (int)(4.0f * scale);
    const int colGap = (int)(6.0f * scale);

    // Layout sections in rows of `columns` width
    for (int i = 0; i < numSections; i += columns)
    {
        // Calculate row height: max of section heights in this row
        int sectionsInRow = std::min(columns, numSections - i);
        int colWidth = (area.getWidth() - colGap * (sectionsInRow - 1)) / sectionsInRow;

        // Estimate section heights
        auto estimateHeight = [&](int idx) -> int
        {
            const int headerH = (int)(18.0f * scale);
            const int rowH = (int)(22.0f * scale);
            const int spacing = (int)(3.0f * scale);
            switch (idx)
            {
                case 0: return headerH + rowH * 4 + spacing * 4;   // TIME: delay, reverse, sync, div
                case 1: return headerH + rowH * 4 + spacing * 4;   // MAIN: fb, mix, trim, mode
                case 2: return headerH + rowH * 2 + spacing * 2;   // STEREO: pp, offset
                case 3: return headerH + rowH * 2 + spacing * 2;   // TONE: hp, lp
                case 4: return headerH + rowH * 4 + spacing * 4;   // MOD: label, rate, depth, drive
                case 5: return headerH + rowH * 2 + spacing * 2;   // DUCK: amount, threshold
                default: return 0;
            }
        };

        int maxHeight = 0;
        for (int j = 0; j < sectionsInRow; ++j)
            maxHeight = std::max(maxHeight, estimateHeight(i + j));

        auto rowArea = area.removeFromTop(maxHeight);

        for (int j = 0; j < sectionsInRow; ++j)
        {
            auto sectionArea = rowArea.removeFromLeft(colWidth);
            if (j < sectionsInRow - 1)
                rowArea.removeFromLeft(colGap);

            layoutSection(i + j, sectionArea);
        }

        area.removeFromTop(gap);
    }
}

void GenXDelayEditor::layoutSection(int sectionIndex, juce::Rectangle<int> area)
{
    const float w = (float)getWidth();
    const float h = (float)getHeight();
    const float scale = std::min(w / 600.0f, h / 420.0f);

    const int headerH = (int)(18.0f * scale);
    const int rowH = (int)(22.0f * scale);
    const int spacing = (int)(3.0f * scale);
    const int labelWidth = (int)(65.0f * scale);

    auto layoutSliderRow = [&](juce::Label& label, juce::Slider& slider)
    {
        auto row = area.removeFromTop(rowH);
        label.setBounds(row.removeFromLeft(labelWidth));
        slider.setBounds(row.reduced(1));
        area.removeFromTop(spacing);
    };

    auto layoutToggleRow = [&](juce::ToggleButton& button)
    {
        auto row = area.removeFromTop(rowH);
        row.removeFromLeft(labelWidth);
        button.setBounds(row.reduced(1));
        area.removeFromTop(spacing);
    };

    auto layoutComboRow = [&](juce::Label& label, juce::ComboBox& combo)
    {
        auto row = area.removeFromTop(rowH);
        label.setBounds(row.removeFromLeft(labelWidth));
        combo.setBounds(row.removeFromLeft((int)(80.0f * scale)).reduced(1));
        area.removeFromTop(spacing);
    };

    // Skip the header area (painted in paint())
    area.removeFromTop(headerH);

    switch (sectionIndex)
    {
        case 0: // TIME
            layoutSliderRow(delayTimeLabel, delayTimeSlider);
            layoutToggleRow(reverseButton);
            layoutToggleRow(tempoSyncButton);
            layoutComboRow(noteDivisionLabel, noteDivisionBox);
            break;

        case 1: // MAIN
            layoutSliderRow(feedbackLabel, feedbackSlider);
            layoutSliderRow(mixLabel, mixSlider);
            layoutSliderRow(trimLabel, trimSlider);
            {
                auto row = area.removeFromTop(rowH);
                row.removeFromLeft(labelWidth);
                int halfW = row.getWidth() / 2;
                digitalModeButton.setBounds(row.removeFromLeft(halfW).reduced(1));
                analogModeButton.setBounds(row.removeFromLeft(halfW).reduced(1));
            }
            break;

        case 2: // STEREO
            layoutToggleRow(pingPongButton);
            layoutSliderRow(stereoOffsetLabel, stereoOffsetSlider);
            break;

        case 3: // TONE
            layoutSliderRow(highPassLabel, highPassSlider);
            layoutSliderRow(lowPassLabel, lowPassSlider);
            break;

        case 4: // MODULATION
        {
            area.removeFromTop(spacing); // extra spacing for "(Analog only)" text area
            layoutSliderRow(modRateLabel, modRateSlider);
            layoutSliderRow(modDepthLabel, modDepthSlider);
            layoutSliderRow(driveLabel, driveSlider);
            break;
        }

        case 5: // DUCK
            layoutSliderRow(duckAmountLabel, duckAmountSlider);
            layoutSliderRow(duckThresholdLabel, duckThresholdSlider);
            break;
    }
}

std::vector<GenXDelayEditor::SectionBounds>
GenXDelayEditor::calculateSectionBounds(juce::Rectangle<int> area, int columns) const
{
    const float w = (float)getWidth();
    const float h = (float)getHeight();
    const float scale = std::min(w / 600.0f, h / 420.0f);

    const int gap = (int)(4.0f * scale);
    const int colGap = (int)(6.0f * scale);
    const int numSections = 6;

    auto estimateHeight = [&](int idx) -> int
    {
        const int headerH = (int)(18.0f * scale);
        const int rowH = (int)(22.0f * scale);
        const int spacing = (int)(3.0f * scale);
        switch (idx)
        {
            case 0: return headerH + rowH * 4 + spacing * 4;
            case 1: return headerH + rowH * 4 + spacing * 4;
            case 2: return headerH + rowH * 2 + spacing * 2;
            case 3: return headerH + rowH * 2 + spacing * 2;
            case 4: return headerH + rowH * 4 + spacing * 4;
            case 5: return headerH + rowH * 2 + spacing * 2;
            default: return 0;
        }
    };

    std::vector<SectionBounds> result;

    for (int i = 0; i < numSections; i += columns)
    {
        int sectionsInRow = std::min(columns, numSections - i);
        int colWidth = (area.getWidth() - colGap * (sectionsInRow - 1)) / sectionsInRow;

        int maxHeight = 0;
        for (int j = 0; j < sectionsInRow; ++j)
            maxHeight = std::max(maxHeight, estimateHeight(i + j));

        auto rowArea = area.removeFromTop(maxHeight);

        for (int j = 0; j < sectionsInRow; ++j)
        {
            auto sectionArea = rowArea.removeFromLeft(colWidth);
            if (j < sectionsInRow - 1)
                rowArea.removeFromLeft(colGap);

            result.push_back({ sectionArea, i + j });
        }

        area.removeFromTop(gap);
    }

    return result;
}
