#include "PluginProcessor.h"
#include "PluginEditor.h"

//==============================================================================
GenXDelayProcessor::GenXDelayProcessor()
    : AudioProcessor(BusesProperties()
                         .withInput("Input", juce::AudioChannelSet::stereo(), true)
                         .withOutput("Output", juce::AudioChannelSet::stereo(), true)),
      apvts(*this, nullptr, "Parameters", createParameterLayout())
{
}

GenXDelayProcessor::~GenXDelayProcessor()
{
}

//==============================================================================
juce::AudioProcessorValueTreeState::ParameterLayout GenXDelayProcessor::createParameterLayout()
{
    std::vector<std::unique_ptr<juce::RangedAudioParameter>> params;

    // TIME section
    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"delayTime", 1}, "Delay Time",
        juce::NormalisableRange<float>(1.0f, 2500.0f, 0.1f, 0.4f), // Skewed
        300.0f, juce::AudioParameterFloatAttributes().withLabel("ms")));

    params.push_back(std::make_unique<juce::AudioParameterBool>(
        juce::ParameterID{"reverse", 1}, "Reverse", false));

    params.push_back(std::make_unique<juce::AudioParameterBool>(
        juce::ParameterID{"tempoSync", 1}, "Tempo Sync", false));

    params.push_back(std::make_unique<juce::AudioParameterChoice>(
        juce::ParameterID{"noteDivision", 1}, "Note Division",
        juce::StringArray{"1/1", "1/2", "1/2D", "1/2T", "1/4", "1/4D", "1/4T",
                          "1/8", "1/8D", "1/8T", "1/16", "1/16D", "1/16T"},
        4)); // Default: 1/4

    // MAIN section
    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"feedback", 1}, "Feedback",
        juce::NormalisableRange<float>(0.0f, 0.95f, 0.01f),
        0.4f, juce::AudioParameterFloatAttributes().withLabel("%")));

    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"mix", 1}, "Mix",
        juce::NormalisableRange<float>(0.0f, 1.0f, 0.01f),
        0.3f, juce::AudioParameterFloatAttributes().withLabel("%")));

    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"trim", 1}, "Trim",
        juce::NormalisableRange<float>(-12.0f, 12.0f, 0.1f),
        0.0f, juce::AudioParameterFloatAttributes().withLabel("dB")));

    params.push_back(std::make_unique<juce::AudioParameterChoice>(
        juce::ParameterID{"mode", 1}, "Mode",
        juce::StringArray{"Digital", "Analog"}, 0));

    // STEREO section
    params.push_back(std::make_unique<juce::AudioParameterBool>(
        juce::ParameterID{"pingPong", 1}, "Ping Pong", false));

    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"stereoOffset", 1}, "Stereo Offset",
        juce::NormalisableRange<float>(0.0f, 50.0f, 0.1f),
        10.0f, juce::AudioParameterFloatAttributes().withLabel("ms")));

    // TONE section
    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"highPass", 1}, "High-Pass",
        juce::NormalisableRange<float>(20.0f, 1000.0f, 1.0f, 0.4f),
        80.0f, juce::AudioParameterFloatAttributes().withLabel("Hz")));

    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"lowPass", 1}, "Low-Pass",
        juce::NormalisableRange<float>(500.0f, 20000.0f, 1.0f, 0.4f),
        8000.0f, juce::AudioParameterFloatAttributes().withLabel("Hz")));

    // MODULATION section (Analog mode)
    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"modRate", 1}, "Mod Rate",
        juce::NormalisableRange<float>(0.1f, 5.0f, 0.01f, 0.5f),
        0.8f, juce::AudioParameterFloatAttributes().withLabel("Hz")));

    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"modDepth", 1}, "Mod Depth",
        juce::NormalisableRange<float>(0.0f, 1.0f, 0.01f),
        0.3f, juce::AudioParameterFloatAttributes().withLabel("%")));

    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"drive", 1}, "Drive",
        juce::NormalisableRange<float>(0.0f, 1.0f, 0.01f),
        0.2f, juce::AudioParameterFloatAttributes().withLabel("%")));

    // DUCK section
    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"duckAmount", 1}, "Duck Amount",
        juce::NormalisableRange<float>(0.0f, 1.0f, 0.01f),
        0.0f, juce::AudioParameterFloatAttributes().withLabel("%")));

    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"duckThreshold", 1}, "Duck Threshold",
        juce::NormalisableRange<float>(0.0f, 1.0f, 0.01f),
        0.3f));

    return {params.begin(), params.end()};
}

//==============================================================================
const juce::String GenXDelayProcessor::getName() const
{
    return JucePlugin_Name;
}

bool GenXDelayProcessor::acceptsMidi() const { return false; }
bool GenXDelayProcessor::producesMidi() const { return false; }
bool GenXDelayProcessor::isMidiEffect() const { return false; }
double GenXDelayProcessor::getTailLengthSeconds() const { return MAX_DELAY_SECONDS; }

int GenXDelayProcessor::getNumPrograms() { return 1; }
int GenXDelayProcessor::getCurrentProgram() { return 0; }
void GenXDelayProcessor::setCurrentProgram(int) {}
const juce::String GenXDelayProcessor::getProgramName(int) { return {}; }
void GenXDelayProcessor::changeProgramName(int, const juce::String&) {}

//==============================================================================
void GenXDelayProcessor::prepareToPlay(double sampleRate, int)
{
    currentSampleRate = static_cast<float>(sampleRate);

    // Initialize delay lines
    delayLeft.initialize(currentSampleRate, MAX_DELAY_SECONDS);
    delayRight.initialize(currentSampleRate, MAX_DELAY_SECONDS);
    reverseLeft.initialize(currentSampleRate, MAX_DELAY_SECONDS);
    reverseRight.initialize(currentSampleRate, MAX_DELAY_SECONDS);

    // Initialize modulator
    modulator.initialize(currentSampleRate);

    // Initialize ducker
    ducker.initialize(currentSampleRate);

    // Initialize smoothers
    delaySmootherLeft.setCutoff(currentSampleRate, 10.0f);
    delaySmootherRight.setCutoff(currentSampleRate, 10.0f);

    // Initialize filters
    float lpFreq = apvts.getRawParameterValue("lowPass")->load();
    float hpFreq = apvts.getRawParameterValue("highPass")->load();
    filterLeft.update(currentSampleRate, lpFreq, hpFreq);
    filterRight.update(currentSampleRate, lpFreq, hpFreq);

    // Reset state
    feedbackLeft = 0.0f;
    feedbackRight = 0.0f;
    safetyEnvelope = 0.0f;
    safetyGain = 1.0f;
}

void GenXDelayProcessor::releaseResources()
{
    delayLeft.reset();
    delayRight.reset();
    reverseLeft.reset();
    reverseRight.reset();
    filterLeft.reset();
    filterRight.reset();
    modulator.reset();
    ducker.reset();
    delaySmootherLeft.reset();
    delaySmootherRight.reset();
}

bool GenXDelayProcessor::isBusesLayoutSupported(const BusesLayout& layouts) const
{
    if (layouts.getMainOutputChannelSet() != juce::AudioChannelSet::stereo())
        return false;

    if (layouts.getMainInputChannelSet() != juce::AudioChannelSet::stereo() &&
        layouts.getMainInputChannelSet() != juce::AudioChannelSet::mono())
        return false;

    return true;
}

float GenXDelayProcessor::getNoteDivisionMultiplier(NoteDivision div)
{
    switch (div)
    {
        case NoteDivision::Whole:            return 4.0f;
        case NoteDivision::Half:             return 2.0f;
        case NoteDivision::HalfDotted:       return 3.0f;
        case NoteDivision::HalfTriplet:      return 4.0f / 3.0f;
        case NoteDivision::Quarter:          return 1.0f;
        case NoteDivision::QuarterDotted:    return 1.5f;
        case NoteDivision::QuarterTriplet:   return 2.0f / 3.0f;
        case NoteDivision::Eighth:           return 0.5f;
        case NoteDivision::EighthDotted:     return 0.75f;
        case NoteDivision::EighthTriplet:    return 1.0f / 3.0f;
        case NoteDivision::Sixteenth:        return 0.25f;
        case NoteDivision::SixteenthDotted:  return 0.375f;
        case NoteDivision::SixteenthTriplet: return 1.0f / 6.0f;
        default: return 1.0f;
    }
}

float GenXDelayProcessor::updateSafetyLimiter(float stereoPeak)
{
    constexpr float attackTime = 0.001f;
    constexpr float releaseTime = 0.100f;

    float attack = std::exp(-1.0f / (currentSampleRate * attackTime));
    float release = std::exp(-1.0f / (currentSampleRate * releaseTime));

    // Peak follower
    if (stereoPeak > safetyEnvelope)
        safetyEnvelope = attack * safetyEnvelope + (1.0f - attack) * stereoPeak;
    else
        safetyEnvelope = release * safetyEnvelope + (1.0f - release) * stereoPeak;

    // Calculate target gain
    float targetGain = (safetyEnvelope > SAFETY_THRESHOLD)
                           ? SAFETY_THRESHOLD / std::max(safetyEnvelope, 1e-6f)
                           : 1.0f;

    // Smooth gain changes
    float gainAttack = std::exp(-1.0f / (currentSampleRate * 0.001f));
    float gainRelease = std::exp(-1.0f / (currentSampleRate * 0.120f));

    if (targetGain < safetyGain)
        safetyGain = gainAttack * safetyGain + (1.0f - gainAttack) * targetGain;
    else
        safetyGain = gainRelease * safetyGain + (1.0f - gainRelease) * targetGain;

    return safetyGain;
}

void GenXDelayProcessor::processBlock(juce::AudioBuffer<float>& buffer, juce::MidiBuffer&)
{
    juce::ScopedNoDenormals noDenormals;

    auto numInputChannels = getTotalNumInputChannels();
    auto numOutputChannels = getTotalNumOutputChannels();
    auto numSamples = buffer.getNumSamples();

    // Get parameter values
    float delayTimeMs = apvts.getRawParameterValue("delayTime")->load();
    bool reverse = apvts.getRawParameterValue("reverse")->load() > 0.5f;
    bool tempoSync = apvts.getRawParameterValue("tempoSync")->load() > 0.5f;
    int noteDivIdx = static_cast<int>(apvts.getRawParameterValue("noteDivision")->load());
    float feedback = apvts.getRawParameterValue("feedback")->load();
    float mix = apvts.getRawParameterValue("mix")->load();
    float trimDb = apvts.getRawParameterValue("trim")->load();
    int modeIdx = static_cast<int>(apvts.getRawParameterValue("mode")->load());
    bool pingPong = apvts.getRawParameterValue("pingPong")->load() > 0.5f;
    float stereoOffsetMs = apvts.getRawParameterValue("stereoOffset")->load();
    float hpFreq = apvts.getRawParameterValue("highPass")->load();
    float lpFreq = apvts.getRawParameterValue("lowPass")->load();
    float modRate = apvts.getRawParameterValue("modRate")->load();
    float modDepth = apvts.getRawParameterValue("modDepth")->load();
    float drive = apvts.getRawParameterValue("drive")->load();
    float duckAmount = apvts.getRawParameterValue("duckAmount")->load();
    float duckThreshold = apvts.getRawParameterValue("duckThreshold")->load();

    bool isAnalog = (modeIdx == 1);
    float trimGain = juce::Decibels::decibelsToGain(trimDb);

    // Handle tempo sync
    if (tempoSync)
    {
        if (auto* playHead = getPlayHead())
        {
            if (auto posInfo = playHead->getPosition())
            {
                if (auto bpm = posInfo->getBpm())
                {
                    float msPerBeat = 60000.0f / static_cast<float>(*bpm);
                    auto div = static_cast<NoteDivision>(noteDivIdx);
                    delayTimeMs = msPerBeat * getNoteDivisionMultiplier(div);
                }
            }
        }
    }

    // Convert to samples
    float baseDelaySamples = delayTimeMs * currentSampleRate / 1000.0f;
    float offsetSamples = stereoOffsetMs * currentSampleRate / 1000.0f;

    // Update filters
    filterLeft.update(currentSampleRate, lpFreq, hpFreq);
    filterRight.update(currentSampleRate, lpFreq, hpFreq);

    // Update saturators
    float effectiveDrive = isAnalog ? drive : 0.0f;
    saturatorLeft.setDrive(effectiveDrive);
    saturatorRight.setDrive(effectiveDrive);

    // Process samples
    auto* leftChannel = buffer.getWritePointer(0);
    auto* rightChannel = numInputChannels >= 2 ? buffer.getWritePointer(1) : nullptr;

    for (int i = 0; i < numSamples; ++i)
    {
        float inputL = leftChannel[i];
        float inputR = rightChannel ? rightChannel[i] : inputL;

        // Calculate delay times (ping-pong uses equal times)
        float delayL = baseDelaySamples;
        float delayR = pingPong ? baseDelaySamples : (baseDelaySamples + offsetSamples);

        // Apply modulation in analog mode
        if (isAnalog && modDepth > 0.001f)
        {
            float maxModSamples = modDepth * 20.0f;
            auto [modDelayL, modDelayR] = modulator.getModulatedDelays(delayL, delayR, maxModSamples, modRate);
            delayL = modDelayL;
            delayR = modDelayR;
        }

        // Smooth delay times
        float smoothDelayL = delaySmootherLeft.process(delayL);
        float smoothDelayR = delaySmootherRight.process(delayR);

        // Calculate ducking
        float duckGain = (duckAmount > 0.001f)
                             ? ducker.processStereo(inputL, inputR, duckThreshold, duckAmount)
                             : 1.0f;

        // Read from delay lines
        float delayedL, delayedR;
        if (reverse)
        {
            delayedL = reverseLeft.read(smoothDelayL);
            delayedR = reverseRight.read(smoothDelayR);
        }
        else
        {
            delayedL = delayLeft.read(smoothDelayL);
            delayedR = delayRight.read(smoothDelayR);
        }

        // Process through feedback chain
        float filteredL = filterLeft.process(delayedL);
        float filteredR = filterRight.process(delayedR);
        float saturatedL = saturatorLeft.process(filteredL);
        float saturatedR = saturatorRight.process(filteredR);

        // Calculate delay inputs
        float delayInputL, delayInputR;
        if (pingPong)
        {
            float monoIn = 0.5f * (inputL + inputR);
            delayInputL = monoIn + saturatedR * feedback;
            delayInputR = saturatedL * feedback;
        }
        else
        {
            delayInputL = inputL + saturatedL * feedback;
            delayInputR = inputR + saturatedR * feedback;
        }

        // Clamp to prevent runaway
        delayInputL = std::clamp(delayInputL, -1.25f, 1.25f);
        delayInputR = std::clamp(delayInputR, -1.25f, 1.25f);

        // Write to delay lines
        if (reverse)
        {
            reverseLeft.write(delayInputL);
            reverseRight.write(delayInputR);
        }
        else
        {
            delayLeft.write(delayInputL);
            delayRight.write(delayInputR);
        }

        // Store feedback
        feedbackLeft = saturatedL;
        feedbackRight = saturatedR;

        // Mix with ducking
        float wetL = saturatedL * duckGain;
        float wetR = saturatedR * duckGain;

        float outputL = inputL * (1.0f - mix) + wetL * mix;
        float outputR = inputR * (1.0f - mix) + wetR * mix;

        // Apply trim
        outputL *= trimGain;
        outputR *= trimGain;

        // Safety limiter
        float stereoPeak = std::max(std::abs(outputL), std::abs(outputR));
        float limiterGain = updateSafetyLimiter(stereoPeak);
        outputL = std::clamp(outputL * limiterGain, -1.0f, 1.0f);
        outputR = std::clamp(outputR * limiterGain, -1.0f, 1.0f);

        // Write output
        leftChannel[i] = outputL;
        if (rightChannel)
            rightChannel[i] = outputR;
    }

    // Compute per-block peak levels for GUI metering
    peakLevelLeft.store(buffer.getMagnitude(0, 0, numSamples), std::memory_order_relaxed);
    peakLevelRight.store(
        numOutputChannels >= 2 ? buffer.getMagnitude(1, 0, numSamples)
                               : buffer.getMagnitude(0, 0, numSamples),
        std::memory_order_relaxed);

    // Handle mono-to-stereo
    if (numInputChannels == 1 && numOutputChannels == 2)
    {
        buffer.copyFrom(1, 0, buffer, 0, 0, numSamples);
    }
}

//==============================================================================
bool GenXDelayProcessor::hasEditor() const { return true; }

juce::AudioProcessorEditor* GenXDelayProcessor::createEditor()
{
    return new GenXDelayEditor(*this);
}

//==============================================================================
void GenXDelayProcessor::getStateInformation(juce::MemoryBlock& destData)
{
    auto state = apvts.copyState();
    std::unique_ptr<juce::XmlElement> xml(state.createXml());
    copyXmlToBinary(*xml, destData);
}

void GenXDelayProcessor::setStateInformation(const void* data, int sizeInBytes)
{
    std::unique_ptr<juce::XmlElement> xml(getXmlFromBinary(data, sizeInBytes));
    if (xml && xml->hasTagName(apvts.state.getType()))
        apvts.replaceState(juce::ValueTree::fromXml(*xml));
}

//==============================================================================
juce::AudioProcessor* JUCE_CALLTYPE createPluginFilter()
{
    return new GenXDelayProcessor();
}
