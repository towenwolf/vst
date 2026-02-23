#include "PluginEditor.h"
#include <BinaryData.h>

//==============================================================================
// GenXLookAndFeel
//==============================================================================
GenXLookAndFeel::GenXLookAndFeel(const GenXTheme& t) : theme(t) {}

//------------------------------------------------------------------------------
// Rotary Knob — 9-layer rendering
//------------------------------------------------------------------------------
void GenXLookAndFeel::drawRotarySlider(juce::Graphics& g, int x, int y,
    int width, int height, float sliderPos, float startAngle,
    float endAngle, juce::Slider& slider)
{
    const float enabledAlpha = slider.isEnabled() ? 1.0f : 0.3f;
    const auto bounds = juce::Rectangle<int>(x, y, width, height).toFloat();
    const float diameter = std::min(bounds.getWidth(), bounds.getHeight());
    const float radius = diameter * 0.5f;
    const auto centre = bounds.getCentre();
    const float angle = startAngle + sliderPos * (endAngle - startAngle);

    // Layer 1: Hover glow halo
    if (slider.isMouseOver())
    {
        juce::ColourGradient glow(
            theme.accentGlow.withAlpha(GenXKnob::hoverGlowAlpha * enabledAlpha),
            centre,
            theme.accentGlow.withAlpha(0.0f),
            centre.translated(radius * 1.3f, 0), true);
        g.setGradientFill(glow);
        g.fillEllipse(bounds.expanded(radius * 0.3f));
    }

    // Layer 2: Drop shadow
    g.setColour(GenXColors::knobShadow.withAlpha(GenXKnob::shadowAlpha * enabledAlpha));
    g.fillEllipse(bounds.reduced(diameter * 0.12f)
        .translated(GenXKnob::shadowOffset, GenXKnob::shadowOffset));

    // Layer 3: Knob body (matte dark gradient)
    {
        const auto knobBounds = bounds.reduced(diameter * 0.14f);
        juce::ColourGradient bodyGrad(
            GenXColors::knobSurface.brighter(0.08f).withAlpha(enabledAlpha),
            knobBounds.getX(), knobBounds.getY(),
            GenXColors::knobSurface.darker(0.05f).withAlpha(enabledAlpha),
            knobBounds.getRight(), knobBounds.getBottom(), false);
        g.setGradientFill(bodyGrad);
        g.fillEllipse(knobBounds);
    }

    // Layer 4: Grip lines
    {
        const float gripRadius = radius * 0.62f;
        g.setColour(GenXColors::knobGrip.withAlpha(GenXKnob::gripAlpha * enabledAlpha));
        for (int i = 0; i < (int)GenXKnob::gripLineCount; ++i)
        {
            float a = (float)i / GenXKnob::gripLineCount * juce::MathConstants<float>::twoPi;
            float inner = gripRadius * 0.7f;
            g.drawLine(
                centre.x + std::cos(a) * inner,
                centre.y + std::sin(a) * inner,
                centre.x + std::cos(a) * gripRadius,
                centre.y + std::sin(a) * gripRadius, 0.5f);
        }
    }

    // Layer 5: Edge ring
    {
        const auto ringBounds = bounds.reduced(diameter * 0.14f);
        g.setColour(GenXColors::knobEdge.withAlpha(enabledAlpha));
        g.drawEllipse(ringBounds, GenXKnob::edgeThickness);
    }

    // Layer 6: Track arc (background / inactive)
    {
        juce::Path trackArc;
        trackArc.addCentredArc(centre.x, centre.y, radius * 0.82f,
            radius * 0.82f, 0.0f, startAngle, endAngle, true);
        g.setColour(GenXColors::track.withAlpha(0.4f * enabledAlpha));
        g.strokePath(trackArc,
            juce::PathStrokeType(GenXKnob::arcThickness,
                juce::PathStrokeType::curved,
                juce::PathStrokeType::rounded));
    }

    // Layer 7: Value arc + glow
    if (sliderPos > 0.0f)
    {
        juce::Path valueArc;
        valueArc.addCentredArc(centre.x, centre.y, radius * 0.82f,
            radius * 0.82f, 0.0f, startAngle, angle, true);

        // Glow layer
        g.setColour(theme.accentGlow.withAlpha(0.15f * enabledAlpha));
        g.strokePath(valueArc,
            juce::PathStrokeType(GenXKnob::arcThickness + GenXKnob::arcGlowExtra,
                juce::PathStrokeType::curved,
                juce::PathStrokeType::rounded));

        // Main arc
        g.setColour(theme.accent.withAlpha(0.9f * enabledAlpha));
        g.strokePath(valueArc,
            juce::PathStrokeType(GenXKnob::arcThickness,
                juce::PathStrokeType::curved,
                juce::PathStrokeType::rounded));
    }

    // Layer 8: Pointer indicator
    {
        juce::Path pointer;
        float pointerInnerR = radius * GenXKnob::pointerInner;
        float pointerOuterR = radius * GenXKnob::pointerOuter;
        pointer.addLineSegment(juce::Line<float>(
            centre.x + std::sin(angle) * pointerInnerR,
            centre.y - std::cos(angle) * pointerInnerR,
            centre.x + std::sin(angle) * pointerOuterR,
            centre.y - std::cos(angle) * pointerOuterR), 0.0f);
        g.setColour(theme.accent.withAlpha(enabledAlpha));
        g.strokePath(pointer,
            juce::PathStrokeType(GenXKnob::pointerThickness,
                juce::PathStrokeType::curved,
                juce::PathStrokeType::rounded));
    }

    // Layer 9: Center cap
    {
        const float capSize = diameter * 0.12f;
        g.setColour(GenXColors::bg0.withAlpha(enabledAlpha));
        g.fillEllipse(centre.x - capSize, centre.y - capSize,
            capSize * 2.0f, capSize * 2.0f);
    }
}

//------------------------------------------------------------------------------
// Linear Slider — horizontal/vertical track with accent fill
//------------------------------------------------------------------------------
void GenXLookAndFeel::drawLinearSlider(juce::Graphics& g, int x, int y,
    int width, int height, float sliderPos, float minSliderPos, float maxSliderPos,
    juce::Slider::SliderStyle style, juce::Slider& slider)
{
    const float enabledAlpha = slider.isEnabled() ? 1.0f : 0.3f;
    const bool isHorizontal = (style == juce::Slider::LinearHorizontal
                            || style == juce::Slider::LinearBar);

    const float trackThickness = 4.0f;
    const float thumbSize = 14.0f;

    if (isHorizontal)
    {
        float trackY = (float)y + (float)height * 0.5f - trackThickness * 0.5f;
        float trackLeft = (float)x + thumbSize * 0.5f;
        float trackRight = (float)x + (float)width - thumbSize * 0.5f;

        // Background track
        g.setColour(GenXColors::track.withAlpha(0.4f * enabledAlpha));
        g.fillRoundedRectangle(trackLeft, trackY, trackRight - trackLeft,
                               trackThickness, trackThickness * 0.5f);

        // Filled portion
        float fillEnd = sliderPos;
        if (fillEnd > trackLeft)
        {
            g.setColour(theme.accent.withAlpha(0.85f * enabledAlpha));
            g.fillRoundedRectangle(trackLeft, trackY, fillEnd - trackLeft,
                                   trackThickness, trackThickness * 0.5f);
        }

        // Thumb
        g.setColour(GenXColors::knobSurface.withAlpha(enabledAlpha));
        g.fillEllipse(fillEnd - thumbSize * 0.5f, trackY + trackThickness * 0.5f - thumbSize * 0.5f,
                      thumbSize, thumbSize);
        g.setColour(GenXColors::knobEdge.withAlpha(enabledAlpha));
        g.drawEllipse(fillEnd - thumbSize * 0.5f, trackY + trackThickness * 0.5f - thumbSize * 0.5f,
                      thumbSize, thumbSize, 1.0f);
        g.setColour(theme.accent.withAlpha(enabledAlpha));
        g.fillEllipse(fillEnd - 3.0f, trackY + trackThickness * 0.5f - 3.0f, 6.0f, 6.0f);
    }
    else
    {
        float trackX = (float)x + (float)width * 0.5f - trackThickness * 0.5f;
        float trackTop = (float)y + thumbSize * 0.5f;
        float trackBottom = (float)y + (float)height - thumbSize * 0.5f;

        // Background track
        g.setColour(GenXColors::track.withAlpha(0.4f * enabledAlpha));
        g.fillRoundedRectangle(trackX, trackTop, trackThickness,
                               trackBottom - trackTop, trackThickness * 0.5f);

        // Filled portion (from bottom up)
        float fillTop = sliderPos;
        if (fillTop < trackBottom)
        {
            g.setColour(theme.accent.withAlpha(0.85f * enabledAlpha));
            g.fillRoundedRectangle(trackX, fillTop, trackThickness,
                                   trackBottom - fillTop, trackThickness * 0.5f);
        }

        // Thumb
        g.setColour(GenXColors::knobSurface.withAlpha(enabledAlpha));
        g.fillEllipse(trackX + trackThickness * 0.5f - thumbSize * 0.5f,
                      fillTop - thumbSize * 0.5f, thumbSize, thumbSize);
        g.setColour(GenXColors::knobEdge.withAlpha(enabledAlpha));
        g.drawEllipse(trackX + trackThickness * 0.5f - thumbSize * 0.5f,
                      fillTop - thumbSize * 0.5f, thumbSize, thumbSize, 1.0f);
        g.setColour(theme.accent.withAlpha(enabledAlpha));
        g.fillEllipse(trackX + trackThickness * 0.5f - 3.0f,
                      fillTop - 3.0f, 6.0f, 6.0f);
    }
}

//------------------------------------------------------------------------------
// Toggle Button
//------------------------------------------------------------------------------
void GenXLookAndFeel::drawToggleButton(juce::Graphics& g,
    juce::ToggleButton& button, bool highlighted, bool /*down*/)
{
    auto bounds = button.getLocalBounds().toFloat().reduced(1.0f);
    const float enabledAlpha = button.isEnabled() ? 1.0f : 0.3f;
    const bool isOn = button.getToggleState();

    if (isOn)
    {
        g.setColour(theme.accent.withAlpha(0.85f * enabledAlpha));
        g.fillRoundedRectangle(bounds, GenXRadius::button);
    }
    else
    {
        g.setColour(GenXColors::bg2.withAlpha(enabledAlpha));
        g.fillRoundedRectangle(bounds, GenXRadius::button);
        g.setColour(GenXColors::border.withAlpha(enabledAlpha));
        g.drawRoundedRectangle(bounds, GenXRadius::button, 1.0f);
    }

    if (highlighted && button.isEnabled())
    {
        g.setColour(juce::Colours::white.withAlpha(0.05f));
        g.fillRoundedRectangle(bounds, GenXRadius::button);
    }

    g.setColour(isOn ? juce::Colours::white.withAlpha(enabledAlpha)
                     : GenXColors::textSecondary.withAlpha(enabledAlpha));
    g.setFont(labelFont.withHeight(GenXType::buttonText));
    g.drawText(button.getButtonText(), bounds, juce::Justification::centred);
}

//------------------------------------------------------------------------------
// Combo Box
//------------------------------------------------------------------------------
void GenXLookAndFeel::drawComboBox(juce::Graphics& g, int width, int height,
    bool /*isButtonDown*/, int, int, int, int, juce::ComboBox&)
{
    auto bounds = juce::Rectangle<float>(0, 0, (float)width, (float)height);
    g.setColour(GenXColors::bg1);
    g.fillRoundedRectangle(bounds, GenXRadius::button);
    g.setColour(GenXColors::border);
    g.drawRoundedRectangle(bounds, GenXRadius::button, 1.0f);

    // Dropdown arrow in accent color
    const float arrowSize = height * 0.3f;
    const float arrowX = width - height * 0.5f;
    const float arrowY = height * 0.5f;
    juce::Path arrow;
    arrow.addTriangle(
        arrowX - arrowSize * 0.5f, arrowY - arrowSize * 0.25f,
        arrowX + arrowSize * 0.5f, arrowY - arrowSize * 0.25f,
        arrowX, arrowY + arrowSize * 0.25f);
    g.setColour(theme.accent);
    g.fillPath(arrow);
}

//------------------------------------------------------------------------------
// Popup Menu
//------------------------------------------------------------------------------
void GenXLookAndFeel::drawPopupMenuBackground(juce::Graphics& g, int width, int height)
{
    g.fillAll(GenXColors::bgOverlay);
    g.setColour(GenXColors::border);
    g.drawRect(0, 0, width, height, 1);
}

void GenXLookAndFeel::drawPopupMenuItem(juce::Graphics& g,
    const juce::Rectangle<int>& area, bool isSeparator, bool isActive,
    bool isHighlighted, bool /*isTicked*/, bool /*hasSubMenu*/,
    const juce::String& text, const juce::String&,
    const juce::Drawable*, const juce::Colour*)
{
    if (isSeparator)
    {
        g.setColour(GenXColors::border);
        g.drawHorizontalLine(area.getCentreY(), (float)area.getX() + 4.0f,
            (float)area.getRight() - 4.0f);
        return;
    }

    if (isHighlighted && isActive)
    {
        g.setColour(theme.accent.withAlpha(0.2f));
        g.fillRect(area);
        g.setColour(GenXColors::textPrimary);
    }
    else
    {
        g.setColour(isActive ? GenXColors::textPrimary : GenXColors::textDisabled);
    }

    g.setFont(labelFont.withHeight(GenXType::controlLabel));
    g.drawText(text, area.reduced(8, 0), juce::Justification::centredLeft);
}

//------------------------------------------------------------------------------
// Label
//------------------------------------------------------------------------------
void GenXLookAndFeel::drawLabel(juce::Graphics& g, juce::Label& label)
{
    g.fillAll(label.findColour(juce::Label::backgroundColourId));

    if (!label.isBeingEdited())
    {
        g.setColour(label.findColour(juce::Label::textColourId)
            .withMultipliedAlpha(label.isEnabled() ? 1.0f : 0.3f));
        g.setFont(label.getFont());
        g.drawText(label.getText(), label.getLocalBounds(),
            label.getJustificationType(), true);
    }
}

//------------------------------------------------------------------------------
// Fonts
//------------------------------------------------------------------------------
juce::Font GenXLookAndFeel::getComboBoxFont(juce::ComboBox&)
{
    return valueFont.withHeight(GenXType::controlValue);
}

juce::Font GenXLookAndFeel::getPopupMenuFont()
{
    return labelFont.withHeight(GenXType::controlLabel);
}

//==============================================================================
//==============================================================================
// GenXTemplateEditor — Plugin Window
//==============================================================================
//==============================================================================

GenXTemplateEditor::GenXTemplateEditor(GenXTemplateProcessor& p)
    : AudioProcessorEditor(&p),
      processorRef(p),
      // ── CHANGE: Set your accent color, display name, and short name ───
      lnf(GenXTheme::create(GenXAccent::utility, "GENX TEMPLATE", "TEMPLATE"))
{
    setLookAndFeel(&lnf);

    // ── Load fonts from binary data ─────────────────────────────────────
    auto bebasTypeface = juce::Typeface::createSystemTypefaceFor(
        BinaryData::BebasNeueRegular_ttf, BinaryData::BebasNeueRegular_ttfSize);
    auto interTypeface = juce::Typeface::createSystemTypefaceFor(
        BinaryData::InterVariable_ttf, BinaryData::InterVariable_ttfSize);
    auto monoTypeface = juce::Typeface::createSystemTypefaceFor(
        BinaryData::JetBrainsMonoRegular_ttf, BinaryData::JetBrainsMonoRegular_ttfSize);

    titleFont  = juce::Font(juce::FontOptions(bebasTypeface)).withHeight(GenXType::titleSize);
    headerFont = juce::Font(juce::FontOptions(bebasTypeface)).withHeight(GenXType::sectionHeader);
    labelFont  = juce::Font(juce::FontOptions(interTypeface)).withHeight(GenXType::controlLabel);
    valueFont  = juce::Font(juce::FontOptions(monoTypeface)).withHeight(GenXType::controlValue);

    // Share fonts with the LookAndFeel
    lnf.titleFont  = titleFont;
    lnf.headerFont = headerFont;
    lnf.labelFont  = labelFont;
    lnf.valueFont  = valueFont;

    // ── Setup controls ──────────────────────────────────────────────────
    setupKnob(gainSlider, gainLabel, "GAIN");
    setupKnob(mixSlider, mixLabel, "MIX");
    setupToggle(bypassButton);

    addAndMakeVisible(gainSlider);
    addAndMakeVisible(gainLabel);
    addAndMakeVisible(mixSlider);
    addAndMakeVisible(mixLabel);
    addAndMakeVisible(bypassButton);

    // ── Bind to APVTS parameters ────────────────────────────────────────
    gainAttachment = std::make_unique<SliderAttachment>(
        processorRef.getAPVTS(), "gain", gainSlider);
    mixAttachment = std::make_unique<SliderAttachment>(
        processorRef.getAPVTS(), "mix", mixSlider);
    bypassAttachment = std::make_unique<ButtonAttachment>(
        processorRef.getAPVTS(), "bypass", bypassButton);

    // ── Window size and constraints ─────────────────────────────────────
    setSize(GenXWindow::defaultWidth, GenXWindow::defaultHeight);
    setResizable(true, true);
    setResizeLimits(GenXWindow::minWidth, GenXWindow::minHeight,
                    GenXWindow::maxWidth, GenXWindow::maxHeight);
    getConstrainer()->setFixedAspectRatio(GenXWindow::aspectRatio);
}

GenXTemplateEditor::~GenXTemplateEditor()
{
    setLookAndFeel(nullptr);
}

//==============================================================================
// Setup Helpers
//==============================================================================
void GenXTemplateEditor::setupKnob(juce::Slider& slider, juce::Label& label,
                                    const juce::String& text)
{
    slider.setSliderStyle(juce::Slider::RotaryHorizontalVerticalDrag);
    slider.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 60, 16);
    slider.setColour(juce::Slider::textBoxTextColourId, GenXColors::textPrimary);
    slider.setColour(juce::Slider::textBoxOutlineColourId, juce::Colours::transparentBlack);

    label.setText(text, juce::dontSendNotification);
    label.setJustificationType(juce::Justification::centred);
    label.setFont(labelFont);
    label.setColour(juce::Label::textColourId, GenXColors::textSecondary);
}

void GenXTemplateEditor::setupToggle(juce::ToggleButton& toggle)
{
    toggle.setClickingTogglesState(true);
}

//==============================================================================
// Paint — draws background, header bar, and section cards
//==============================================================================
void GenXTemplateEditor::paint(juce::Graphics& g)
{
    auto ctx = GenXLayoutContext::compute(getWidth(), getHeight());
    auto area = getLocalBounds();

    // Background
    g.fillAll(GenXColors::bg0);

    // Header bar
    auto headerArea = area.removeFromTop((int)(GenXSpace::headerBarHeight * ctx.scale));
    drawHeaderBar(g, headerArea, ctx.scale);

    // Content area
    area.reduce(ctx.margin, ctx.margin);

    // ── Section cards ───────────────────────────────────────────────────
    // CHANGE: Adjust section layout for your plugin.
    // This template draws two example sections side by side.

    int sectionHeight = (int)(160 * ctx.scale);

    if (ctx.columns >= 2)
    {
        int colWidth = (area.getWidth() - ctx.columnGap) / 2;

        auto mainArea = juce::Rectangle<int>(
            area.getX(), area.getY(), colWidth, sectionHeight);
        drawSectionCard(g, mainArea, "MAIN", ctx.scale);

        auto controlArea = juce::Rectangle<int>(
            area.getX() + colWidth + ctx.columnGap, area.getY(),
            colWidth, sectionHeight);
        drawSectionCard(g, controlArea, "CONTROLS", ctx.scale);
    }
    else
    {
        auto mainArea = area.removeFromTop(sectionHeight);
        drawSectionCard(g, mainArea, "MAIN", ctx.scale);

        area.removeFromTop(ctx.sectionGap);

        auto controlArea = area.removeFromTop(sectionHeight);
        drawSectionCard(g, controlArea, "CONTROLS", ctx.scale);
    }
}

//==============================================================================
// Resized — positions all controls
//==============================================================================
void GenXTemplateEditor::resized()
{
    auto ctx = GenXLayoutContext::compute(getWidth(), getHeight());
    auto area = getLocalBounds();

    // Skip past header bar
    area.removeFromTop((int)(GenXSpace::headerBarHeight * ctx.scale));

    // Content area
    area.reduce(ctx.margin, ctx.margin);

    int sectionHeight = (int)(160 * ctx.scale);
    int headerH = (int)(GenXSpace::headerHeight * ctx.scale);
    int pad = (int)(GenXSpace::cardPadding * ctx.scale);

    if (ctx.columns >= 2)
    {
        int colWidth = (area.getWidth() - ctx.columnGap) / 2;

        // MAIN section — knobs
        auto mainArea = juce::Rectangle<int>(
            area.getX() + pad, area.getY() + headerH + pad,
            colWidth - pad * 2, sectionHeight - headerH - pad * 2);

        int knobW = mainArea.getWidth() / 2;
        placeKnob(gainSlider, gainLabel,
            mainArea.removeFromLeft(knobW), ctx.scale);
        placeKnob(mixSlider, mixLabel,
            mainArea, ctx.scale);

        // CONTROLS section — toggle
        auto controlArea = juce::Rectangle<int>(
            area.getX() + colWidth + ctx.columnGap + pad,
            area.getY() + headerH + pad,
            colWidth - pad * 2, sectionHeight - headerH - pad * 2);

        int toggleW = (int)(80 * ctx.scale);
        int toggleH = (int)(GenXSpace::toggleHeight * ctx.scale);
        bypassButton.setBounds(
            controlArea.getCentreX() - toggleW / 2,
            controlArea.getCentreY() - toggleH / 2,
            toggleW, toggleH);
    }
    else
    {
        // Single column layout
        auto mainArea = area.removeFromTop(sectionHeight)
            .reduced(pad).withTrimmedTop(headerH);

        int knobW = mainArea.getWidth() / 2;
        placeKnob(gainSlider, gainLabel,
            mainArea.removeFromLeft(knobW), ctx.scale);
        placeKnob(mixSlider, mixLabel, mainArea, ctx.scale);

        area.removeFromTop(ctx.sectionGap);

        auto controlArea = area.removeFromTop(sectionHeight)
            .reduced(pad).withTrimmedTop(headerH);

        int toggleW = (int)(80 * ctx.scale);
        int toggleH = (int)(GenXSpace::toggleHeight * ctx.scale);
        bypassButton.setBounds(
            controlArea.getCentreX() - toggleW / 2,
            controlArea.getCentreY() - toggleH / 2,
            toggleW, toggleH);
    }
}

//==============================================================================
// Drawing Helpers
//==============================================================================
void GenXTemplateEditor::drawHeaderBar(juce::Graphics& g,
    juce::Rectangle<int> area, float scale)
{
    auto bounds = area.toFloat();
    const auto& theme = lnf.getTheme();

    // Dark header background
    g.setColour(GenXColors::bg0.darker(0.15f));
    g.fillRect(bounds);

    // Plugin name
    g.setFont(titleFont.withHeight(GenXType::titleSize * scale));
    g.setColour(GenXColors::textPrimary);
    g.drawText(theme.displayName,
        bounds.reduced(GenXSpace::margin * scale, 0),
        juce::Justification::centredLeft);

    // Accent-colored underline beneath the title
    float lineY = bounds.getBottom() - 2.0f;
    float nameWidth = titleFont.withHeight(GenXType::titleSize * scale)
        .getStringWidthFloat(theme.displayName);
    float lineX = GenXSpace::margin * scale;
    g.setColour(theme.accent);
    g.drawLine(lineX, lineY, lineX + nameWidth, lineY, 2.0f);

    // Bottom border
    g.setColour(GenXColors::border);
    g.drawHorizontalLine((int)bounds.getBottom() - 1,
        bounds.getX(), bounds.getRight());
}

void GenXTemplateEditor::drawSectionCard(juce::Graphics& g,
    juce::Rectangle<int> area, const juce::String& title, float scale)
{
    auto bounds = area.toFloat();

    // Card background
    g.setColour(GenXColors::bg1);
    g.fillRoundedRectangle(bounds, GenXRadius::card);

    // Card border
    g.setColour(GenXColors::border);
    g.drawRoundedRectangle(bounds, GenXRadius::card, 1.0f);

    // Section header text (uppercase)
    float headerH = GenXSpace::headerHeight * scale;
    auto headerArea = bounds.removeFromTop(headerH);
    g.setFont(headerFont.withHeight(GenXType::sectionHeader * scale));
    g.setColour(GenXColors::textSecondary);
    g.drawText(title.toUpperCase(),
        headerArea.reduced(GenXSpace::cardPadding * scale, 0),
        juce::Justification::centredLeft);
}

void GenXTemplateEditor::placeKnob(juce::Slider& slider, juce::Label& label,
    juce::Rectangle<int> area, float scale)
{
    int knobDiam = (int)(GenXSpace::knobSize * scale);
    int valH = (int)(GenXSpace::valueHeight * scale);
    int labH = (int)(GenXSpace::labelHeight * scale);
    int totalH = knobDiam + valH + labH;

    // Center vertically in the area
    int startY = area.getCentreY() - totalH / 2;
    int cx = area.getCentreX();

    slider.setBounds(cx - knobDiam / 2, startY, knobDiam, knobDiam + valH);
    label.setBounds(cx - knobDiam / 2, startY + knobDiam + valH, knobDiam, labH);
    label.setFont(labelFont.withHeight(GenXType::controlLabel * scale));
}
