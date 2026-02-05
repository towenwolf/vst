use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

mod analyzer;
mod editor;
mod profiles;
mod ring_buffer;

use analyzer::{ChromaExtractor, FftProcessor, KeyDetector};
use ring_buffer::RingBuffer;

/// FFT size options
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FftSize {
    #[id = "2048"]
    #[name = "2048 (Fast)"]
    Size2048,
    #[id = "4096"]
    #[name = "4096 (Default)"]
    #[default]
    Size4096,
    #[id = "8192"]
    #[name = "8192 (Accurate)"]
    Size8192,
}

impl FftSize {
    fn as_usize(&self) -> usize {
        match self {
            FftSize::Size2048 => 2048,
            FftSize::Size4096 => 4096,
            FftSize::Size8192 => 8192,
        }
    }
}

/// Detected note name
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteName {
    #[default]
    C,
    #[name = "C#"]
    CSharp,
    D,
    #[name = "D#"]
    DSharp,
    E,
    F,
    #[name = "F#"]
    FSharp,
    G,
    #[name = "G#"]
    GSharp,
    A,
    #[name = "A#"]
    ASharp,
    B,
}

impl From<usize> for NoteName {
    fn from(value: usize) -> Self {
        match value % 12 {
            0 => NoteName::C,
            1 => NoteName::CSharp,
            2 => NoteName::D,
            3 => NoteName::DSharp,
            4 => NoteName::E,
            5 => NoteName::F,
            6 => NoteName::FSharp,
            7 => NoteName::G,
            8 => NoteName::GSharp,
            9 => NoteName::A,
            10 => NoteName::ASharp,
            11 => NoteName::B,
            _ => NoteName::C,
        }
    }
}

/// Detected mode
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Major,
    Minor,
}

/// Shared analysis output (thread-safe)
#[derive(Default)]
pub struct AnalysisOutput {
    pub root: AtomicU32,
    pub mode: AtomicU32,
    pub confidence: AtomicU32,
}

/// Plugin parameters
#[derive(Params)]
pub struct KeyDetectorParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,

    #[id = "fft_size"]
    fft_size: EnumParam<FftSize>,

    #[id = "smoothing"]
    smoothing: FloatParam,

    #[id = "threshold"]
    threshold: FloatParam,

    // Output parameters (read-only, for host/M4L to read detected values)
    #[id = "out_root"]
    pub out_root: EnumParam<NoteName>,

    #[id = "out_mode"]
    pub out_mode: EnumParam<Mode>,

    #[id = "out_confidence"]
    pub out_confidence: FloatParam,
}

impl Default for KeyDetectorParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            fft_size: EnumParam::new("FFT Size", FftSize::Size4096),

            smoothing: FloatParam::new(
                "Smoothing",
                0.3,
                FloatRange::Skewed {
                    min: 0.05,
                    max: 2.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_unit(" s")
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            threshold: FloatParam::new(
                "Threshold",
                20.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 50.0,
                },
            )
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_rounded(0)),

            // Output parameters - these are updated by the plugin to expose detected values
            out_root: EnumParam::new("Detected Root", NoteName::C)
                .hide()
                .non_automatable(),

            out_mode: EnumParam::new("Detected Mode", Mode::Major)
                .hide()
                .non_automatable(),

            out_confidence: FloatParam::new(
                "Detected Confidence",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_unit(" %")
            .hide()
            .non_automatable(),
        }
    }
}

/// Key Detector Plugin
struct KeyDetectorPlugin {
    params: Arc<KeyDetectorParams>,
    sample_rate: f32,
    output: Arc<AnalysisOutput>,

    // DSP components
    ring_buffer: RingBuffer,
    fft_processor: FftProcessor,
    chroma_extractor: ChromaExtractor,
    key_detector: KeyDetector,

    // Processing state
    samples_since_fft: usize,
    current_fft_size: usize,
    fft_buffer: Vec<f32>,
}

impl Default for KeyDetectorPlugin {
    fn default() -> Self {
        let fft_size = 4096;
        let sample_rate = 44100.0;
        let smoothing = 0.3;

        Self {
            params: Arc::new(KeyDetectorParams::default()),
            sample_rate,
            output: Arc::new(AnalysisOutput::default()),

            ring_buffer: RingBuffer::new(fft_size),
            fft_processor: FftProcessor::new(fft_size),
            chroma_extractor: ChromaExtractor::new(sample_rate, fft_size, smoothing),
            key_detector: KeyDetector::new(10, 0.1),

            samples_since_fft: 0,
            current_fft_size: fft_size,
            fft_buffer: vec![0.0; fft_size],
        }
    }
}

impl KeyDetectorPlugin {
    fn reconfigure(&mut self, fft_size: usize, smoothing: f32) {
        if fft_size != self.current_fft_size {
            self.current_fft_size = fft_size;
            self.ring_buffer.resize(fft_size);
            self.fft_processor.resize(fft_size);
            self.fft_buffer.resize(fft_size, 0.0);
        }

        self.chroma_extractor
            .reconfigure(self.sample_rate, fft_size, smoothing);
    }
}

impl Plugin for KeyDetectorPlugin {
    const NAME: &'static str = "Key Detector";
    const VENDOR: &'static str = "trwolf";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        // Stereo
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        // Mono
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = false;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.output.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        let fft_size = self.params.fft_size.value().as_usize();
        let smoothing = self.params.smoothing.value();

        self.reconfigure(fft_size, smoothing);

        true
    }

    fn reset(&mut self) {
        self.ring_buffer.reset();
        self.chroma_extractor.reset();
        self.key_detector.reset();
        self.samples_since_fft = 0;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Check for parameter changes
        let fft_size = self.params.fft_size.value().as_usize();
        let smoothing = self.params.smoothing.value();
        let threshold = self.params.threshold.value();

        if fft_size != self.current_fft_size {
            self.reconfigure(fft_size, smoothing);
        } else {
            self.chroma_extractor.set_smoothing(smoothing);
        }

        let hop_size = fft_size / 4;
        let num_channels = buffer.channels();

        for sample_idx in 0..buffer.samples() {
            // Sum to mono
            let mono_sample = if num_channels >= 2 {
                (buffer.as_slice()[0][sample_idx] + buffer.as_slice()[1][sample_idx]) * 0.5
            } else {
                buffer.as_slice()[0][sample_idx]
            };

            // Add to ring buffer
            self.ring_buffer.push(mono_sample);
            self.samples_since_fft += 1;

            // Check if we should run FFT
            if self.samples_since_fft >= hop_size {
                self.samples_since_fft = 0;

                // Copy samples from ring buffer
                self.ring_buffer.copy_to_slice(&mut self.fft_buffer);

                // Run FFT
                let magnitude = self.fft_processor.process(&self.fft_buffer);

                // Extract chroma
                let chroma = self.chroma_extractor.process(magnitude);

                // Detect key
                let result = self.key_detector.update(chroma);

                // Update output if above threshold
                let confidence = result.confidence();
                if confidence >= threshold {
                    self.output.root.store(result.root as u32, Ordering::Relaxed);
                    self.output
                        .mode
                        .store(if result.is_major { 0 } else { 1 }, Ordering::Relaxed);
                }
                self.output
                    .confidence
                    .store((confidence * 100.0) as u32, Ordering::Relaxed);
            }
        }

        // Audio pass-through (analyzer doesn't modify audio)
        ProcessStatus::Normal
    }
}

impl ClapPlugin for KeyDetectorPlugin {
    const CLAP_ID: &'static str = "com.trwolf.key-detector";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Real-time musical key detection (root note and mode)");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Analyzer,
        ClapFeature::Stereo,
        ClapFeature::Mono,
    ];
}

impl Vst3Plugin for KeyDetectorPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"TrwolfKeyDetect!";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Analyzer];
}

nih_export_clap!(KeyDetectorPlugin);
nih_export_vst3!(KeyDetectorPlugin);
