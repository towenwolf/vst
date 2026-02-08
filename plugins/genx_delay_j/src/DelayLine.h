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

// Reverse delay line with Hann-windowed grain crossfade
class ReverseDelayLine
{
public:
    ReverseDelayLine() = default;

    void initialize(float sr, float maxDelaySeconds)
    {
        sampleRate = sr;
        size_t bufferSize = static_cast<size_t>(std::ceil(sr * maxDelaySeconds)) + 1;
        buffer.resize(bufferSize, 0.0f);
        writePos = 0;
        grainPhase = 0.0f;
    }

    void reset()
    {
        std::fill(buffer.begin(), buffer.end(), 0.0f);
        writePos = 0;
        grainPhase = 0.0f;
    }

    void write(float sample)
    {
        if (buffer.empty()) return;
        buffer[writePos] = sample;
        writePos = (writePos + 1) % buffer.size();
    }

    float read(float delaySamples)
    {
        if (buffer.empty() || delaySamples < 1.0f) return 0.0f;

        size_t chunkSize = static_cast<size_t>(delaySamples);
        if (chunkSize < 2) chunkSize = 2;

        // Two overlapping grains with Hann window crossfade
        float grain1 = readGrain(0, chunkSize);
        float grain2 = readGrain(chunkSize / 2, chunkSize);

        // Hann window for crossfade
        float t = grainPhase;
        float w1 = 0.5f * (1.0f - std::cos(static_cast<float>(M_PI) * t));
        float w2 = 0.5f * (1.0f - std::cos(static_cast<float>(M_PI) * (t + 1.0f)));

        // Advance grain phase
        grainPhase += 1.0f / static_cast<float>(chunkSize);
        if (grainPhase >= 1.0f) grainPhase -= 1.0f;

        return grain1 * w1 + grain2 * w2;
    }

private:
    float readGrain(size_t offset, size_t chunkSize) const
    {
        // Read backwards from the write position
        float phase = grainPhase + static_cast<float>(offset) / static_cast<float>(chunkSize);
        if (phase >= 1.0f) phase -= 1.0f;

        size_t sampleOffset = static_cast<size_t>(phase * static_cast<float>(chunkSize));
        size_t readIdx = (writePos + buffer.size() - chunkSize + sampleOffset) % buffer.size();

        // Reverse: read from end towards start of chunk
        readIdx = (writePos + buffer.size() - 1 - sampleOffset) % buffer.size();

        return buffer[readIdx];
    }

    std::vector<float> buffer;
    size_t writePos = 0;
    float sampleRate = 44100.0f;
    float grainPhase = 0.0f;
};
