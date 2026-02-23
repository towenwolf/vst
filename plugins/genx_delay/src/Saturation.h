#pragma once

#include <cmath>
#include <algorithm>

// Soft saturation using tanh approximation
class Saturator
{
public:
    Saturator() = default;

    void setDrive(float drive)
    {
        // Drive maps 0-1 to 1-5x input gain
        this->driveGain = 1.0f + drive * 4.0f;
        // Output compensation to maintain perceived volume
        this->outputGain = 1.0f / (0.5f + 0.5f * driveGain);
    }

    float process(float input) const
    {
        if (driveGain <= 1.001f)
            return input;

        float driven = input * driveGain;
        // Padé approximant of tanh for efficiency
        float saturated = fastTanh(driven);
        return saturated * outputGain;
    }

private:
    float driveGain = 1.0f;
    float outputGain = 1.0f;

    static float fastTanh(float x)
    {
        // Clamp to avoid overflow
        x = std::clamp(x, -4.0f, 4.0f);
        float x2 = x * x;
        // Padé approximant: tanh(x) ≈ x(27 + x²) / (27 + 9x²)
        return x * (27.0f + x2) / (27.0f + 9.0f * x2);
    }
};
