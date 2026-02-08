# GenX Delay (JUCE)

A full C++/JUCE implementation of the GenX Delay plugin - an emulation of delays popular in 00s alternative/rock music.

## Features

- **Delay Time**: 1-2500ms with tempo sync option
- **Tempo Sync**: Lock to host BPM with note divisions (1/1 to 1/16 including dotted and triplet)
- **Reverse Mode**: Grain-based reverse delay with crossfade
- **Ping Pong**: Stereo ping-pong delay mode
- **Feedback Filters**: High-pass and low-pass in the feedback loop
- **Analog Mode**: Modulation (chorus/vibrato) and saturation
- **Ducking**: Reduce wet signal when dry input is loud
- **Safety Limiter**: Prevents runaway feedback

## Building

### Requirements

- CMake 3.22+
- C++ compiler with C++17 support
- Xcode (macOS) / Visual Studio (Windows) / GCC (Linux)

### macOS

```bash
# Install CMake if needed
brew install cmake

# Build
./build.sh

# Or manually:
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . --config Release
```

### Windows

```bash
mkdir build && cd build
cmake .. -G "Visual Studio 17 2022"
cmake --build . --config Release
```

### Linux

```bash
# Install dependencies
sudo apt install cmake build-essential libasound2-dev \
    libfreetype6-dev libx11-dev libxrandr-dev \
    libxinerama-dev libxcursor-dev libgl1-mesa-dev

mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build .
```

## Output Formats

The build produces:
- **VST3**: `GenXDelayJ.vst3`
- **AU**: `GenXDelayJ.component` (macOS only)
- **Standalone**: `GenX Delay.app` / `GenXDelayJ.exe`

Plugins are automatically copied to system plugin folders on macOS.

## Plugin Locations

After building, find the plugins at:

**macOS**:
- `~/Library/Audio/Plug-Ins/VST3/GenXDelayJ.vst3`
- `~/Library/Audio/Plug-Ins/Components/GenXDelayJ.component`

**Windows**:
- `C:\Program Files\Common Files\VST3\GenXDelayJ.vst3`

**Linux**:
- `~/.vst3/GenXDelayJ.vst3`

## Parameters

| Section | Parameter | Range | Default |
|---------|-----------|-------|---------|
| TIME | Delay Time | 1-2500 ms | 300 ms |
| TIME | Reverse | On/Off | Off |
| TIME | Tempo Sync | On/Off | Off |
| TIME | Note Division | 1/1 to 1/16T | 1/4 |
| MAIN | Feedback | 0-95% | 40% |
| MAIN | Mix | 0-100% | 30% |
| MAIN | Trim | -12 to +12 dB | 0 dB |
| MAIN | Mode | Digital/Analog | Digital |
| STEREO | Ping Pong | On/Off | Off |
| STEREO | Stereo Offset | 0-50 ms | 10 ms |
| TONE | High-Pass | 20-1000 Hz | 80 Hz |
| TONE | Low-Pass | 500-20000 Hz | 8000 Hz |
| MOD | Mod Rate | 0.1-5 Hz | 0.8 Hz |
| MOD | Mod Depth | 0-100% | 30% |
| MOD | Drive | 0-100% | 20% |
| DUCK | Duck Amount | 0-100% | 0% |
| DUCK | Duck Threshold | 0-100% | 30% |

## Architecture

```
genx_delay_j/
├── CMakeLists.txt       # Build configuration
├── build.sh             # Build script
└── src/
    ├── PluginProcessor.h/cpp   # Audio processing (DSP)
    ├── PluginEditor.h/cpp      # GUI
    ├── DelayLine.h             # Delay line + reverse delay
    ├── Modulation.h            # Stereo LFO modulator
    ├── Filters.h               # Biquad HP/LP filters
    ├── Saturation.h            # Soft saturation
    └── Ducker.h                # Dynamic ducking
```

## Credits

- Built with [JUCE](https://juce.com/) 8.0.4
- Inspired by classic 00s alternative rock delay tones
