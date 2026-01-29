# High-Pass Filter Plugin - Design Document

## Overview

A variable-slope high-pass filter audio effect plugin built with nih-plug (Rust).

| Attribute | Value |
|-----------|-------|
| **Name** | High-Pass Filter |
| **Type** | Audio Effect |
| **Formats** | VST3, CLAP |
| **Channels** | Stereo, Mono |
| **Framework** | nih-plug (Rust) |

---

## 1. Requirements

### Functional Requirements

1. **High-pass filtering** - Attenuate frequencies below a user-defined cutoff
2. **Variable slope** - User-selectable filter steepness: 6, 12, 18, or 24 dB/octave
3. **Resonance control** - Adjustable Q factor for emphasis at cutoff frequency
4. **Real-time parameter changes** - Smooth, click-free parameter automation
5. **Stereo processing** - Independent filtering per channel

### Non-Functional Requirements

1. **Low CPU usage** - Efficient DSP suitable for multiple instances
2. **Low latency** - Zero-latency processing (no lookahead)
3. **Stability** - No denormals, NaN, or infinity in output
4. **DAW compatibility** - Works in Ableton Live, FL Studio (VST3/CLAP hosts)

### Out of Scope (First Iteration)

- Custom GUI (using host-provided generic UI)
- AudioUnit format (Logic Pro)
- Sidechain input
- Oversampling
- Preset system

---

## 2. Architecture

### 2.1 High-Level Structure

```
┌─────────────────────────────────────────────────────────┐
│                    HighPassFilter                        │
│                    (Plugin struct)                       │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────────────────┐ │
│  │  HighPassParams │    │  FilterChain [2]            │ │
│  │  (Arc<Params>)  │    │  (per-channel DSP)          │ │
│  ├─────────────────┤    ├─────────────────────────────┤ │
│  │ - cutoff        │    │ ┌─────────────────────────┐ │ │
│  │ - resonance     │    │ │ BiquadState [2]         │ │ │
│  │ - slope         │    │ │ (cascaded 2nd order)    │ │ │
│  └─────────────────┘    │ ├─────────────────────────┤ │ │
│                         │ │ FirstOrderHPState       │ │ │
│                         │ │ (for 6/18 dB modes)     │ │ │
│                         │ └─────────────────────────┘ │ │
│                         └─────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

### 2.2 Module Organization

```
src/
├── lib.rs          # Plugin entry point, parameters, Plugin trait impl
└── filter.rs       # DSP: BiquadState, FirstOrderHPState, FilterChain
```

### 2.3 Data Flow

```
Audio Input (per sample)
        │
        ▼
┌───────────────────┐
│ Read smoothed     │
│ parameters        │
│ (cutoff, Q, slope)│
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ Update filter     │
│ coefficients      │
│ (if changed)      │
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ FilterChain       │
│ .process(sample)  │
│                   │
│ 6dB:  1st order   │
│ 12dB: 1 biquad    │
│ 18dB: biquad+1st  │
│ 24dB: 2 biquads   │
└───────────────────┘
        │
        ▼
Audio Output
```

---

## 3. Parameter Design

### 3.1 Parameter Table

| Parameter | ID | Type | Range | Default | Unit | Smoothing |
|-----------|-----|------|-------|---------|------|-----------|
| Cutoff | `cutoff` | Float | 20 - 20,000 | 200 | Hz | Logarithmic 50ms |
| Resonance | `resonance` | Float | 0.5 - 10.0 | 0.707 | Q | Linear 50ms |
| Slope | `slope` | Enum | 4 values | 12 dB/oct | - | None |

### 3.2 Slope Enum Values

| ID | Display Name | Filter Stages |
|----|--------------|---------------|
| `6db` | 6 dB/oct | 1 first-order |
| `12db` | 12 dB/oct | 1 biquad |
| `18db` | 18 dB/oct | 1 biquad + 1 first-order |
| `24db` | 24 dB/oct | 2 biquads |

### 3.3 Parameter Behavior

**Cutoff Frequency**
- Logarithmic scaling (skew factor -2.0) for musical control
- Display: "200 Hz", "1.50 kHz", etc.
- Smoothing prevents zipper noise during automation

**Resonance (Q)**
- Default 0.707 = Butterworth (maximally flat passband)
- Higher values create resonant peak at cutoff
- Q > 5 can cause significant boost, may clip

**Slope**
- Discrete selection, no interpolation between modes
- Steeper slopes = more aggressive filtering
- 24 dB/oct useful for surgical low-end removal

---

## 4. DSP Design

### 4.1 Filter Topology

Using **Direct Form 2 Transposed** biquad structure:

```
Input ──►(×b0)──►(+)──────────────────────►(+)──► Output
                 │                          ▲
                 ▼                          │
              [z⁻¹]                         │
                 │                          │
                 ├──►(×b1)──►(+)──►(×-a1)───┤
                 │           ▲              │
                 ▼           │              │
              [z⁻¹]          │              │
                 │           │              │
                 └──►(×b2)───┘──►(×-a2)─────┘
```

**Advantages:**
- Better numerical precision for low frequencies
- Less susceptible to coefficient quantization noise
- Single delay line shared between feedback and feedforward

### 4.2 Coefficient Calculation

Using Robert Bristow-Johnson's Audio EQ Cookbook formulas.

**High-Pass Biquad Coefficients:**

```
ω₀ = 2π × f_c / f_s
α = sin(ω₀) / (2 × Q)

b₀ = (1 + cos(ω₀)) / 2
b₁ = -(1 + cos(ω₀))
b₂ = (1 + cos(ω₀)) / 2
a₀ = 1 + α
a₁ = -2 × cos(ω₀)
a₂ = 1 - α

// Normalize by a₀
b₀' = b₀/a₀, b₁' = b₁/a₀, b₂' = b₂/a₀
a₁' = a₁/a₀, a₂' = a₂/a₀
```

**First-Order High-Pass (for 6dB slope):**

```
ω_c = 2π × f_c
T = 1 / f_s
ω_a = (2/T) × tan(ω_c × T / 2)  // Pre-warping
g = ω_a × T / 2

b₀ = 1 / (1 + g)
b₁ = -b₀
a₁ = (g - 1) / (g + 1)
```

### 4.3 Cascading Strategy

| Slope | Structure | Total Poles |
|-------|-----------|-------------|
| 6 dB/oct | 1st order | 1 pole |
| 12 dB/oct | Biquad | 2 poles |
| 18 dB/oct | Biquad → 1st order | 3 poles |
| 24 dB/oct | Biquad → Biquad | 4 poles |

For 24 dB/oct, both biquads use the same Q value. This creates a resonant peak rather than Butterworth response. True Butterworth 4th-order would require Q values of 0.541 and 1.307 for the two stages.

### 4.4 Processing Loop

```rust
for sample in buffer.iter_samples() {
    // 1. Get smoothed parameter values
    let cutoff = params.cutoff.smoothed.next();
    let q = params.resonance.smoothed.next();
    let slope = params.slope.value();

    // 2. Update coefficients (every sample for smooth automation)
    filter.update_coefficients(sample_rate, cutoff, q, slope);

    // 3. Process through filter chain
    output = filter.process(input);
}
```

---

## 5. Plugin Integration

### 5.1 nih-plug Traits

| Trait | Purpose |
|-------|---------|
| `Plugin` | Core plugin info, audio I/O, process function |
| `Params` | Parameter definitions and serialization |
| `ClapPlugin` | CLAP-specific metadata |
| `Vst3Plugin` | VST3-specific metadata |

### 5.2 Audio I/O Layouts

```rust
const AUDIO_IO_LAYOUTS: &[AudioIOLayout] = &[
    // Stereo (preferred)
    AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
    },
    // Mono fallback
    AudioIOLayout {
        main_input_channels: NonZeroU32::new(1),
        main_output_channels: NonZeroU32::new(1),
    },
];
```

### 5.3 Lifecycle

1. **`default()`** - Create plugin instance with default params
2. **`initialize()`** - Called when plugin loads; store sample rate
3. **`reset()`** - Called on transport stop; clear filter states
4. **`process()`** - Called per audio buffer; main DSP loop

---

## 6. Build System

### 6.1 Cargo Workspace

```
vst/
├── Cargo.toml          # Workspace root
├── .cargo/config.toml  # Alias: cargo xtask = cargo run -p xtask
├── src/                # Plugin source
└── xtask/              # Build/bundle tool
    ├── Cargo.toml
    └── src/main.rs
```

### 6.2 Dependencies

```toml
[dependencies]
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git" }

[lib]
crate-type = ["cdylib"]  # Dynamic library for plugin
```

### 6.3 Build Commands

```bash
# Debug build
cargo build

# Release build + bundle
cargo xtask bundle highpass_filter --release

# Run tests
cargo test
```

### 6.4 Output Artifacts

```
target/bundled/
├── highpass_filter.vst3/
│   └── Contents/
│       ├── Info.plist
│       └── MacOS/
│           └── highpass_filter
└── highpass_filter.clap
```

---

## 7. Testing Strategy

### 7.1 Unit Tests

| Test | Purpose |
|------|---------|
| `test_biquad_coefficients` | Verify coefficient calculation produces valid values |
| `test_dc_rejection` | Confirm HPF attenuates DC (0 Hz) to near zero |
| `test_high_frequency_passthrough` | Verify signals above cutoff pass through |

### 7.2 Manual DAW Testing

1. Load plugin in Ableton/FL Studio
2. Send white noise through filter
3. Use spectrum analyzer to verify:
   - Cutoff frequency matches parameter
   - Slope matches selected dB/oct
   - Resonance creates peak at cutoff
4. Automate cutoff sweep - should be smooth, no clicks
5. Switch slopes during playback - should not crash or click

### 7.3 Edge Cases

- Cutoff at extremes (20 Hz, 20 kHz)
- Q at maximum (10.0) - check for instability
- Rapid parameter changes
- Very low sample rates (22.05 kHz)
- Very high sample rates (192 kHz)

---

## 8. Future Enhancements

### Phase 2: GUI
- Custom iced or egui interface
- Frequency response display
- Input/output level meters

### Phase 3: Advanced Features
- Additional filter types (low-pass, band-pass)
- Oversampling for reduced aliasing
- Sidechain input for ducking
- A/B comparison

### Phase 4: Formats
- AudioUnit for Logic Pro (requires different framework or wrapper)
- AAX for Pro Tools
- LV2 for Linux DAWs

---

## 9. References

1. [Audio EQ Cookbook - Robert Bristow-Johnson](https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html)
2. [nih-plug Documentation](https://nih-plug.robbertvanderhelm.nl/)
3. [nih-plug GitHub](https://github.com/robbert-vdh/nih-plug)
4. [Digital Biquad Filter - Wikipedia](https://en.wikipedia.org/wiki/Digital_biquad_filter)
5. [Direct Form II Transposed](https://ccrma.stanford.edu/~jos/fp/Transposed_Direct_Forms.html)
