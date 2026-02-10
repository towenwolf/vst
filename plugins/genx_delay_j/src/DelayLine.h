#pragma once

#include <vector>
#include <cmath>

// Simple delay line with linear interpolation
class DelayLine
{
public:
    DelayLine() = default;

    void initialize(float sr, float maxDelaySeconds)
    {
        sampleRate = sr;
        size_t bufferSize = static_cast<size_t>(std::ceil(sr * maxDelaySeconds)) + 1;
        buffer.resize(bufferSize, 0.0f);
        writePos = 0;
    }

    void reset()
    {
        std::fill(buffer.begin(), buffer.end(), 0.0f);
        writePos = 0;
    }

    void write(float sample)
    {
        if (buffer.empty()) return;
        buffer[writePos] = sample;
        writePos = (writePos + 1) % buffer.size();
    }

    float read(float delaySamples) const
    {
        if (buffer.empty()) return 0.0f;

        float readPos = static_cast<float>(writePos) - delaySamples - 1.0f;
        while (readPos < 0.0f)
            readPos += static_cast<float>(buffer.size());

        size_t idx0 = static_cast<size_t>(readPos) % buffer.size();
        size_t idx1 = (idx0 + 1) % buffer.size();
        float frac = readPos - std::floor(readPos);

        return buffer[idx0] * (1.0f - frac) + buffer[idx1] * frac;
    }

private:
    std::vector<float> buffer;
    size_t writePos = 0;
    float sampleRate = 44100.0f;
};

// Reverse delay line with two overlapping Hann-windowed grains.
// Captures chunks of audio (chunk length = delay time) and plays them backwards.
// Two grains staggered by half a chunk ensure click-free output.
class ReverseDelayLine
{
public:
    ReverseDelayLine() = default;

    void initialize(float sr, float maxDelaySeconds)
    {
        sampleRate = sr;
        maxSamples = static_cast<size_t>(std::ceil(sr * maxDelaySeconds)) + 1;
        buffer.resize(maxSamples, 0.0f);
        writePos = 0;

        size_t defaultChunk = static_cast<size_t>(sr * 0.3f); // 300ms default
        grains[0] = { 0, 0, defaultChunk };
        grains[1] = { 0, defaultChunk / 2, defaultChunk };
    }

    void reset()
    {
        std::fill(buffer.begin(), buffer.end(), 0.0f);
        writePos = 0;

        size_t chunkSize = grains[0].chunkSize;
        grains[0] = { 0, 0, chunkSize };
        grains[1] = { 0, chunkSize / 2, chunkSize };
    }

    void write(float sample)
    {
        if (buffer.empty()) return;
        buffer[writePos] = sample;
        writePos = (writePos + 1) % maxSamples;
    }

    float read(float delaySamples)
    {
        if (buffer.empty()) return 0.0f;

        size_t chunkSize = std::max(static_cast<size_t>(delaySamples), size_t(2));
        float output = 0.0f;

        for (auto& grain : grains)
        {
            // Read from buffer in reverse: start position minus counter
            size_t readPos = (grain.start >= grain.counter)
                ? grain.start - grain.counter
                : grain.start + maxSamples - grain.counter;
            float sample = buffer[readPos % maxSamples];

            // Full Hann window: 0.5 * (1 - cos(2*PI*t)) where t = counter / chunkSize
            float t = static_cast<float>(grain.counter) / static_cast<float>(grain.chunkSize);
            float window = 0.5f * (1.0f - std::cos(2.0f * static_cast<float>(M_PI) * t));

            output += sample * window;

            // Advance grain
            grain.counter += 1;
            if (grain.counter >= grain.chunkSize)
            {
                // Reset grain: start reading from the most recent sample
                grain.counter = 0;
                grain.chunkSize = chunkSize;
                grain.start = (writePos == 0) ? maxSamples - 1 : writePos - 1;
            }
        }

        return output;
    }

private:
    struct Grain
    {
        size_t start = 0;
        size_t counter = 0;
        size_t chunkSize = 1;
    };

    std::vector<float> buffer;
    size_t writePos = 0;
    size_t maxSamples = 0;
    float sampleRate = 44100.0f;
    Grain grains[2];
};
