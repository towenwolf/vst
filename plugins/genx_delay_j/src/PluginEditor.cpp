#include "PluginEditor.h"
#include "BinaryData.h"

using namespace AtariColors;

//==============================================================================
// AtariLookAndFeel
//==============================================================================

AtariLookAndFeel::AtariLookAndFeel()
{
    setColour(juce::ResizableWindow::backgroundColourId, bgBlack);
    setColour(juce::Label::textColourId, textSilver);
    setColour(juce::Slider::textBoxTextColourId, textSilver);
    setColour(juce::Slider::textBoxBackgroundColourId, juce::Colours::transparentBlack);
    setColour(juce::Slider::textBoxOutlineColourId, juce::Colours::transparentBlack);
    setColour(juce::TextButton::buttonColourId, bgRidge);
    setColour(juce::TextButton::textColourOffId, textSilver);
    setColour(juce::ComboBox::backgroundColourId, bgRidge);
    setColour(juce::ComboBox::textColourId, textSilver);
    setColour(juce::ComboBox::outlineColourId, textDim.withAlpha(0.3f));
    setColour(juce::ComboBox::arrowColourId, textDim);
    setColour(juce::PopupMenu::backgroundColourId, bgRidge);
    setColour(juce::PopupMenu::textColourId, textSilver);
    setColour(juce::PopupMenu::highlightedBackgroundColourId, accentOrange);
    setColour(juce::PopupMenu::highlightedTextColourId, bgBlack);
    setColour(juce::ToggleButton::textColourId, textSilver);
    setColour(juce::ToggleButton::tickColourId, accentOrange);
    setColour(juce::ToggleButton::tickDisabledColourId, textDim.withAlpha(0.3f));
    setColour(juce::CaretComponent::caretColourId, textSilver);
}

void AtariLookAndFeel::drawRotarySlider(juce::Graphics& g, int x, int y, int width, int height,
                                          float sliderPos, float rotaryStartAngle, float rotaryEndAngle,
                                          juce::Slider& slider)
{
    const float diameter = (float)juce::jmin(width, height);
    const float radius = diameter * 0.5f;
    const float centreX = (float)x + (float)width * 0.5f;
    const float centreY = (float)y + (float)height * 0.5f;
    const float arcRadius = radius - 4.0f;

    const bool isHovered = slider.isMouseOverOrDragging();
    const float enabledAlpha = slider.isEnabled() ? 1.0f : 0.3f;

    // 1. Glow halo — subtle orange
    {
        const float glowAlpha = isHovered ? 0.12f : 0.05f;
        g.setColour(accentOrange.withAlpha(glowAlpha * enabledAlpha));
        g.fillEllipse(centreX - radius, centreY - radius, diameter, diameter);
    }

    // 2. Silver knob body — chrome look with gradient
    {
        const float bodyRadius = radius - 2.0f;
        const auto knobTop = isHovered
            ? knobSilver.brighter(0.15f)
            : knobSilver;
        const auto knobBot = knobDark;

        juce::ColourGradient grad(knobTop.withAlpha(enabledAlpha), centreX, centreY - bodyRadius,
                                   knobBot.withAlpha(enabledAlpha), centreX, centreY + bodyRadius, false);
        g.setGradientFill(grad);
        g.fillEllipse(centreX - bodyRadius, centreY - bodyRadius, bodyRadius * 2.0f, bodyRadius * 2.0f);
    }

    // 3. Track arc (background)
    {
        juce::Path trackArc;
        trackArc.addCentredArc(centreX, centreY, arcRadius, arcRadius, 0.0f,
                               rotaryStartAngle, rotaryEndAngle, true);
        g.setColour(knobDark.withAlpha(0.5f * enabledAlpha));
        g.strokePath(trackArc, juce::PathStrokeType(2.0f));
    }

    // 4. Value arc — always orange
    {
        const float valueAngle = rotaryStartAngle + sliderPos * (rotaryEndAngle - rotaryStartAngle);
        if (sliderPos > 0.0f)
        {
            juce::Path valueArc;
            valueArc.addCentredArc(centreX, centreY, arcRadius, arcRadius, 0.0f,
                                   rotaryStartAngle, valueAngle, true);
            g.setColour(accentOrange.withAlpha(enabledAlpha));
            g.strokePath(valueArc, juce::PathStrokeType(2.5f));
        }

        // 5. Pointer dot — bright silver
        const float dotRadius = 2.0f;
        const float dotX = centreX + arcRadius * std::sin(valueAngle);
        const float dotY = centreY - arcRadius * std::cos(valueAngle);
        g.setColour(textSilver.withAlpha(enabledAlpha));
        g.fillEllipse(dotX - dotRadius, dotY - dotRadius, dotRadius * 2.0f, dotRadius * 2.0f);
    }

    // 6. Center dot — dark depth cue
    {
        const float cdRadius = 1.5f;
        g.setColour(knobDark.withAlpha(enabledAlpha));
        g.fillEllipse(centreX - cdRadius, centreY - cdRadius, cdRadius * 2.0f, cdRadius * 2.0f);
    }
}

void AtariLookAndFeel::drawToggleButton(juce::Graphics& g, juce::ToggleButton& button,
                                          bool shouldDrawButtonAsHighlighted, bool /*shouldDrawButtonAsDown*/)
{
    const float pillW = 24.0f;
    const float pillH = 12.0f;
    const float pillY = ((float)button.getHeight() - pillH) * 0.5f;
    const float pillR = pillH * 0.5f;

    juce::Rectangle<float> pillBounds(4.0f, pillY, pillW, pillH);

    if (button.getToggleState())
    {
        g.setColour(button.isEnabled() ? accentOrange : accentOrange.withAlpha(0.3f));
        g.fillRoundedRectangle(pillBounds, pillR);
    }
    else
    {
        g.setColour(button.isEnabled() ? knobDark : knobDark.withAlpha(0.3f));
        g.fillRoundedRectangle(pillBounds, pillR);
        g.setColour(textDim.withAlpha(0.4f));
        g.drawRoundedRectangle(pillBounds, pillR, 1.0f);
    }

    if (shouldDrawButtonAsHighlighted)
    {
        g.setColour(accentOrange.withAlpha(0.10f));
        g.fillRoundedRectangle(pillBounds.expanded(2.0f), pillR + 2.0f);
    }

    const float textX = pillBounds.getRight() + 6.0f;
    juce::Colour textCol = button.getToggleState() ? textSilver : textDim;
    if (!button.isEnabled())
        textCol = textCol.withAlpha(0.3f);
    g.setColour(textCol);
    g.setFont(bodyFont.withHeight(11.0f));
    g.drawText(button.getButtonText(),
               juce::Rectangle<float>(textX, 0.0f, (float)button.getWidth() - textX, (float)button.getHeight()),
               juce::Justification::centredLeft);
}

void AtariLookAndFeel::drawComboBox(juce::Graphics& g, int width, int height, bool /*isButtonDown*/,
                                      int /*buttonX*/, int /*buttonY*/, int /*buttonW*/, int /*buttonH*/,
                                      juce::ComboBox& box)
{
    auto bounds = juce::Rectangle<float>(0.0f, 0.0f, (float)width, (float)height);

    g.setColour(bgRidge);
    g.fillRoundedRectangle(bounds, 3.0f);

    g.setColour(textDim.withAlpha(0.3f));
    g.drawRoundedRectangle(bounds.reduced(0.5f), 3.0f, 1.0f);

    const float arrowSize = 6.0f;
    const float arrowX = (float)width - 16.0f;
    const float arrowY = ((float)height - arrowSize * 0.5f) * 0.5f;

    juce::Path arrow;
    arrow.addTriangle(arrowX, arrowY,
                      arrowX + arrowSize, arrowY,
                      arrowX + arrowSize * 0.5f, arrowY + arrowSize * 0.5f);

    g.setColour(box.isEnabled() ? textDim : textDim.withAlpha(0.3f));
    g.fillPath(arrow);
}

void AtariLookAndFeel::drawPopupMenuBackground(juce::Graphics& g, int width, int height)
{
    g.fillAll(bgRidge);
    g.setColour(textDim.withAlpha(0.2f));
    g.drawRect(0, 0, width, height);
}

void AtariLookAndFeel::drawPopupMenuItem(juce::Graphics& g, const juce::Rectangle<int>& area,
                                           bool /*isSeparator*/, bool isActive, bool isHighlighted,
                                           bool isTicked, bool /*hasSubMenu*/,
                                           const juce::String& text, const juce::String& /*shortcutKeyText*/,
                                           const juce::Drawable* /*icon*/, const juce::Colour* /*textColour*/)
{
    if (isHighlighted && isActive)
    {
        g.setColour(accentOrange);
        g.fillRect(area);
        g.setColour(bgBlack);
    }
    else
    {
        g.setColour(isActive ? textSilver : textSilver.withAlpha(0.4f));
    }

    auto textArea = area.reduced(8, 0);
    g.setFont(bodyFont.withHeight(11.0f));
    g.drawText(text, textArea, juce::Justification::centredLeft);

    if (isTicked)
    {
        g.setColour(isHighlighted ? bgBlack : accentOrange);
        auto tickBounds = area.withLeft(area.getRight() - area.getHeight()).reduced(5);
        g.fillEllipse(tickBounds.toFloat());
    }
}

void AtariLookAndFeel::drawLabel(juce::Graphics& g, juce::Label& label)
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

juce::Font AtariLookAndFeel::getComboBoxFont(juce::ComboBox& /*box*/)
{
    return bodyFont.withHeight(11.0f);
}

juce::Font AtariLookAndFeel::getPopupMenuFont()
{
    return bodyFont.withHeight(11.0f);
}

//==============================================================================
// Helpers
//==============================================================================

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
    auto bebasTypeface = juce::Typeface::createSystemTypefaceFor(
        BinaryData::BebasNeueRegular_ttf, BinaryData::BebasNeueRegular_ttfSize);
    auto righteousTypeface = juce::Typeface::createSystemTypefaceFor(
        BinaryData::RighteousRegular_ttf, BinaryData::RighteousRegular_ttfSize);
    auto josefinTypeface = juce::Typeface::createSystemTypefaceFor(
        BinaryData::JosefinSansVariable_ttf, BinaryData::JosefinSansVariable_ttfSize);

    titleFont = juce::Font(juce::FontOptions(bebasTypeface));
    headerFont = juce::Font(juce::FontOptions(righteousTypeface));
    bodyFont = juce::Font(juce::FontOptions(josefinTypeface));

    atariLnf.setBodyFont(bodyFont);
    setLookAndFeel(&atariLnf);

    auto& apvts = processorRef.getAPVTS();

    // TIME
    setupSlider(delayTimeSlider, delayTimeLabel, "Delay Time");
    delayTimeSlider.setSkewFactorFromMidPoint(300.0);
    delayTimeSlider.textFromValueFunction = [](double v) { return juce::String((int)v) + " ms"; };
    delayTimeSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue(); };
    delayTimeAttachment = std::make_unique<SliderAttachment>(apvts, "delayTime", delayTimeSlider);

    addAndMakeVisible(reverseButton);
    reverseAttachment = std::make_unique<ButtonAttachment>(apvts, "reverse", reverseButton);

    addAndMakeVisible(tempoSyncButton);
    tempoSyncAttachment = std::make_unique<ButtonAttachment>(apvts, "tempoSync", tempoSyncButton);

    noteDivisionBox.addItemList({ "1/1", "1/2", "1/2D", "1/2T", "1/4", "1/4D", "1/4T",
                                   "1/8", "1/8D", "1/8T", "1/16", "1/16D", "1/16T" }, 1);
    noteDivisionLabel.setText("", juce::dontSendNotification);
    addAndMakeVisible(noteDivisionBox);
    addAndMakeVisible(noteDivisionLabel);
    noteDivisionAttachment = std::make_unique<ComboBoxAttachment>(apvts, "noteDivision", noteDivisionBox);

    // MAIN
    setupSlider(feedbackSlider, feedbackLabel, "Feedback");
    feedbackSlider.textFromValueFunction = [](double v) { return juce::String((int)(v * 100.0)) + "%"; };
    feedbackSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    feedbackAttachment = std::make_unique<SliderAttachment>(apvts, "feedback", feedbackSlider);

    setupSlider(mixSlider, mixLabel, "Mix");
    mixSlider.textFromValueFunction = [](double v) { return juce::String((int)(v * 100.0)) + "%"; };
    mixSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    mixAttachment = std::make_unique<SliderAttachment>(apvts, "mix", mixSlider);

    setupSlider(trimSlider, trimLabel, "Trim");
    trimSlider.textFromValueFunction = [](double v) { return juce::String(v, 1) + " dB"; };
    trimSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue(); };
    trimAttachment = std::make_unique<SliderAttachment>(apvts, "trim", trimSlider);

    digitalModeButton.setClickingTogglesState(false);
    analogModeButton.setClickingTogglesState(false);
    addAndMakeVisible(digitalModeButton);
    addAndMakeVisible(analogModeButton);

    digitalModeButton.onClick = [&apvts]() {
        if (auto* param = apvts.getParameter("mode"))
        { param->beginChangeGesture(); param->setValueNotifyingHost(0.0f); param->endChangeGesture(); }
    };
    analogModeButton.onClick = [&apvts]() {
        if (auto* param = apvts.getParameter("mode"))
        { param->beginChangeGesture(); param->setValueNotifyingHost(1.0f); param->endChangeGesture(); }
    };

    // STEREO
    addAndMakeVisible(pingPongButton);
    pingPongAttachment = std::make_unique<ButtonAttachment>(apvts, "pingPong", pingPongButton);

    setupSlider(stereoOffsetSlider, stereoOffsetLabel, "Offset");
    stereoOffsetSlider.textFromValueFunction = [](double v) { return juce::String((int)v) + " ms"; };
    stereoOffsetSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue(); };
    stereoOffsetAttachment = std::make_unique<SliderAttachment>(apvts, "stereoOffset", stereoOffsetSlider);

    // TONE
    setupSlider(highPassSlider, highPassLabel, "High");
    highPassSlider.setSkewFactorFromMidPoint(200.0);
    highPassSlider.textFromValueFunction = [](double v) { return juce::String((int)v) + " Hz"; };
    highPassSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue(); };
    highPassAttachment = std::make_unique<SliderAttachment>(apvts, "highPass", highPassSlider);

    setupSlider(lowPassSlider, lowPassLabel, "Low");
    lowPassSlider.setSkewFactorFromMidPoint(4000.0);
    lowPassSlider.textFromValueFunction = [](double v)
    {
        if (v >= 1000.0) return juce::String(v / 1000.0, 1) + " kHz";
        return juce::String((int)v) + " Hz";
    };
    lowPassSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue(); };
    lowPassAttachment = std::make_unique<SliderAttachment>(apvts, "lowPass", lowPassSlider);

    // MODULATION
    setupSlider(modRateSlider, modRateLabel, "Rate");
    modRateSlider.setSkewFactorFromMidPoint(1.5);
    modRateSlider.textFromValueFunction = [](double v) { return juce::String(v, 1) + " Hz"; };
    modRateSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue(); };
    modRateAttachment = std::make_unique<SliderAttachment>(apvts, "modRate", modRateSlider);

    setupSlider(modDepthSlider, modDepthLabel, "Depth");
    modDepthSlider.textFromValueFunction = [](double v) { return juce::String((int)(v * 100.0)) + "%"; };
    modDepthSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    modDepthAttachment = std::make_unique<SliderAttachment>(apvts, "modDepth", modDepthSlider);

    setupSlider(driveSlider, driveLabel, "Drive");
    driveSlider.textFromValueFunction = [](double v) { return juce::String((int)(v * 100.0)) + "%"; };
    driveSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    driveAttachment = std::make_unique<SliderAttachment>(apvts, "drive", driveSlider);

    // DUCK
    setupSlider(duckAmountSlider, duckAmountLabel, "Amount");
    duckAmountSlider.textFromValueFunction = [](double v) { return juce::String((int)(v * 100.0)) + "%"; };
    duckAmountSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    duckAmountAttachment = std::make_unique<SliderAttachment>(apvts, "duckAmount", duckAmountSlider);

    setupSlider(duckThresholdSlider, duckThresholdLabel, "Threshold");
    duckThresholdSlider.textFromValueFunction = [](double v) { return juce::String((int)(v * 100.0)) + "%"; };
    duckThresholdSlider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
    duckThresholdAttachment = std::make_unique<SliderAttachment>(apvts, "duckThreshold", duckThresholdSlider);

    apvts.addParameterListener("mode", this);
    isAnalogMode = apvts.getRawParameterValue("mode")->load() >= 0.5f;
    updateModeButtons();
    updateModulationEnabled();

    setSize(660, 470);
    setResizable(true, true);
    setResizeLimits(440, 330, 1200, 900);
}

GenXDelayEditor::~GenXDelayEditor()
{
    processorRef.getAPVTS().removeParameterListener("mode", this);
    setLookAndFeel(nullptr);
}

void GenXDelayEditor::setupSlider(juce::Slider& slider, juce::Label& label,
                                    const juce::String& text)
{
    slider.setSliderStyle(juce::Slider::RotaryHorizontalVerticalDrag);
    slider.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 55, 14);
    addAndMakeVisible(slider);

    label.setText(text, juce::dontSendNotification);
    label.setFont(bodyFont.withHeight(10.0f));
    label.setColour(juce::Label::textColourId, textDim);
    label.setJustificationType(juce::Justification::centred);
    addAndMakeVisible(label);
}

void GenXDelayEditor::updateModeButtons()
{
    if (isAnalogMode)
    {
        digitalModeButton.setColour(juce::TextButton::buttonColourId, bgRidge);
        digitalModeButton.setColour(juce::TextButton::textColourOffId, textDim);
        analogModeButton.setColour(juce::TextButton::buttonColourId, accentOrange);
        analogModeButton.setColour(juce::TextButton::textColourOffId, bgBlack);
    }
    else
    {
        digitalModeButton.setColour(juce::TextButton::buttonColourId, accentOrange);
        digitalModeButton.setColour(juce::TextButton::textColourOffId, bgBlack);
        analogModeButton.setColour(juce::TextButton::buttonColourId, bgRidge);
        analogModeButton.setColour(juce::TextButton::textColourOffId, textDim);
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
    modRateSlider.setAlpha(alpha);
    modRateLabel.setAlpha(alpha);
    modDepthSlider.setAlpha(alpha);
    modDepthLabel.setAlpha(alpha);
    driveSlider.setAlpha(alpha);
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
    const float scale = std::min(w / 660.0f, h / 470.0f);

    // --- Background: matte black ---
    g.fillAll(bgBlack);

    // --- Walnut woodgrain band on bottom ~45% ---
    const float stripeY = h * 0.53f;

    g.setColour(bgWood);
    g.fillRect(0.0f, stripeY + 2.0f, w, h - stripeY - 2.0f);

    // Subtle horizontal grain lines
    g.setColour(bgWoodDark.withAlpha(0.35f));
    for (float gy = stripeY + 6.0f; gy < h; gy += 4.0f)
        g.drawHorizontalLine((int)gy, 0.0f, w);

    // --- Ribbed plastic texture in the top black area ---
    {
        const int titleHeight = (int)(40.0f * scale);
        g.setColour(juce::Colour(32, 32, 36).withAlpha(0.5f));
        for (float ry = (float)titleHeight; ry < stripeY; ry += 3.0f)
            g.drawHorizontalLine((int)ry, 0.0f, w);
    }

    // --- Orange accent stripe ---
    g.setColour(accentOrange);
    g.fillRect(0.0f, stripeY, w, 2.0f);

    // --- Title: "GENX DELAY" in Bebas Neue, chrome silver ---
    const int titleHeight = (int)(40.0f * scale);
    auto titleArea = bounds.removeFromTop(titleHeight);

    g.setColour(textSilver);
    g.setFont(titleFont.withHeight(28.0f * scale));
    g.drawText("GENX DELAY", titleArea, juce::Justification::centred);

    // Orange underline below title
    {
        const float underlineW = 90.0f * scale;
        const float underlineX = (w - underlineW) * 0.5f;
        const float underlineY = titleArea.getBottom() - 3.0f * scale;
        g.setColour(accentOrange.withAlpha(0.7f));
        g.fillRect(underlineX, underlineY, underlineW, 1.5f);
    }

    // --- Section panels ---
    auto area = getLocalBounds();
    area.removeFromTop(titleHeight);
    const int margin = (int)(10.0f * scale);
    area = area.reduced(margin);

    int columns;
    if (w >= 560.0f) columns = 3;
    else if (w >= 380.0f) columns = 2;
    else columns = 1;

    auto sections = calculateSectionBounds(area, columns);

    const float cornerRadius = 4.0f;

    for (auto& section : sections)
    {
        auto sb = section.bounds.toFloat();

        // Panel background — dark plastic
        g.setColour(bgRidge);
        g.fillRoundedRectangle(sb, cornerRadius);

        // Thin orange accent line at top of panel
        juce::Colour accent = accentOrange;
        if (section.sectionIndex == 4 && !isAnalogMode)
            accent = accent.withAlpha(0.35f);

        g.setColour(accent);
        g.fillRect(sb.getX() + cornerRadius, sb.getY(), sb.getWidth() - cornerRadius * 2.0f, 1.0f);

        // Section header text in orange
        auto headerArea = section.bounds;
        auto headerRow = headerArea.removeFromTop((int)(20.0f * scale));

        g.setColour(accent);
        g.setFont(headerFont.withHeight(11.0f * scale));
        g.drawText(getSectionName(section.sectionIndex), headerRow.reduced(8, 0),
                   juce::Justification::centredLeft);

        // "(Analog only)" subtitle
        if (section.sectionIndex == 4)
        {
            g.setFont(bodyFont.withHeight(9.0f * scale));
            g.setColour(accent.withAlpha(0.7f));
            auto subtitleArea = headerRow;
            subtitleArea.setLeft(headerRow.getX() + (int)(90.0f * scale));
            g.drawText("(Analog only)", subtitleArea, juce::Justification::centredLeft);
        }
    }
}

void GenXDelayEditor::resized()
{
    auto area = getLocalBounds();
    const float w = (float)area.getWidth();
    const float h = (float)area.getHeight();
    const float scale = std::min(w / 660.0f, h / 470.0f);

    const int titleHeight = (int)(40.0f * scale);
    area.removeFromTop(titleHeight);

    const int margin = (int)(10.0f * scale);
    area = area.reduced(margin);

    int columns;
    if (w >= 560.0f) columns = 3;
    else if (w >= 380.0f) columns = 2;
    else columns = 1;

    const int numSections = 6;
    const int gap = (int)(6.0f * scale);
    const int colGap = (int)(6.0f * scale);

    for (int i = 0; i < numSections; i += columns)
    {
        int sectionsInRow = std::min(columns, numSections - i);
        int colWidth = (area.getWidth() - colGap * (sectionsInRow - 1)) / sectionsInRow;

        auto estimateHeight = [&](int idx) -> int
        {
            const int knobRow = (int)(80.0f * scale);
            const int toggleRow = (int)(22.0f * scale);
            const int headerH = (int)(20.0f * scale);
            const int pad = (int)(8.0f * scale);
            switch (idx)
            {
                case 0: return headerH + knobRow + toggleRow * 2 + pad * 2;
                case 1: return headerH + knobRow + knobRow + toggleRow + pad * 2;
                case 2: return headerH + knobRow + toggleRow + pad * 2;
                case 3: return headerH + knobRow + pad * 2;
                case 4: return headerH + knobRow + knobRow + pad * 2;
                case 5: return headerH + knobRow + pad * 2;
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
    const float scale = std::min(w / 660.0f, h / 470.0f);

    const int headerH = (int)(20.0f * scale);
    const int knobSize = (int)(52.0f * scale);
    const int valueH = (int)(14.0f * scale);
    const int labelH = (int)(14.0f * scale);
    const int toggleH = (int)(22.0f * scale);
    const int pad = (int)(4.0f * scale);

    area.removeFromTop(headerH);
    area = area.reduced((int)(4.0f * scale), 0);

    auto placeKnob = [&](juce::Slider& slider, juce::Label& label, juce::Rectangle<int> knobArea)
    {
        int cx = knobArea.getCentreX();
        slider.setBounds(cx - knobSize / 2, knobArea.getY(), knobSize, knobSize + valueH);
        label.setBounds(cx - knobSize / 2 - 5, knobArea.getY() + knobSize + valueH, knobSize + 10, labelH);
    };

    const int knobRowH = knobSize + valueH + labelH;

    switch (sectionIndex)
    {
        case 0: // TIME
        {
            atariLnf.sectionColor = accentOrange;

            auto knobArea = area.removeFromTop(knobRowH);
            placeKnob(delayTimeSlider, delayTimeLabel, knobArea);

            area.removeFromTop(pad);

            auto toggleRow = area.removeFromTop(toggleH);
            int halfW = toggleRow.getWidth() / 2;
            reverseButton.setBounds(toggleRow.removeFromLeft(halfW));
            tempoSyncButton.setBounds(toggleRow);

            area.removeFromTop(pad);

            auto comboRow = area.removeFromTop(toggleH);
            int comboW = (int)(90.0f * scale);
            int comboX = comboRow.getCentreX() - comboW / 2;
            noteDivisionBox.setBounds(comboX, comboRow.getY(), comboW, comboRow.getHeight());
            noteDivisionLabel.setBounds(0, 0, 0, 0);
            break;
        }

        case 1: // MAIN
        {
            atariLnf.sectionColor = accentOrange;

            auto topKnobRow = area.removeFromTop(knobRowH);
            int colW = topKnobRow.getWidth() / 2;
            placeKnob(feedbackSlider, feedbackLabel, topKnobRow.removeFromLeft(colW));
            placeKnob(mixSlider, mixLabel, topKnobRow);

            area.removeFromTop(pad);

            auto trimRow = area.removeFromTop(knobRowH);
            placeKnob(trimSlider, trimLabel, trimRow);

            area.removeFromTop(pad);

            auto modeRow = area.removeFromTop(toggleH);
            int modeW = (int)(100.0f * scale);
            int modeX = modeRow.getCentreX() - modeW;
            digitalModeButton.setBounds(modeX, modeRow.getY(), modeW, modeRow.getHeight());
            analogModeButton.setBounds(modeX + modeW, modeRow.getY(), modeW, modeRow.getHeight());
            break;
        }

        case 2: // STEREO
        {
            atariLnf.sectionColor = accentOrange;

            auto knobArea = area.removeFromTop(knobRowH);
            placeKnob(stereoOffsetSlider, stereoOffsetLabel, knobArea);

            area.removeFromTop(pad);

            auto toggleRow = area.removeFromTop(toggleH);
            int toggleW = (int)(100.0f * scale);
            pingPongButton.setBounds(toggleRow.getCentreX() - toggleW / 2, toggleRow.getY(),
                                     toggleW, toggleRow.getHeight());
            break;
        }

        case 3: // TONE
        {
            atariLnf.sectionColor = accentOrange;

            auto knobRow = area.removeFromTop(knobRowH);
            int colW = knobRow.getWidth() / 2;
            placeKnob(highPassSlider, highPassLabel, knobRow.removeFromLeft(colW));
            placeKnob(lowPassSlider, lowPassLabel, knobRow);
            break;
        }

        case 4: // MODULATION
        {
            atariLnf.sectionColor = accentOrange;

            auto topKnobRow = area.removeFromTop(knobRowH);
            int colW = topKnobRow.getWidth() / 2;
            placeKnob(modRateSlider, modRateLabel, topKnobRow.removeFromLeft(colW));
            placeKnob(modDepthSlider, modDepthLabel, topKnobRow);

            area.removeFromTop(pad);

            auto driveRow = area.removeFromTop(knobRowH);
            placeKnob(driveSlider, driveLabel, driveRow);
            break;
        }

        case 5: // DUCK
        {
            atariLnf.sectionColor = accentOrange;

            auto knobRow = area.removeFromTop(knobRowH);
            int colW = knobRow.getWidth() / 2;
            placeKnob(duckAmountSlider, duckAmountLabel, knobRow.removeFromLeft(colW));
            placeKnob(duckThresholdSlider, duckThresholdLabel, knobRow);
            break;
        }
    }
}

std::vector<GenXDelayEditor::SectionBounds>
GenXDelayEditor::calculateSectionBounds(juce::Rectangle<int> area, int columns) const
{
    const float w = (float)getWidth();
    const float h = (float)getHeight();
    const float scale = std::min(w / 660.0f, h / 470.0f);

    const int gap = (int)(6.0f * scale);
    const int colGap = (int)(6.0f * scale);
    const int numSections = 6;

    auto estimateHeight = [&](int idx) -> int
    {
        const int knobRow = (int)(80.0f * scale);
        const int toggleRow = (int)(22.0f * scale);
        const int headerH = (int)(20.0f * scale);
        const int pad = (int)(8.0f * scale);
        switch (idx)
        {
            case 0: return headerH + knobRow + toggleRow * 2 + pad * 2;
            case 1: return headerH + knobRow + knobRow + toggleRow + pad * 2;
            case 2: return headerH + knobRow + toggleRow + pad * 2;
            case 3: return headerH + knobRow + pad * 2;
            case 4: return headerH + knobRow + knobRow + pad * 2;
            case 5: return headerH + knobRow + pad * 2;
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
