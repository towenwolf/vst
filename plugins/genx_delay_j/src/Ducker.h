#pragma once

#include <cmath>
#include <algorithm>

// Dynamic ducker - reduces wet signal when dry input is loud
class Ducker
{
public:
    Ducker() = default;

    void initialize(float sr)
    {
        sampleRate = sr;
        // Default: 5ms attack, 200ms release
        setTimes(0.005f, 0.200f);
    }

    void setTimes(float attackSec, float releaseSec)
    {
        attackCoeff = std::exp(-1.0f / (sampleRate * attackSec));
        releaseCoeff = std::exp(-1.0f / (sampleRate * releaseSec));
    }

    void reset()
    {
        envelope = 0.0f;
    }

    // Returns gain reduction factor (0-1) to apply to wet signal
    float processStereo(float inputL, float inputR, float threshold, float amount)
    {
        // Peak detection
        float peak = std::max(std::abs(inputL), std::abs(inputR));

        // Envelope follower
        if (peak > envelope)
            envelope = attackCoeff * envelope + (1.0f - attackCoeff) * peak;
        else
            envelope = releaseCoeff * envelope + (1.0f - releaseCoeff) * peak;

        // Calculate gain reduction
        if (envelope > threshold)
        {
            float excess = envelope - threshold;
            float reduction = std::min(excess * amount * 2.0f, 1.0f);
            return 1.0f - reduction;
        }

        return 1.0f;
    }

private:
    float sampleRate = 44100.0f;
    float attackCoeff = 0.0f;
    float releaseCoeff = 0.0f;
    float envelope = 0.0f;
};
