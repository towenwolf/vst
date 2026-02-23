#include "PluginProcessor.h"
#include "PluginEditor.h"

//==============================================================================
GenXTemplateProcessor::GenXTemplateProcessor()
    : AudioProcessor(BusesProperties()
          .withInput("Input", juce::AudioChannelSet::stereo(), true)
          .withOutput("Output", juce::AudioChannelSet::stereo(), true)),
      apvts(*this, nullptr, "PARAMETERS", createParameterLayout())
{
}

GenXTemplateProcessor::~GenXTemplateProcessor() {}

//==============================================================================
juce::AudioProcessorValueTreeState::ParameterLayout
GenXTemplateProcessor::createParameterLayout()
{
    std::vector<std::unique_ptr<juce::RangedAudioParameter>> params;

    // ── Example parameters — replace with your own ──────────────────────
    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID("gain", 1), "Gain",
        juce::NormalisableRange<float>(-24.0f, 24.0f, 0.1f), 0.0f,
        juce::AudioParameterFloatAttributes().withLabel("dB")));

    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID("mix", 1), "Mix",
        juce::NormalisableRange<float>(0.0f, 100.0f, 0.1f), 100.0f,
        juce::AudioParameterFloatAttributes().withLabel("%")));

    params.push_back(std::make_unique<juce::AudioParameterBool>(
        juce::ParameterID("bypass", 1), "Bypass", false));

    return { params.begin(), params.end() };
}

//==============================================================================
void GenXTemplateProcessor::prepareToPlay(double /*sampleRate*/, int /*samplesPerBlock*/)
{
    // TODO: Initialize your DSP here
}

void GenXTemplateProcessor::releaseResources() {}

bool GenXTemplateProcessor::isBusesLayoutSupported(const BusesLayout& layouts) const
{
    if (layouts.getMainOutputChannelSet() != juce::AudioChannelSet::mono()
        && layouts.getMainOutputChannelSet() != juce::AudioChannelSet::stereo())
        return false;

    if (layouts.getMainOutputChannelSet() != layouts.getMainInputChannelSet())
        return false;

    return true;
}

void GenXTemplateProcessor::processBlock(juce::AudioBuffer<float>& buffer,
                                          juce::MidiBuffer& /*midiMessages*/)
{
    juce::ScopedNoDenormals noDenormals;

    auto totalNumInputChannels = getTotalNumInputChannels();
    auto totalNumOutputChannels = getTotalNumOutputChannels();

    for (auto i = totalNumInputChannels; i < totalNumOutputChannels; ++i)
        buffer.clear(i, 0, buffer.getNumSamples());

    // Read parameters
    float gainDB = apvts.getRawParameterValue("gain")->load();
    float mix    = apvts.getRawParameterValue("mix")->load() / 100.0f;
    bool bypass  = apvts.getRawParameterValue("bypass")->load() > 0.5f;

    if (bypass)
        return;

    float gainLinear = juce::Decibels::decibelsToGain(gainDB);

    // TODO: Replace with your DSP processing
    for (int channel = 0; channel < totalNumInputChannels; ++channel)
    {
        auto* channelData = buffer.getWritePointer(channel);

        for (int sample = 0; sample < buffer.getNumSamples(); ++sample)
        {
            float dry = channelData[sample];
            float wet = dry * gainLinear;
            channelData[sample] = dry * (1.0f - mix) + wet * mix;
        }
    }
}

//==============================================================================
bool GenXTemplateProcessor::hasEditor() const { return true; }

juce::AudioProcessorEditor* GenXTemplateProcessor::createEditor()
{
    return new GenXTemplateEditor(*this);
}

//==============================================================================
const juce::String GenXTemplateProcessor::getName() const { return JucePlugin_Name; }
bool GenXTemplateProcessor::acceptsMidi() const { return false; }
bool GenXTemplateProcessor::producesMidi() const { return false; }
bool GenXTemplateProcessor::isMidiEffect() const { return false; }
double GenXTemplateProcessor::getTailLengthSeconds() const { return 0.0; }

int GenXTemplateProcessor::getNumPrograms() { return 1; }
int GenXTemplateProcessor::getCurrentProgram() { return 0; }
void GenXTemplateProcessor::setCurrentProgram(int) {}
const juce::String GenXTemplateProcessor::getProgramName(int) { return {}; }
void GenXTemplateProcessor::changeProgramName(int, const juce::String&) {}

//==============================================================================
void GenXTemplateProcessor::getStateInformation(juce::MemoryBlock& destData)
{
    auto state = apvts.copyState();
    std::unique_ptr<juce::XmlElement> xml(state.createXml());
    copyXmlToBinary(*xml, destData);
}

void GenXTemplateProcessor::setStateInformation(const void* data, int sizeInBytes)
{
    std::unique_ptr<juce::XmlElement> xml(getXmlFromBinary(data, sizeInBytes));
    if (xml != nullptr && xml->hasTagName(apvts.state.getType()))
        apvts.replaceState(juce::ValueTree::fromXml(*xml));
}

//==============================================================================
juce::AudioProcessor* JUCE_CALLTYPE createPluginFilter()
{
    return new GenXTemplateProcessor();
}
