# GenX Delay - Product Requirements Document

**Version:** 1.0
**Date:** 2026-02-13
**Author:** trwolf
**Status:** Active Development

---

## 1. Product Overview

GenX Delay is a stereo delay audio plugin inspired by 00s alternative/rock delay tones. It features a dual-mode architecture (Digital/Analog), grain-based reverse delay, ping-pong stereo, feedback tone shaping, dynamic ducking, and a Pioneer VFD receiver-inspired GUI.

**Target Formats:** VST3, AU (macOS), Standalone
**Target Platforms:** macOS, Windows, Linux
**Framework:** JUCE 8.0.4 / C++17

---

## 2. Target User

- Music producers and mix engineers working in rock, alternative, shoegaze, post-punk, and experimental genres
- Musicians who want a delay that sounds "alive" in Analog mode and pristine in Digital mode
- Users of DAWs supporting VST3 (all platforms) or AU (macOS)

---

## 3. Core Features

### 3.1 Delay Engine

| Requirement | Spec | Status |
|-------------|------|--------|
| Max delay time | 2500 ms | Done |
| Linear interpolation for fractional delay | Yes | Done |
| Smooth delay time changes (10 Hz one-pole LP) | Yes | Done |
| Tempo sync from host BPM | 13 note divisions (1/1 to 1/16T) | Done |
| Reverse delay (grain-based, Hann-windowed) | 2 overlapping grains, click-free | Done |
| Feedback range | 0-95% (hard-capped for safety) | Done |
| Safety limiter | Soft peak follower, threshold 0.95 | Done |
| Tail length reported to host | 2.5 seconds | Done |

### 3.2 Dual Mode System

| Mode | Description | Status |
|------|-------------|--------|
| **Digital** | Clean delay path, no modulation or saturation | Done |
| **Analog** | Adds stereo LFO modulation, soft saturation (tanh), and drive control | Done |

### 3.3 Stereo Processing

| Feature | Spec | Status |
|---------|------|--------|
| Ping-pong mode | L/R crossfeed in feedback loop | Done |
| Stereo offset | 0-50 ms offset between L/R for width | Done |
| Stereo modulation | 180-degree phase offset LFO between channels | Done |

### 3.4 Tone Shaping

| Feature | Spec | Status |
|---------|------|--------|
| High-pass filter in feedback | 20-1000 Hz, Butterworth (Q=0.707) | Done |
| Low-pass filter in feedback | 500-20000 Hz, Butterworth (Q=0.707) | Done |
| Saturation (Analog mode) | Pade approximant tanh, drive 1-5x | Done |

### 3.5 Dynamics

| Feature | Spec | Status |
|---------|------|--------|
| Ducking | Envelope follower (5ms attack / 200ms release) | Done |
| Duck amount | 0-100% | Done |
| Duck threshold | 0-100% | Done |

### 3.6 Signal Flow

```
INPUT -> Ducker Envelope Detection
      -> Modulation (Analog mode only)
      -> Delay Time Smoothing
      -> Read from Delay Line (forward or reverse)
      -> Feedback Filter (HP -> LP)
      -> Saturator (Analog mode, drive > 0)
      -> Ping-Pong crossfeed (if enabled)
      -> Feedback mixback
      -> Write to Delay Line
      -> Dry/Wet Mix
      -> Ducker application
      -> Trim gain
      -> Safety Limiter
      -> OUTPUT
```

---

## 4. Parameters

### 4.1 Complete Parameter Table

| ID | Section | Parameter | Range | Default | Type |
|----|---------|-----------|-------|---------|------|
| 1 | TIME | Delay Time | 1-2500 ms | 300 ms | Float (skewed) |
| 2 | TIME | Reverse | On/Off | Off | Bool |
| 3 | TIME | Tempo Sync | On/Off | Off | Bool |
| 4 | TIME | Note Division | 1/1 to 1/16T | 1/4 | Choice (13) |
| 5 | MAIN | Feedback | 0-95% | 40% | Float |
| 6 | MAIN | Mix | 0-100% | 30% | Float |
| 7 | MAIN | Trim | -12 to +12 dB | 0 dB | Float |
| 8 | MAIN | Mode | Digital/Analog | Digital | Choice (2) |
| 9 | STEREO | Ping Pong | On/Off | Off | Bool |
| 10 | STEREO | Stereo Offset | 0-50 ms | 10 ms | Float |
| 11 | TONE | High-Pass | 20-1000 Hz | 80 Hz | Float (skewed) |
| 12 | TONE | Low-Pass | 500-20000 Hz | 8000 Hz | Float (skewed) |
| 13 | MOD | Mod Rate | 0.1-5 Hz | 0.8 Hz | Float (skewed) |
| 14 | MOD | Mod Depth | 0-100% | 30% | Float |
| 15 | MOD | Drive | 0-100% | 20% | Float |
| 16 | DUCK | Duck Amount | 0-100% | 0% | Float |
| 17 | DUCK | Duck Threshold | 0-100% | 30% | Float |

---

## 5. GUI Requirements

### 5.1 Visual Theme: Pioneer VFD Receiver

| Element | Spec | Status |
|---------|------|--------|
| Theme | Matte black chassis, amber VFD phosphor glow | Done |
| Primary color | Amber RGB(255, 176, 0) | Done |
| Background | Deep black RGB(10, 10, 12) | Done |
| Dim/inactive | RGB(140, 95, 0) | Done |
| Display fonts | DSEG14Classic (headers), DSEG7Classic (values) | Done |
| UI fonts | JosefinSans, BebasNeue, Righteous | Done |
| All fonts embedded | Binary data, no external dependencies | Done |

### 5.2 Layout

| Element | Spec | Status |
|---------|------|--------|
| Default size | 660 x 580 px | Done |
| Resizable | Yes, aspect ratio locked (660:580) | Done |
| Min/Max | 530x460 to 1250x1100 | Done |
| Responsive columns | 3 cols (>=560w), 2 cols (>=380w), 1 col (<380w) | Done |
| Section organization | TIME, MAIN, STEREO, TONE, MODULATION, DUCK | Done |
| Title display | "GENX DELAY" in VFD style with phosphor bloom | Done |
| Vignette effect | Edge darkening on all sides | Done |
| Background texture | Procedural grain noise | Done |

### 5.3 Custom Components

| Component | Spec | Status |
|-----------|------|--------|
| PioneerLookAndFeel | Full custom renderer | Done |
| Rotary knobs | Ribbed body, amber arc, red indicator, glow halo | Done |
| Toggle buttons | Lit vs recessed states, phosphor glow | Done |
| Combo box | Dark recessed display, amber arrow | Done |
| VFD text renderer | 3-layer phosphor bloom effect | Done |
| Modulation section dimming | 0.3 alpha when Digital mode | Done |

---

## 6. Technical Requirements

### 6.1 Audio Performance

| Requirement | Spec |
|-------------|------|
| Real-time safe | No allocations in processBlock |
| Sample rates | 44.1k, 48k, 96k, 192k+ |
| Buffer sizes | Any (host-determined) |
| Audio I/O | Stereo in/out (mono input auto-promoted) |
| Thread safety | Atomic peak metering, relaxed memory order |
| Parameter smoothing | One-pole LP for delay time changes |

### 6.2 Build System

| Requirement | Spec |
|-------------|------|
| Build system | CMake 3.22+ |
| C++ standard | C++17 |
| JUCE version | 8.0.4 (FetchContent) |
| Compile defs | No web browser, no CURL, no splash screen |
| Binary data | 5 embedded fonts via juce_add_binary_data |

### 6.3 State Management

| Requirement | Spec | Status |
|-------------|------|--------|
| Parameter persistence | APVTS XML serialization | Done |
| Preset save/load | Via host (DAW preset system) | Done |
| Undo/redo | Via APVTS (host-managed) | Done |

---

## 7. Future Roadmap

### Phase 1: Polish & Release Prep (Current)
- [ ] QA pass across multiple DAWs (Logic, Ableton, Reaper, FL Studio)
- [ ] CPU profiling and optimization pass
- [ ] Validate all parameter ranges and edge cases
- [ ] Test at extreme sample rates (192kHz)
- [ ] Test with mono/stereo/surround bus configurations
- [ ] Accessibility review (keyboard navigation, screen reader labels)

### Phase 2: Preset System
- [ ] Factory preset bank (8-16 presets showcasing all modes)
- [ ] Preset browser UI (dropdown or panel)
- [ ] User preset save/load (file-based or host-managed)
- [ ] Preset categories (Clean, Analog, Reverse, Rhythmic, Ambient)

### Phase 3: Visual Enhancements
- [ ] Real-time waveform or delay tap visualization
- [ ] Input/output level metering (VU-style, VFD amber theme)
- [ ] Animated feedback path indicator
- [ ] A/B comparison toggle

### Phase 4: Feature Expansion
- [ ] Multi-tap delay mode (2-4 taps with independent time/feedback/pan)
- [ ] Mid-side processing option
- [ ] Modulation matrix (LFO -> any parameter)
- [ ] External sidechain input for ducking
- [ ] Freeze/infinite hold mode
- [ ] Stereo width control (Haas effect fine-tuning)

### Phase 5: Distribution
- [ ] Installer packages (macOS .pkg, Windows .msi)
- [ ] Code signing and notarization (macOS)
- [ ] User manual / documentation site
- [ ] Demo version with time-limited bypass
- [ ] Landing page and marketing materials

---

## 8. Architecture Reference

### 8.1 Source File Map

```
src/
  PluginProcessor.h/cpp   -- AudioProcessor, APVTS, DSP orchestration
  PluginEditor.h/cpp       -- GUI, PioneerLookAndFeel, layout
  DelayLine.h/cpp          -- DelayLine + ReverseDelayLine (grain-based)
  Modulation.h/cpp         -- StereoModulator (sine LFO)
  Filters.h/cpp            -- FeedbackFilter (biquad HP+LP)
  Saturation.h/cpp         -- Saturator (tanh Pade approximant)
  Ducker.h/cpp             -- Ducker (envelope follower)
assets/fonts/              -- 5 embedded TTF files
CMakeLists.txt             -- Build configuration
build.sh                   -- macOS build helper script
```

### 8.2 Class Relationships

```
GenXDelayProcessor (AudioProcessor)
  |-- DelayLine x2 (L/R)
  |-- ReverseDelayLine x2 (L/R)
  |-- FeedbackFilter x2 (L/R)
  |-- StereoModulator
  |-- Saturator x2 (L/R)
  |-- Ducker
  |-- OnePoleLP x2 (smoothers)
  |-- APVTS (parameter state)

GenXDelayEditor (AudioProcessorEditor, APVTS::Listener)
  |-- PioneerLookAndFeel
  |-- Sliders x12, Labels x12
  |-- ToggleButtons x4, TextButtons x2
  |-- ComboBox x1
  |-- Attachments (Slider/Button/ComboBox)
```

---

## 9. Acceptance Criteria

The plugin is considered release-ready when:

1. **Stability:** No crashes across 4+ hours of continuous use in at least 3 DAWs
2. **Audio quality:** Clean signal path in Digital mode (null test passes), warm coloration in Analog mode
3. **CPU:** < 3% single-core usage at 44.1kHz/512 buffer on modern hardware
4. **Parameters:** All 17 parameters automate correctly, save/restore state, and display accurate values
5. **GUI:** Responsive at all supported sizes, no visual glitches, consistent VFD theme
6. **Compatibility:** Validated in Logic Pro, Ableton Live, Reaper (minimum)
7. **Formats:** VST3 and AU scan and load without errors
8. **Edge cases:** Graceful behavior at 0% mix, 95% feedback, 1ms delay, 2500ms delay, reverse + ping-pong combo
