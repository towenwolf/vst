#pragma once

#include <cmath>

// Simple one-pole lowpass filter for smoothing
class OnePoleLP
{
public:
    OnePoleLP() = default;

    void setCutoff(float sampleRate, float cutoffHz)
    {
        float omega = 2.0f * static_cast<float>(M_PI) * cutoffHz / sampleRate;
        coeff = 1.0f - std::exp(-omega);
    }

    void reset()
    {
        state = 0.0f;
    }

    float process(float input)
    {
        state += coeff * (input - state);
        return state;
    }

private:
    float coeff = 0.1f;
    float state = 0.0f;
};

// Biquad filter for feedback tone shaping
class BiquadFilter
{
public:
    enum Type { LowPass, HighPass };

    BiquadFilter() = default;

    void setCoefficients(float sampleRate, float frequency, float q, Type type)
    {
        float omega = 2.0f * static_cast<float>(M_PI) * frequency / sampleRate;
        float sinOmega = std::sin(omega);
        float cosOmega = std::cos(omega);
        float alpha = sinOmega / (2.0f * q);

        float a0;

        if (type == LowPass)
        {
            b0 = (1.0f - cosOmega) / 2.0f;
            b1 = 1.0f - cosOmega;
            b2 = (1.0f - cosOmega) / 2.0f;
            a0 = 1.0f + alpha;
            a1 = -2.0f * cosOmega;
            a2 = 1.0f - alpha;
        }
        else // HighPass
        {
            b0 = (1.0f + cosOmega) / 2.0f;
            b1 = -(1.0f + cosOmega);
            b2 = (1.0f + cosOmega) / 2.0f;
            a0 = 1.0f + alpha;
            a1 = -2.0f * cosOmega;
            a2 = 1.0f - alpha;
        }

        // Normalize
        b0 /= a0;
        b1 /= a0;
        b2 /= a0;
        a1 /= a0;
        a2 /= a0;
    }

    void reset()
    {
        x1 = x2 = y1 = y2 = 0.0f;
    }

    float process(float input)
    {
        float output = b0 * input + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;

        x2 = x1;
        x1 = input;
        y2 = y1;
        y1 = output;

        return output;
    }

private:
    float b0 = 1.0f, b1 = 0.0f, b2 = 0.0f;
    float a1 = 0.0f, a2 = 0.0f;
    float x1 = 0.0f, x2 = 0.0f;
    float y1 = 0.0f, y2 = 0.0f;
};

// Combined HP + LP feedback filter chain
class FeedbackFilter
{
public:
    FeedbackFilter() = default;

    void update(float sampleRate, float lowpassFreq, float highpassFreq)
    {
        constexpr float Q = 0.707f; // Butterworth
        lowpass.setCoefficients(sampleRate, lowpassFreq, Q, BiquadFilter::LowPass);
        highpass.setCoefficients(sampleRate, highpassFreq, Q, BiquadFilter::HighPass);
    }

    void reset()
    {
        highpass.reset();
        lowpass.reset();
    }

    float process(float input)
    {
        return lowpass.process(highpass.process(input));
    }

private:
    BiquadFilter highpass;
    BiquadFilter lowpass;
};
