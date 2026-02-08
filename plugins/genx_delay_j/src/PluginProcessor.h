#pragma once

#include <juce_audio_processors/juce_audio_processors.h>
#include "DelayLine.h"
#include "Modulation.h"
#include "Filters.h"
#include "Saturation.h"
#include "Ducker.h"

//==============================================================================
// GenX Delay - JUCE Implementation
// An emulation of delays popular in 00s alternative/rock music
//==============================================================================
class GenXDelayProcessor : public juce::AudioProcessor
{
public:
    //==============================================================================
    GenXDelayProcessor();
    ~GenXDelayProcessor() override;

    //==============================================================================
    void prepareToPlay(double sampleRate, int samplesPerBlock) override;
    void releaseResources() override;

    bool isBusesLayoutSupported(const BusesLayout& layouts) const override;

    void processBlock(juce::AudioBuffer<float>&, juce::MidiBuffer&) override;

    //==============================================================================
    juce::AudioProcessorEditor* createEditor() override;
    bool hasEditor() const override;

    //==============================================================================
    const juce::String getName() const override;

    bool acceptsMidi() const override;
    bool producesMidi() const override;
    bool isMidiEffect() const override;
    double getTailLengthSeconds() const override;

    //==============================================================================
    int getNumPrograms() override;
    int getCurrentProgram() override;
    void setCurrentProgram(int index) override;
    const juce::String getProgramName(int index) override;
    void changeProgramName(int index, const juce::String& newName) override;

    //==============================================================================
    void getStateInformation(juce::MemoryBlock& destData) override;
    void setStateInformation(const void* data, int sizeInBytes) override;

    //==============================================================================
    // Parameter access
    juce::AudioProcessorValueTreeState& getAPVTS() { return apvts; }

    // Delay mode enum
    enum class DelayMode { Digital, Analog };

    // Note division enum
    enum class NoteDivision
    {
        Whole, Half, HalfDotted, HalfTriplet,
        Quarter, QuarterDotted, QuarterTriplet,
        Eighth, EighthDotted, EighthTriplet,
        Sixteenth, SixteenthDotted, SixteenthTriplet
    };

    static float getNoteDivisionMultiplier(NoteDivision div);

private:
    //==============================================================================
    juce::AudioProcessorValueTreeState apvts;
    juce::AudioProcessorValueTreeState::ParameterLayout createParameterLayout();

    // DSP components
    DelayLine delayLeft, delayRight;
    ReverseDelayLine reverseLeft, reverseRight;
    FeedbackFilter filterLeft, filterRight;
    StereoModulator modulator;
    Saturator saturatorLeft, saturatorRight;
    Ducker ducker;
    OnePoleLP delaySmootherLeft, delaySmootherRight;

    // State
    float currentSampleRate = 44100.0f;
    float feedbackLeft = 0.0f;
    float feedbackRight = 0.0f;

    // Safety limiter state
    float safetyEnvelope = 0.0f;
    float safetyGain = 1.0f;

    // Constants
    static constexpr float MAX_DELAY_SECONDS = 2.5f;
    static constexpr float SAFETY_THRESHOLD = 0.95f;

    float updateSafetyLimiter(float stereoPeak);

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(GenXDelayProcessor)
};
