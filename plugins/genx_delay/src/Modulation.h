#pragma once

#include <cmath>
#include <utility>

// Stereo LFO modulator for chorus/vibrato effect
class StereoModulator
{
public:
    StereoModulator() = default;

    void initialize(float sr)
    {
        sampleRate = sr;
        phaseLeft = 0.0f;
        phaseRight = 0.5f; // 180° offset for stereo width
    }

    void reset()
    {
        phaseLeft = 0.0f;
        phaseRight = 0.5f;
    }

    // Returns modulated delay times for L/R channels
    std::pair<float, float> getModulatedDelays(
        float baseDelayL,
        float baseDelayR,
        float maxModSamples,
        float rate)
    {
        // Sine LFO
        float modL = std::sin(phaseLeft * 2.0f * static_cast<float>(M_PI));
        float modR = std::sin(phaseRight * 2.0f * static_cast<float>(M_PI));

        // Advance phase
        float phaseInc = rate / sampleRate;
        phaseLeft += phaseInc;
        phaseRight += phaseInc;

        if (phaseLeft >= 1.0f) phaseLeft -= 1.0f;
        if (phaseRight >= 1.0f) phaseRight -= 1.0f;

        return {
            baseDelayL + modL * maxModSamples,
            baseDelayR + modR * maxModSamples
        };
    }

private:
    float sampleRate = 44100.0f;
    float phaseLeft = 0.0f;
    float phaseRight = 0.5f;
};
