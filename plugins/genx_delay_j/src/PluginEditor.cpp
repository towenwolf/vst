#include "PluginEditor.h"
#include "BinaryData.h"

using namespace PioneerColors;

//==============================================================================
// PioneerLookAndFeel
//==============================================================================

PioneerLookAndFeel::PioneerLookAndFeel()
{
    setColour(juce::ResizableWindow::backgroundColourId, bgBlack);
    setColour(juce::Label::textColourId, vfdAmber);
    setColour(juce::Slider::textBoxTextColourId, vfdAmber);
    setColour(juce::Slider::textBoxBackgroundColourId, juce::Colours::transparentBlack);
    setColour(juce::Slider::textBoxOutlineColourId, juce::Colours::transparentBlack);
    setColour(juce::TextButton::buttonColourId, bgPanel);
    setColour(juce::TextButton::textColourOffId, vfdAmber);
    setColour(juce::ComboBox::backgroundColourId, displayBg);
    setColour(juce::ComboBox::textColourId, vfdAmber);
    setColour(juce::ComboBox::outlineColourId, vfdAmberDim.withAlpha(0.3f));
    setColour(juce::ComboBox::arrowColourId, vfdAmberDim);
    setColour(juce::PopupMenu::backgroundColourId, bgPanel);
    setColour(juce::PopupMenu::textColourId, vfdAmber);
    setColour(juce::PopupMenu::highlightedBackgroundColourId, vfdAmber);
    setColour(juce::PopupMenu::highlightedTextColourId, bgBlack);
    setColour(juce::ToggleButton::textColourId, vfdAmber);
    setColour(juce::ToggleButton::tickColourId, vfdAmber);
    setColour(juce::ToggleButton::tickDisabledColourId, vfdAmberDim.withAlpha(0.3f));
    setColour(juce::CaretComponent::caretColourId, vfdAmber);
}

void PioneerLookAndFeel::drawRotarySlider(juce::Graphics& g, int x, int y, int width, int height,
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
    const float valueAngle = rotaryStartAngle + sliderPos * (rotaryEndAngle - rotaryStartAngle);

    // 1. Amber glow halo
    {
        const float glowAlpha = isHovered ? 0.12f : 0.04f;
        juce::ColourGradient glow(vfdAmber.withAlpha(glowAlpha * enabledAlpha), centreX, centreY,
                                   vfdAmber.withAlpha(0.0f), centreX, centreY - radius, true);
        g.setGradientFill(glow);
        g.fillEllipse(centreX - radius - 2.0f, centreY - radius - 2.0f,
                       diameter + 4.0f, diameter + 4.0f);
    }

    // 2. Drop shadow
    {
        g.setColour(juce::Colours::black.withAlpha(0.4f * enabledAlpha));
        const float shadowR = radius - 1.0f;
        g.fillEllipse(centreX - shadowR + 0.5f, centreY - shadowR + 1.5f,
                       shadowR * 2.0f, shadowR * 2.0f);
    }

    // 3. Dark knob body
    {
        const float bodyRadius = radius - 2.0f;
        juce::ColourGradient bodyGrad(
            knobBody.brighter(isHovered ? 0.15f : 0.05f).withAlpha(enabledAlpha),
            centreX - bodyRadius * 0.3f, centreY - bodyRadius * 0.3f,
            juce::Colour(15, 15, 18).withAlpha(enabledAlpha),
            centreX + bodyRadius * 0.5f, centreY + bodyRadius * 0.5f, true);
        g.setGradientFill(bodyGrad);
        g.fillEllipse(centreX - bodyRadius, centreY - bodyRadius,
                       bodyRadius * 2.0f, bodyRadius * 2.0f);
    }

    // 4. Ribbed grip lines (subtle radial lines)
    {
        const float bodyRadius = radius - 2.0f;
        g.setColour(juce::Colours::white.withAlpha(0.03f * enabledAlpha));
        for (int ri = 0; ri < 24; ++ri)
        {
            float angle = (float)ri * juce::MathConstants<float>::twoPi / 24.0f;
            float x1 = centreX + (bodyRadius * 0.4f) * std::sin(angle);
            float y1 = centreY - (bodyRadius * 0.4f) * std::cos(angle);
            float x2 = centreX + (bodyRadius * 0.9f) * std::sin(angle);
            float y2 = centreY - (bodyRadius * 0.9f) * std::cos(angle);
            g.drawLine(x1, y1, x2, y2, 0.5f);
        }
    }

    // 5. Edge ring
    {
        const float bodyRadius = radius - 2.0f;
        g.setColour(knobEdge.withAlpha(0.6f * enabledAlpha));
        g.drawEllipse(centreX - bodyRadius, centreY - bodyRadius,
                       bodyRadius * 2.0f, bodyRadius * 2.0f, 1.0f);
    }

    // 6. Track arc (background)
    {
        juce::Path trackArc;
        trackArc.addCentredArc(centreX, centreY, arcRadius, arcRadius, 0.0f,
                               rotaryStartAngle, rotaryEndAngle, true);
        g.setColour(knobEdge.withAlpha(0.4f * enabledAlpha));
        g.strokePath(trackArc, juce::PathStrokeType(2.0f));
    }

    // 7. Value arc — amber with VFD glow
    if (sliderPos > 0.0f)
    {
        // Outer glow
        juce::Path glowArc;
        glowArc.addCentredArc(centreX, centreY, arcRadius, arcRadius, 0.0f,
                               rotaryStartAngle, valueAngle, true);
        g.setColour(vfdAmber.withAlpha(0.12f * enabledAlpha));
        g.strokePath(glowArc, juce::PathStrokeType(6.0f));

        // Main value arc
        juce::Path valueArc;
        valueArc.addCentredArc(centreX, centreY, arcRadius, arcRadius, 0.0f,
                               rotaryStartAngle, valueAngle, true);
        g.setColour(vfdAmber.withAlpha(enabledAlpha));
        g.strokePath(valueArc, juce::PathStrokeType(2.5f));
    }

    // 8. Pointer indicator line on knob body (like Pioneer's red line)
    {
        const float bodyRadius = radius - 2.0f;
        const float lineInner = bodyRadius * 0.25f;
        const float lineOuter = bodyRadius * 0.75f;
        float px1 = centreX + lineInner * std::sin(valueAngle);
        float py1 = centreY - lineInner * std::cos(valueAngle);
        float px2 = centreX + lineOuter * std::sin(valueAngle);
        float py2 = centreY - lineOuter * std::cos(valueAngle);

        // Glow
        g.setColour(indicatorRed.withAlpha(0.3f * enabledAlpha));
        g.drawLine(px1, py1, px2, py2, 3.0f);

        // Crisp line
        g.setColour(indicatorRed.withAlpha(enabledAlpha));
        g.drawLine(px1, py1, px2, py2, 1.5f);
    }

    // 9. Center cap
    {
        const float cdRadius = 2.0f;
        g.setColour(juce::Colour(8, 8, 10).withAlpha(enabledAlpha));
        g.fillEllipse(centreX - cdRadius, centreY - cdRadius, cdRadius * 2.0f, cdRadius * 2.0f);
    }
}

void PioneerLookAndFeel::drawToggleButton(juce::Graphics& g, juce::ToggleButton& button,
                                            bool shouldDrawButtonAsHighlighted, bool /*shouldDrawButtonAsDown*/)
{
    const float btnW = 28.0f;
    const float btnH = 14.0f;
    const float btnY = ((float)button.getHeight() - btnH) * 0.5f;

    juce::Rectangle<float> btnBounds(4.0f, btnY, btnW, btnH);

    if (button.getToggleState())
    {
        // Lit amber glow behind
        g.setColour(vfdAmber.withAlpha(button.isEnabled() ? 0.15f : 0.05f));
        g.fillRect(btnBounds.expanded(2.0f));

        // Lit button face
        g.setColour(button.isEnabled() ? vfdAmber : vfdAmber.withAlpha(0.3f));
        g.fillRect(btnBounds);

        // Inner highlight
        g.setColour(vfdAmberGlow.withAlpha(0.3f));
        g.fillRect(btnBounds.removeFromTop(2.0f));
    }
    else
    {
        // Dark recessed button
        g.setColour(button.isEnabled() ? displayBg : displayBg.withAlpha(0.5f));
        g.fillRect(btnBounds);

        // Border
        g.setColour(knobEdge.withAlpha(0.5f));
        g.drawRect(btnBounds, 1.0f);
    }

    if (shouldDrawButtonAsHighlighted)
    {
        g.setColour(vfdAmber.withAlpha(0.08f));
        g.fillRect(btnBounds.expanded(2.0f));
    }

    const float textX = 4.0f + btnW + 6.0f;
    juce::Colour textCol = button.getToggleState() ? vfdAmber : vfdAmberDim;
    if (!button.isEnabled())
        textCol = textCol.withAlpha(0.3f);
    g.setColour(textCol);
    g.setFont(segmentFont.withHeight(12.0f));
    g.drawText(button.getButtonText(),
               juce::Rectangle<float>(textX, 0.0f, (float)button.getWidth() - textX, (float)button.getHeight()),
               juce::Justification::centredLeft);
}

void PioneerLookAndFeel::drawComboBox(juce::Graphics& g, int width, int height, bool /*isButtonDown*/,
                                        int /*buttonX*/, int /*buttonY*/, int /*buttonW*/, int /*buttonH*/,
                                        juce::ComboBox& box)
{
    auto bounds = juce::Rectangle<float>(0.0f, 0.0f, (float)width, (float)height);

    // Dark recessed display
    g.setColour(displayBg);
    g.fillRect(bounds);

    // Amber border
    g.setColour(vfdAmberDim.withAlpha(0.3f));
    g.drawRect(bounds, 1.0f);

    // Arrow
    const float arrowSize = 6.0f;
    const float arrowX = (float)width - 16.0f;
    const float arrowY = ((float)height - arrowSize * 0.5f) * 0.5f;

    juce::Path arrow;
    arrow.addTriangle(arrowX, arrowY,
                      arrowX + arrowSize, arrowY,
                      arrowX + arrowSize * 0.5f, arrowY + arrowSize * 0.5f);

    g.setColour(box.isEnabled() ? vfdAmber : vfdAmberDim.withAlpha(0.3f));
    g.fillPath(arrow);
}

void PioneerLookAndFeel::drawPopupMenuBackground(juce::Graphics& g, int width, int height)
{
    g.fillAll(bgPanel);
    g.setColour(vfdAmberDim.withAlpha(0.3f));
    g.drawRect(0, 0, width, height);
}

void PioneerLookAndFeel::drawPopupMenuItem(juce::Graphics& g, const juce::Rectangle<int>& area,
                                             bool /*isSeparator*/, bool isActive, bool isHighlighted,
                                             bool isTicked, bool /*hasSubMenu*/,
                                             const juce::String& text, const juce::String& /*shortcutKeyText*/,
                                             const juce::Drawable* /*icon*/, const juce::Colour* /*textColour*/)
{
    if (isHighlighted && isActive)
    {
        g.setColour(vfdAmber);
        g.fillRect(area);
        g.setColour(bgBlack);
    }
    else
    {
        g.setColour(isActive ? vfdAmber : vfdAmber.withAlpha(0.4f));
    }

    auto textArea = area.reduced(8, 0);
    g.setFont(segmentFont.withHeight(11.0f));
    g.drawText(text, textArea, juce::Justification::centredLeft);

    if (isTicked)
    {
        g.setColour(isHighlighted ? bgBlack : vfdAmber);
        auto tickBounds = area.withLeft(area.getRight() - area.getHeight()).reduced(5);
        g.fillEllipse(tickBounds.toFloat());
    }
}

void PioneerLookAndFeel::drawLabel(juce::Graphics& g, juce::Label& label)
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

void PioneerLookAndFeel::drawButtonText(juce::Graphics& g, juce::TextButton& button,
                                          bool /*shouldDrawButtonAsHighlighted*/,
                                          bool /*shouldDrawButtonAsDown*/)
{
    g.setFont(segmentFont.withHeight(12.0f));
    g.setColour(button.findColour(button.getToggleState() ? juce::TextButton::textColourOnId
                                                          : juce::TextButton::textColourOffId));
    g.drawText(button.getButtonText(), button.getLocalBounds(),
               juce::Justification::centred, true);
}

juce::Font PioneerLookAndFeel::getComboBoxFont(juce::ComboBox& /*box*/)
{
    return segmentFont.withHeight(11.0f);
}

juce::Font PioneerLookAndFeel::getPopupMenuFont()
{
    return segmentFont.withHeight(11.0f);
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
    // Load DSEG fonts
    auto dseg14Typeface = juce::Typeface::createSystemTypefaceFor(
        BinaryData::DSEG14ClassicRegular_ttf, BinaryData::DSEG14ClassicRegular_ttfSize);
    auto dseg7Typeface = juce::Typeface::createSystemTypefaceFor(
        BinaryData::DSEG7ClassicRegular_ttf, BinaryData::DSEG7ClassicRegular_ttfSize);

    titleFont  = juce::Font(juce::FontOptions(dseg14Typeface));
    headerFont = juce::Font(juce::FontOptions(dseg14Typeface));
    bodyFont   = juce::Font(juce::FontOptions(dseg14Typeface));
    numFont    = juce::Font(juce::FontOptions(dseg7Typeface));

    pioneerLnf.setFonts(titleFont, numFont);
    setLookAndFeel(&pioneerLnf);

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

    setSize(800, 580);
    setResizable(true, true);
    setResizeLimits(760, 552, 1520, 1100);
    if (auto* c = getConstrainer())
        c->setFixedAspectRatio(800.0 / 580.0);
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
    slider.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 55, 16);
    addAndMakeVisible(slider);

    label.setText(text, juce::dontSendNotification);
    label.setFont(bodyFont.withHeight(11.0f).boldened());
    label.setColour(juce::Label::textColourId, vfdAmberDim);
    label.setJustificationType(juce::Justification::centred);
    addAndMakeVisible(label);
}

void GenXDelayEditor::updateModeButtons()
{
    if (isAnalogMode)
    {
        digitalModeButton.setColour(juce::TextButton::buttonColourId, bgPanel);
        digitalModeButton.setColour(juce::TextButton::textColourOffId, vfdAmberDim);
        analogModeButton.setColour(juce::TextButton::buttonColourId, vfdAmber);
        analogModeButton.setColour(juce::TextButton::textColourOffId, bgBlack);
    }
    else
    {
        digitalModeButton.setColour(juce::TextButton::buttonColourId, vfdAmber);
        digitalModeButton.setColour(juce::TextButton::textColourOffId, bgBlack);
        analogModeButton.setColour(juce::TextButton::buttonColourId, bgPanel);
        analogModeButton.setColour(juce::TextButton::textColourOffId, vfdAmberDim);
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
// VFD glow text helper — draws text with multi-layer phosphor bloom
//==============================================================================
void GenXDelayEditor::drawVFDText(juce::Graphics& g, const juce::String& text,
                                    juce::Rectangle<int> area, const juce::Font& font,
                                    juce::Justification just, float glowIntensity)
{
    g.setFont(font);

    // Layer 1: Wide soft glow
    g.setColour(vfdAmber.withAlpha(0.06f * glowIntensity));
    g.drawText(text, area.expanded(4, 3), just, true);

    // Layer 2: Medium glow
    g.setColour(vfdAmber.withAlpha(0.12f * glowIntensity));
    g.drawText(text, area.expanded(2, 1), just, true);

    // Layer 3: Crisp bright text
    g.setColour(vfdAmberGlow.withAlpha(0.9f * glowIntensity));
    g.drawText(text, area, just, true);
}

//==============================================================================
void GenXDelayEditor::paint(juce::Graphics& g)
{
    auto bounds = getLocalBounds();
    const float w = (float)bounds.getWidth();
    const float h = (float)bounds.getHeight();
    const float scale = std::min(w / 800.0f, h / 580.0f);

    const int margin = (int)(10.0f * scale);

    // --- Chassis body: deep matte black ---
    g.fillAll(bgBlack);

    // Subtle horizontal brushed-metal grain texture
    {
        juce::Random rng(42);
        for (int py = 0; py < (int)h; py += 2)
        {
            float rowAlpha = 0.01f + rng.nextFloat() * 0.015f;
            g.setColour(juce::Colours::white.withAlpha(rowAlpha));
            g.drawHorizontalLine(py, 0.0f, w);
        }
    }

    // --- Recessed display window ---
    const int displayTop = (int)(6.0f * scale);
    const int displayHeight = (int)(h * 0.05f);
    auto displayRect = juce::Rectangle<int>(margin, displayTop,
                                             (int)w - margin * 2, displayHeight);

    // Bevel: dark inner shadow edges
    g.setColour(juce::Colours::black.withAlpha(0.6f));
    g.drawRect(displayRect.expanded(1), 1);

    // Display background
    g.setColour(displayBg);
    g.fillRect(displayRect);

    // Bottom-right highlight edge (bevel illusion)
    g.setColour(chassisGrey.withAlpha(0.3f));
    g.drawHorizontalLine(displayRect.getBottom(), (float)displayRect.getX(), (float)displayRect.getRight());
    g.drawVerticalLine(displayRect.getRight(), (float)displayRect.getY(), (float)displayRect.getBottom());

    // Top-left dark edge (inset bevel)
    g.setColour(juce::Colours::black.withAlpha(0.4f));
    g.drawHorizontalLine(displayRect.getY(), (float)displayRect.getX(), (float)displayRect.getRight());
    g.drawVerticalLine(displayRect.getX(), (float)displayRect.getY(), (float)displayRect.getBottom());

    // --- Horizontal divider ridge between display and control area ---
    const float dividerY = (float)(displayRect.getBottom()) + 2.0f * scale;
    g.setColour(chassisGrey.withAlpha(0.4f));
    g.drawHorizontalLine((int)dividerY, (float)margin, w - (float)margin);
    g.setColour(juce::Colours::black.withAlpha(0.5f));
    g.drawHorizontalLine((int)dividerY + 1, (float)margin, w - (float)margin);

    // --- Title: "GENX DELAY" with VFD glow ---
    const int titleHeight = (int)(32.0f * scale);
    auto titleArea = displayRect.removeFromTop(titleHeight);

    drawVFDText(g, "GENX DELAY", titleArea, titleFont.withHeight(24.0f * scale));

    // Amber underline with glow
    {
        const float underlineW = 120.0f * scale;
        const float underlineX = ((float)titleArea.getX() + (float)titleArea.getWidth() * 0.5f) - underlineW * 0.5f;
        const float underlineY = (float)titleArea.getBottom() - 2.0f * scale;

        // Glow
        g.setColour(vfdAmber.withAlpha(0.08f));
        g.fillRect(underlineX - 3.0f, underlineY - 1.0f, underlineW + 6.0f, 4.0f);

        // Crisp line
        g.setColour(vfdAmber.withAlpha(0.7f));
        g.fillRect(underlineX, underlineY, underlineW, 1.0f);
    }

    // --- Section panels below divider ---
    auto area = juce::Rectangle<int>(margin, (int)(dividerY + 2.0f * scale),
                                      (int)w - margin * 2, (int)h - (int)(dividerY + 2.0f * scale));
    const int sectionMargin = (int)(3.0f * scale);
    area = area.reduced(sectionMargin);

    int columns;
    if (w >= 560.0f) columns = 3;
    else if (w >= 380.0f) columns = 2;
    else columns = 1;

    auto sections = calculateSectionBounds(area, columns);

    for (auto& section : sections)
    {
        auto sb = section.bounds.toFloat();

        // Thin amber outline rectangle (VFD sub-section)
        float accentAlpha = 0.25f;
        if (section.sectionIndex == 4 && !isAnalogMode)
            accentAlpha = 0.10f;
        g.setColour(vfdAmber.withAlpha(accentAlpha));
        g.drawRoundedRectangle(sb, 2.0f, 0.5f);

        // Section header with VFD glow
        auto headerRow = section.bounds;
        auto headerArea = headerRow.removeFromTop((int)(22.0f * scale));

        float headerGlow = (section.sectionIndex == 4 && !isAnalogMode) ? 0.35f : 1.0f;
        drawVFDText(g, getSectionName(section.sectionIndex), headerArea.reduced(6, 0),
                    headerFont.withHeight(12.0f * scale),
                    juce::Justification::centredLeft, headerGlow);

    }

    // --- Vignette darkening at edges ---
    {
        juce::ColourGradient topVig(juce::Colours::black.withAlpha(0.2f), 0.0f, 0.0f,
                                     juce::Colours::transparentBlack, 0.0f, 12.0f * scale, false);
        g.setGradientFill(topVig);
        g.fillRect(0.0f, 0.0f, w, 12.0f * scale);

        juce::ColourGradient botVig(juce::Colours::black.withAlpha(0.25f), 0.0f, h,
                                     juce::Colours::transparentBlack, 0.0f, h - 12.0f * scale, false);
        g.setGradientFill(botVig);
        g.fillRect(0.0f, h - 12.0f * scale, w, 12.0f * scale);

        juce::ColourGradient leftVig(juce::Colours::black.withAlpha(0.15f), 0.0f, 0.0f,
                                      juce::Colours::transparentBlack, 10.0f * scale, 0.0f, false);
        g.setGradientFill(leftVig);
        g.fillRect(0.0f, 0.0f, 10.0f * scale, h);

        juce::ColourGradient rightVig(juce::Colours::black.withAlpha(0.15f), w, 0.0f,
                                       juce::Colours::transparentBlack, w - 10.0f * scale, 0.0f, false);
        g.setGradientFill(rightVig);
        g.fillRect(w - 10.0f * scale, 0.0f, 10.0f * scale, h);
    }
}

void GenXDelayEditor::resized()
{
    auto area = getLocalBounds();
    const float w = (float)area.getWidth();
    const float h = (float)area.getHeight();
    const float scale = std::min(w / 800.0f, h / 580.0f);

    const int margin = (int)(10.0f * scale);

    // --- Control area below divider ---
    const int displayTop = (int)(6.0f * scale);
    const int displayHeight = (int)(h * 0.05f);
    const float dividerY = (float)(displayTop + displayHeight) + 2.0f * scale;

    area = juce::Rectangle<int>(margin, (int)(dividerY + 2.0f * scale),
                                 (int)w - margin * 2, (int)h - (int)(dividerY + 2.0f * scale));
    const int sectionMargin = (int)(3.0f * scale);
    area = area.reduced(sectionMargin);

    int columns;
    if (w >= 560.0f) columns = 3;
    else if (w >= 380.0f) columns = 2;
    else columns = 1;

    const int numSections = 6;
    const int gap = (int)(3.0f * scale);
    const int colGap = (int)(4.0f * scale);

    for (int i = 0; i < numSections; i += columns)
    {
        int sectionsInRow = std::min(columns, numSections - i);
        int colWidth = (area.getWidth() - colGap * (sectionsInRow - 1)) / sectionsInRow;

        auto estimateHeight = [&](int idx) -> int
        {
            const int knobRow = (int)(100.0f * scale);
            const int toggleRow = (int)(28.0f * scale);
            const int headerH = (int)(22.0f * scale);
            const int pad = (int)(12.0f * scale);
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
    const float scale = std::min(w / 800.0f, h / 580.0f);

    const int headerH = (int)(22.0f * scale);
    const int knobSize = (int)(68.0f * scale);
    const int valueH = (int)(18.0f * scale);
    const int labelH = (int)(18.0f * scale);
    const int toggleH = (int)(28.0f * scale);
    const int pad = (int)(8.0f * scale);

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
            pioneerLnf.sectionColor = vfdAmber;

            int contentH = knobRowH + pad + toggleH + pad + toggleH;
            int vOffset = (area.getHeight() - contentH) / 2;
            if (vOffset > 0) area.removeFromTop(vOffset);

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
            pioneerLnf.sectionColor = vfdAmber;

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
            pioneerLnf.sectionColor = vfdAmber;

            int contentH = knobRowH + pad + toggleH;
            int vOffset = (area.getHeight() - contentH) / 2;
            if (vOffset > 0) area.removeFromTop(vOffset);

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
            pioneerLnf.sectionColor = vfdAmber;

            int contentH = knobRowH;
            int vOffset = (area.getHeight() - contentH) / 2;
            if (vOffset > 0) area.removeFromTop(vOffset);

            auto knobRow = area.removeFromTop(knobRowH);
            int colW = knobRow.getWidth() / 2;
            placeKnob(highPassSlider, highPassLabel, knobRow.removeFromLeft(colW));
            placeKnob(lowPassSlider, lowPassLabel, knobRow);
            break;
        }

        case 4: // MODULATION
        {
            pioneerLnf.sectionColor = vfdAmber;

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
            pioneerLnf.sectionColor = vfdAmber;

            int contentH = knobRowH;
            int vOffset = (area.getHeight() - contentH) / 2;
            if (vOffset > 0) area.removeFromTop(vOffset);

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
    const float scale = std::min(w / 800.0f, h / 580.0f);

    const int gap = (int)(3.0f * scale);
    const int colGap = (int)(4.0f * scale);
    const int numSections = 6;

    auto estimateHeight = [&](int idx) -> int
    {
        const int knobRow = (int)(100.0f * scale);
        const int toggleRow = (int)(28.0f * scale);
        const int headerH = (int)(22.0f * scale);
        const int pad = (int)(12.0f * scale);
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
