# Musical Key Detector VST3 Plugin - Design Document

## Overview

A real-time musical key detection plugin that identifies the root note and mode (major/minor) of audio input.

| Attribute | Value |
|-----------|-------|
| **Name** | Key Detector |
| **Type** | Audio Analyzer |
| **Formats** | VST3, CLAP |
| **Channels** | Stereo (summed to mono), Mono |
| **Framework** | nih-plug (Rust) |
| **GUI** | None (host generic UI) |

### Target Use Cases

| Use Case | Description |
|----------|-------------|
| **Production** | Identify key of samples/loops for harmonic compatibility |
| **Live Performance** | DJs monitoring incoming audio for seamless mixing |
| **Music Education** | Real-time harmonic analysis for students |
| **Sound Design** | Identify tonal characteristics of complex textures |

---

## 1. DSP Architecture

### 1.1 FFT Configuration

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| FFT Size | 4096 (default) | 10.8 Hz resolution at 44.1kHz, good balance |
| Overlap | 75% | Smooth updates (~43 Hz) |
| Hop Size | 1024 samples | FFT_SIZE / 4 |
| Window | Hann | Low spectral leakage, good frequency resolution |

**Selectable FFT Sizes:**

| Size | Frequency Resolution (44.1kHz) | Update Latency |
|------|-------------------------------|----------------|
| 2048 | 21.5 Hz | 11.6 ms |
| 4096 | 10.8 Hz | 23.2 ms |
| 8192 | 5.4 Hz | 46.4 ms |

### 1.2 Hann Window Function

```
w[n] = 0.5 × (1 - cos(2π × n / (N - 1)))
```

Pre-computed at initialization for efficiency.

### 1.3 Frequency-to-Bin Mapping

```
freq[k] = k × sample_rate / fft_size
```

For 44.1kHz and 4096 FFT: Bin 1 = 10.77 Hz

---

## 2. Chroma Feature Extraction

### 2.1 Algorithm

1. **Compute magnitude spectrum** from FFT output
2. **Filter frequency range**: 65 Hz (C2) to 2000 Hz (B6)
3. **Map bins to pitch classes** (0-11, where C=0)
4. **Accumulate magnitudes** into 12-bin chroma vector
5. **Normalize** chroma vector (sum to 1.0)

### 2.2 Bin-to-Pitch-Class Mapping

```rust
fn bin_to_pitch_class(bin: usize, sample_rate: f32, fft_size: usize) -> Option<usize> {
    let freq = bin as f32 * sample_rate / fft_size as f32;

    // Filter: 65 Hz (C2) to 2000 Hz (B6)
    if freq < 65.0 || freq > 2000.0 {
        return None;
    }

    // Convert to MIDI note (A4 = 440 Hz = MIDI 69)
    let midi_note = 12.0 * (freq / 440.0).log2() + 69.0;

    // Pitch class = MIDI note mod 12
    Some((midi_note.round() as i32).rem_euclid(12) as usize)
}
```

### 2.3 Chroma Accumulation

```rust
let mut chroma = [0.0f32; 12];

for (bin, &mag) in magnitude.iter().enumerate() {
    if let Some(pc) = bin_to_pitch_class(bin, sample_rate, fft_size) {
        chroma[pc] += mag;
    }
}

// Normalize
let sum: f32 = chroma.iter().sum();
if sum > 0.0 {
    for c in &mut chroma {
        *c /= sum;
    }
}
```

### 2.4 Temporal Smoothing (EMA)

```rust
fn smooth_chroma(current: &[f32; 12], smoothed: &mut [f32; 12], alpha: f32) {
    for i in 0..12 {
        smoothed[i] = alpha * current[i] + (1.0 - alpha) * smoothed[i];
    }
}

// Alpha from time constant (tau in seconds)
fn alpha_from_tau(tau: f32, hop_time: f32) -> f32 {
    1.0 - (-hop_time / tau).exp()
}
```

| Smoothing | Tau | Alpha (43 Hz) | Use Case |
|-----------|-----|---------------|----------|
| Fast | 0.1s | 0.20 | Quick response |
| Medium | 0.3s | 0.07 | Default |
| Slow | 1.0s | 0.02 | Stable output |

---

## 3. Key Detection Algorithm

### 3.1 Krumhansl-Schmuckler Key Profiles

Based on Krumhansl & Kessler (1982) perceptual studies:

```rust
// Index: C=0, C#=1, D=2, ... B=11
const MAJOR_PROFILE: [f32; 12] = [
    6.35, 2.23, 3.48, 2.33, 4.38, 4.09,
    2.52, 5.19, 2.39, 3.66, 2.29, 2.88
];

const MINOR_PROFILE: [f32; 12] = [
    6.33, 2.68, 3.52, 5.38, 2.60, 3.53,
    2.54, 4.75, 3.98, 2.69, 3.34, 3.17
];
```

**Profile Interpretation:**
- Major: Tonic (6.35) > Fifth (5.19) > Third (4.38)
- Minor: Tonic (6.33) > Minor Third (5.38) > Fifth (4.75)

### 3.2 Correlation-Based Detection

**Step 1: Z-Score Normalization**

```rust
fn z_normalize(v: &[f32; 12]) -> [f32; 12] {
    let mean: f32 = v.iter().sum::<f32>() / 12.0;
    let variance: f32 = v.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / 12.0;
    let std_dev = variance.sqrt();

    let mut normalized = [0.0f32; 12];
    if std_dev > 1e-10 {
        for i in 0..12 {
            normalized[i] = (v[i] - mean) / std_dev;
        }
    }
    normalized
}
```

**Step 2: Pearson Correlation**

```rust
fn pearson_correlation(a: &[f32; 12], b: &[f32; 12]) -> f32 {
    // Both inputs are z-normalized
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>() / 12.0
}
```

**Step 3: Profile Rotation**

```rust
fn rotate_profile(profile: &[f32; 12], semitones: usize) -> [f32; 12] {
    let mut rotated = [0.0f32; 12];
    for i in 0..12 {
        rotated[i] = profile[(i + 12 - semitones) % 12];
    }
    rotated
}
```

**Step 4: Find Best Match (24 keys)**

```rust
struct KeyResult {
    root: usize,      // 0-11 (C=0)
    is_major: bool,
    correlation: f32, // -1.0 to 1.0
}

fn detect_key(chroma: &[f32; 12]) -> KeyResult {
    let chroma_norm = z_normalize(chroma);
    let major_norm = z_normalize(&MAJOR_PROFILE);
    let minor_norm = z_normalize(&MINOR_PROFILE);

    let mut best = KeyResult { root: 0, is_major: true, correlation: -1.0 };

    for root in 0..12 {
        // Test major
        let major_corr = pearson_correlation(
            &chroma_norm,
            &z_normalize(&rotate_profile(&MAJOR_PROFILE, root))
        );
        if major_corr > best.correlation {
            best = KeyResult { root, is_major: true, correlation: major_corr };
        }

        // Test minor
        let minor_corr = pearson_correlation(
            &chroma_norm,
            &z_normalize(&rotate_profile(&MINOR_PROFILE, root))
        );
        if minor_corr > best.correlation {
            best = KeyResult { root, is_major: false, correlation: minor_corr };
        }
    }

    best
}
```

### 3.3 Confidence Scoring

| Correlation | Confidence | Interpretation |
|-------------|------------|----------------|
| > 0.8 | High | Strong tonal content |
| 0.5 - 0.8 | Medium | Likely correct |
| 0.2 - 0.5 | Low | Ambiguous |
| < 0.2 | Very Low | No clear key |

```rust
fn correlation_to_confidence(corr: f32) -> f32 {
    ((corr + 1.0) / 2.0 * 100.0).clamp(0.0, 100.0)
}
```

### 3.4 Output Hysteresis

Prevent rapid flickering between detected keys:

```rust
struct KeyDetector {
    current_key: KeyResult,
    hold_counter: u32,
    min_hold_frames: u32,  // e.g., 10 frames
}

impl KeyDetector {
    fn update(&mut self, new_key: KeyResult) -> KeyResult {
        let same_key = new_key.root == self.current_key.root
                    && new_key.is_major == self.current_key.is_major;

        if same_key {
            self.hold_counter = self.hold_counter.saturating_add(1);
            self.current_key.correlation = new_key.correlation;
        } else if new_key.correlation > self.current_key.correlation + 0.1
               || self.hold_counter >= self.min_hold_frames {
            self.current_key = new_key;
            self.hold_counter = 0;
        }

        self.current_key
    }
}
```

---

## 4. Plugin Parameters

### 4.1 User Controls

| Parameter | ID | Type | Range | Default | Description |
|-----------|-----|------|-------|---------|-------------|
| FFT Size | `fft_size` | Enum | 2048/4096/8192 | 4096 | Analysis window |
| Smoothing | `smoothing` | Float | 0.05 - 2.0 s | 0.3 s | Time constant |
| Threshold | `threshold` | Float | 0 - 50% | 20% | Min confidence |

### 4.2 Read-Only Outputs

| Parameter | ID | Type | Range | Description |
|-----------|-----|------|-------|-------------|
| Root Note | `root_note` | Enum | C-B | Detected tonal center |
| Mode | `mode` | Enum | Major/Minor | Detected tonality |
| Confidence | `confidence` | Float | 0-100% | Detection certainty |

### 4.3 Enum Definitions

```rust
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FftSize {
    #[id = "2048"] #[name = "2048 (Fast)"]
    Size2048,
    #[id = "4096"] #[name = "4096 (Default)"] #[default]
    Size4096,
    #[id = "8192"] #[name = "8192 (Accurate)"]
    Size8192,
}

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteName {
    #[default] C, #[name = "C#"] CSharp, D, #[name = "D#"] DSharp,
    E, F, #[name = "F#"] FSharp, G, #[name = "G#"] GSharp,
    A, #[name = "A#"] ASharp, B,
}

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default] Major,
    Minor,
}
```

### 4.4 Thread-Safe Output

```rust
use std::sync::atomic::{AtomicU8, AtomicU32, Ordering};

pub struct AnalysisOutput {
    pub detected_root: AtomicU8,   // 0-11
    pub detected_mode: AtomicU8,   // 0=Major, 1=Minor
    pub confidence: AtomicU32,     // Fixed-point × 100
}
```

---

## 5. Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                       KeyDetectorPlugin                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Audio Input (Stereo)                                           │
│        │                                                         │
│        ▼                                                         │
│  ┌──────────────┐                                               │
│  │ Sum to Mono  │  (L + R) / 2                                  │
│  └──────────────┘                                               │
│        │                                                         │
│        ▼                                                         │
│  ┌──────────────────────────┐                                   │
│  │     Ring Buffer          │  Accumulate FFT_SIZE samples      │
│  │     (4096 samples)       │                                   │
│  └──────────────────────────┘                                   │
│        │                                                         │
│        │ Every hop_size (1024) samples                          │
│        ▼                                                         │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐       │
│  │ Hann Window  │───▶│   RealFFT    │───▶│  Magnitude   │       │
│  └──────────────┘    │  (realfft)   │    │  Spectrum    │       │
│                      └──────────────┘    └──────────────┘       │
│                                                 │                │
│                                                 ▼                │
│                      ┌──────────────────────────────────┐       │
│                      │  Bin → Pitch Class Mapping       │       │
│                      │  (65 Hz - 2000 Hz range)         │       │
│                      └──────────────────────────────────┘       │
│                                                 │                │
│                                                 ▼                │
│                      ┌──────────────────────────────────┐       │
│                      │  Chroma Vector [12]              │       │
│                      │  (C, C#, D, D#, E, F, ...)       │       │
│                      └──────────────────────────────────┘       │
│                                                 │                │
│                                                 ▼                │
│                      ┌──────────────────────────────────┐       │
│                      │  Exponential Moving Average      │       │
│                      │  (configurable time constant)    │       │
│                      └──────────────────────────────────┘       │
│                                                 │                │
│                                                 ▼                │
│                      ┌──────────────────────────────────┐       │
│                      │  Krumhansl-Schmuckler            │       │
│                      │  Key Detection (24 correlations) │       │
│                      └──────────────────────────────────┘       │
│                                                 │                │
│                                                 ▼                │
│                      ┌──────────────────────────────────┐       │
│                      │  Hysteresis Filter               │       │
│                      │  (prevent flickering)            │       │
│                      └──────────────────────────────────┘       │
│                                                 │                │
│                                                 ▼                │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              Arc<AnalysisOutput> (Atomic)                 │   │
│  │  ┌────────────┐  ┌────────────┐  ┌──────────────────┐    │   │
│  │  │ root: U8   │  │ mode: U8   │  │ confidence: U32  │    │   │
│  │  └────────────┘  └────────────┘  └──────────────────┘    │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  Audio Output = Audio Input (pass-through)                      │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Performance Considerations

### 6.1 Pre-computed Tables

| Table | Size | Computed At |
|-------|------|-------------|
| Hann window | FFT_SIZE × f32 | `initialize()` |
| Bin-to-pitch-class LUT | (FFT_SIZE/2+1) × Option<u8> | `initialize()` |
| Normalized key profiles | 2 × 12 × f32 | Const |

### 6.2 Memory Allocation

All buffers pre-allocated at initialization:

```rust
struct Analyzer {
    ring_buffer: Vec<f32>,         // FFT_SIZE
    fft_input: Vec<f32>,           // FFT_SIZE
    fft_output: Vec<Complex32>,    // FFT_SIZE/2 + 1
    fft_scratch: Vec<Complex32>,   // realfft scratch
    magnitude: Vec<f32>,           // FFT_SIZE/2 + 1
    hann_window: Vec<f32>,         // FFT_SIZE
    bin_to_pc_lut: Vec<Option<u8>>,// FFT_SIZE/2 + 1
    current_chroma: [f32; 12],
    smoothed_chroma: [f32; 12],
}
```

### 6.3 CPU Budget

| FFT Size | FFT Time (est.) | Full Cycle |
|----------|-----------------|------------|
| 2048 | ~15 μs | ~25 μs |
| 4096 | ~35 μs | ~50 μs |
| 8192 | ~80 μs | ~100 μs |

At 43 Hz update rate (4096 FFT): ~0.2% CPU on modern hardware.

### 6.4 Ring Buffer

```rust
pub struct RingBuffer {
    data: Vec<f32>,
    write_pos: usize,
    capacity: usize,
}

impl RingBuffer {
    #[inline]
    pub fn push(&mut self, sample: f32) {
        self.data[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.capacity;
    }

    pub fn copy_to_slice(&self, output: &mut [f32]) {
        // Handle wrap-around...
    }
}
```

---

## 7. Dependencies

### 7.1 Required Crates

```toml
[dependencies]
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git" }
realfft = "3.5"       # Real-to-complex FFT
num-complex = "0.4"   # Complex number types
```

### 7.2 Rationale

| Crate | Purpose |
|-------|---------|
| `nih_plug` | Plugin framework (matches existing project) |
| `realfft` | 2× faster than generic FFT for real input |
| `num-complex` | Complex arithmetic (transitive from realfft) |

---

## 8. Module Organization

```
src/
├── lib.rs              # Plugin entry, parameters, traits
├── analyzer/
│   ├── mod.rs          # Module exports
│   ├── fft.rs          # FFT wrapper, window functions
│   ├── chroma.rs       # Chroma extraction
│   ├── key_detect.rs   # Krumhansl-Schmuckler
│   └── hysteresis.rs   # Output smoothing
├── ring_buffer.rs      # Sample accumulation
└── profiles.rs         # Key profile constants
```

---

## 9. Testing Strategy

### 9.1 Unit Tests

| Test | Input | Expected |
|------|-------|----------|
| Pure 440 Hz sine | A4 tone | Pitch class 9 (A) dominant |
| C major chord | C4+E4+G4 | C, E, G bins high |
| C major detection | C major chroma | Root=0, Major, high confidence |
| A minor detection | A minor chroma | Root=9, Minor, high confidence |
| All keys | Rotated profiles | Correct root for each |
| Self-correlation | Profile vs itself | correlation = 1.0 |

### 9.2 Test Signals

| Signal | Purpose |
|--------|---------|
| 440 Hz sine | Single pitch mapping |
| C major chord | Chord detection |
| C major scale | Scale detection |
| White noise | Low confidence expected |
| Silence | Edge case, no crash |
| DC offset | Should be filtered |

### 9.3 DAW Testing

| Test | Expectation |
|------|-------------|
| Load in Ableton | Plugin loads, parameters visible |
| Load in FL Studio | Key displayed in generic UI |
| Change sample rate | Plugin reconfigures |
| Change buffer size | No artifacts |
| Bypass | Zero CPU |

---

## 10. Future Enhancements

### Phase 2: GUI
- Chroma histogram visualization
- Confidence meter
- Key wheel display

### Phase 3: Advanced
- Relative major/minor detection
- Key change detection over time
- Export detected key as MIDI CC

---

## References

1. Krumhansl, C. L. (1990). *Cognitive Foundations of Musical Pitch*
2. [Audio EQ Cookbook - Robert Bristow-Johnson](https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html)
3. [nih-plug Documentation](https://nih-plug.robbertvanderhelm.nl/)
4. [realfft Documentation](https://docs.rs/realfft/latest/realfft/)
5. [Krumhansl-Schmuckler Python Implementation](https://gist.github.com/bmcfee/1f66825cef2eb34c839b42dddbad49fd)
