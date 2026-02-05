//! GenX Delay - An emulation of delays popular in 00s alternative/rock music.
//! Inspired by the warm, modulated delay sounds of artists like Incubus.

use nih_plug::prelude::*;
use std::sync::Arc;

mod delay_line;
mod ducker;
mod filters;
mod modulation;
mod saturation;

use delay_line::DelayLine;
use ducker::Ducker;
use filters::{FeedbackFilter, OnePoleLP};
use modulation::StereoModulator;
use saturation::Saturator;

/// Maximum delay time in seconds.
const MAX_DELAY_SECONDS: f32 = 2.5;

/// Default L/R offset in milliseconds for subtle stereo width.
const DEFAULT_LR_OFFSET_MS: f32 = 10.0;

/// Delay mode: Digital (clean) or Analog (warm with modulation).
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DelayMode {
    #[id = "digital"]
    #[name = "Digital"]
    #[default]
    Digital,
    #[id = "analog"]
    #[name = "Analog"]
    Analog,
}

/// Tempo sync note divisions.
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteDivision {
    #[id = "1/1"]
    #[name = "1/1"]
    Whole,
    #[id = "1/2"]
    #[name = "1/2"]
    Half,
    #[id = "1/2d"]
    #[name = "1/2 dotted"]
    HalfDotted,
    #[id = "1/2t"]
    #[name = "1/2 triplet"]
    HalfTriplet,
    #[id = "1/4"]
    #[name = "1/4"]
    #[default]
    Quarter,
    #[id = "1/4d"]
    #[name = "1/4 dotted"]
    QuarterDotted,
    #[id = "1/4t"]
    #[name = "1/4 triplet"]
    QuarterTriplet,
    #[id = "1/8"]
    #[name = "1/8"]
    Eighth,
    #[id = "1/8d"]
    #[name = "1/8 dotted"]
    EighthDotted,
    #[id = "1/8t"]
    #[name = "1/8 triplet"]
    EighthTriplet,
    #[id = "1/16"]
    #[name = "1/16"]
    Sixteenth,
    #[id = "1/16d"]
    #[name = "1/16 dotted"]
    SixteenthDotted,
    #[id = "1/16t"]
    #[name = "1/16 triplet"]
    SixteenthTriplet,
}

impl NoteDivision {
    /// Get the note division as a multiplier of one beat (quarter note).
    fn as_beat_multiplier(self) -> f32 {
        match self {
            NoteDivision::Whole => 4.0,
            NoteDivision::Half => 2.0,
            NoteDivision::HalfDotted => 3.0,
            NoteDivision::HalfTriplet => 4.0 / 3.0,
            NoteDivision::Quarter => 1.0,
            NoteDivision::QuarterDotted => 1.5,
            NoteDivision::QuarterTriplet => 2.0 / 3.0,
            NoteDivision::Eighth => 0.5,
            NoteDivision::EighthDotted => 0.75,
            NoteDivision::EighthTriplet => 1.0 / 3.0,
            NoteDivision::Sixteenth => 0.25,
            NoteDivision::SixteenthDotted => 0.375,
            NoteDivision::SixteenthTriplet => 1.0 / 6.0,
        }
    }
}

/// Plugin parameters.
#[derive(Params)]
struct GenXDelayParams {
    // === Main Controls ===
    #[id = "delay_time"]
    pub delay_time: FloatParam,

    #[id = "tempo_sync"]
    pub tempo_sync: BoolParam,

    #[id = "note_division"]
    pub note_division: EnumParam<NoteDivision>,

    #[id = "feedback"]
    pub feedback: FloatParam,

    #[id = "mix"]
    pub mix: FloatParam,

    // === Mode ===
    #[id = "mode"]
    pub mode: EnumParam<DelayMode>,

    // === Stereo ===
    #[id = "ping_pong"]
    pub ping_pong: BoolParam,

    #[id = "stereo_offset"]
    pub stereo_offset: FloatParam,

    // === Tone (Feedback Filter) ===
    #[id = "lowpass_freq"]
    pub lowpass_freq: FloatParam,

    #[id = "highpass_freq"]
    pub highpass_freq: FloatParam,

    // === Modulation (Analog mode) ===
    #[id = "mod_rate"]
    pub mod_rate: FloatParam,

    #[id = "mod_depth"]
    pub mod_depth: FloatParam,

    // === Saturation (Analog mode) ===
    #[id = "drive"]
    pub drive: FloatParam,

    // === Ducking ===
    #[id = "duck_amount"]
    pub duck_amount: FloatParam,

    #[id = "duck_threshold"]
    pub duck_threshold: FloatParam,
}

impl Default for GenXDelayParams {
    fn default() -> Self {
        Self {
            delay_time: FloatParam::new(
                "Delay Time",
                300.0,
                FloatRange::Skewed {
                    min: 1.0,
                    max: MAX_DELAY_SECONDS * 1000.0,
                    factor: FloatRange::skew_factor(-1.5),
                },
            )
            .with_unit(" ms")
            .with_value_to_string(formatters::v2s_f32_rounded(1))
            .with_smoother(SmoothingStyle::Logarithmic(100.0)),

            tempo_sync: BoolParam::new("Tempo Sync", false),

            note_division: EnumParam::new("Note Division", NoteDivision::Quarter),

            feedback: FloatParam::new("Feedback", 0.4, FloatRange::Linear { min: 0.0, max: 0.95 })
                .with_unit(" %")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage())
                .with_smoother(SmoothingStyle::Linear(50.0)),

            mix: FloatParam::new("Mix", 0.3, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_unit(" %")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage())
                .with_smoother(SmoothingStyle::Linear(50.0)),

            mode: EnumParam::new("Mode", DelayMode::Digital),

            ping_pong: BoolParam::new("Ping Pong", false),

            stereo_offset: FloatParam::new(
                "Stereo Offset",
                DEFAULT_LR_OFFSET_MS,
                FloatRange::Linear { min: 0.0, max: 50.0 },
            )
            .with_unit(" ms")
            .with_value_to_string(formatters::v2s_f32_rounded(1))
            .with_smoother(SmoothingStyle::Linear(50.0)),

            lowpass_freq: FloatParam::new(
                "Low-Pass",
                8000.0,
                FloatRange::Skewed {
                    min: 500.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-1.5),
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(1))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz())
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            highpass_freq: FloatParam::new(
                "High-Pass",
                80.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 1000.0,
                    factor: FloatRange::skew_factor(-1.5),
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(1))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz())
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            mod_rate: FloatParam::new(
                "Mod Rate",
                0.8,
                FloatRange::Skewed {
                    min: 0.1,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_smoother(SmoothingStyle::Linear(50.0)),

            mod_depth: FloatParam::new("Mod Depth", 0.3, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_unit(" %")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage())
                .with_smoother(SmoothingStyle::Linear(50.0)),

            drive: FloatParam::new("Drive", 0.2, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_unit(" %")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage())
                .with_smoother(SmoothingStyle::Linear(50.0)),

            duck_amount: FloatParam::new(
                "Duck Amount",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage())
            .with_smoother(SmoothingStyle::Linear(50.0)),

            duck_threshold: FloatParam::new(
                "Duck Threshold",
                0.3,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage())
            .with_smoother(SmoothingStyle::Linear(50.0)),
        }
    }
}

/// The GenX Delay plugin.
struct GenXDelay {
    params: Arc<GenXDelayParams>,
    sample_rate: f32,

    // Delay lines (stereo)
    delay_left: DelayLine,
    delay_right: DelayLine,

    // Feedback from previous sample (for ping-pong and feedback loop)
    feedback_left: f32,
    feedback_right: f32,

    // Feedback filters (stereo)
    filter_left: FeedbackFilter,
    filter_right: FeedbackFilter,

    // Modulation
    modulator: StereoModulator,

    // Saturation
    saturator_left: Saturator,
    saturator_right: Saturator,

    // Ducking
    ducker: Ducker,

    // Smoothers for delay time (avoid clicks)
    delay_smoother_left: OnePoleLP,
    delay_smoother_right: OnePoleLP,
}

impl Default for GenXDelay {
    fn default() -> Self {
        Self {
            params: Arc::new(GenXDelayParams::default()),
            sample_rate: 44100.0,
            delay_left: DelayLine::default(),
            delay_right: DelayLine::default(),
            feedback_left: 0.0,
            feedback_right: 0.0,
            filter_left: FeedbackFilter::default(),
            filter_right: FeedbackFilter::default(),
            modulator: StereoModulator::default(),
            saturator_left: Saturator::default(),
            saturator_right: Saturator::default(),
            ducker: Ducker::default(),
            delay_smoother_left: OnePoleLP::default(),
            delay_smoother_right: OnePoleLP::default(),
        }
    }
}

impl Plugin for GenXDelay {
    const NAME: &'static str = "GenX Delay";
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
        // Mono to Stereo
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        // Initialize delay lines
        self.delay_left.initialize(self.sample_rate, MAX_DELAY_SECONDS);
        self.delay_right.initialize(self.sample_rate, MAX_DELAY_SECONDS);

        // Initialize modulator
        self.modulator.initialize(self.sample_rate);

        // Initialize ducker
        self.ducker.initialize(self.sample_rate);

        // Initialize delay time smoothers (very smooth to avoid clicks)
        self.delay_smoother_left.set_cutoff(self.sample_rate, 10.0);
        self.delay_smoother_right.set_cutoff(self.sample_rate, 10.0);

        // Update filters
        let lp = self.params.lowpass_freq.value();
        let hp = self.params.highpass_freq.value();
        self.filter_left.update(self.sample_rate, lp, hp);
        self.filter_right.update(self.sample_rate, lp, hp);

        true
    }

    fn reset(&mut self) {
        self.delay_left.reset();
        self.delay_right.reset();
        self.feedback_left = 0.0;
        self.feedback_right = 0.0;
        self.filter_left.reset();
        self.filter_right.reset();
        self.modulator.reset();
        self.ducker.reset();
        self.delay_smoother_left.reset();
        self.delay_smoother_right.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let num_channels = buffer.channels();

        for mut channel_samples in buffer.iter_samples() {
            // Get smoothed parameter values
            let delay_time_ms = self.params.delay_time.smoothed.next();
            let feedback = self.params.feedback.smoothed.next();
            let mix = self.params.mix.smoothed.next();
            let stereo_offset_ms = self.params.stereo_offset.smoothed.next();
            let lowpass_freq = self.params.lowpass_freq.smoothed.next();
            let highpass_freq = self.params.highpass_freq.smoothed.next();
            let mod_rate = self.params.mod_rate.smoothed.next();
            let mod_depth = self.params.mod_depth.smoothed.next();
            let drive = self.params.drive.smoothed.next();
            let duck_amount = self.params.duck_amount.smoothed.next();
            let duck_threshold = self.params.duck_threshold.smoothed.next();

            // Non-smoothed params
            let tempo_sync = self.params.tempo_sync.value();
            let note_division = self.params.note_division.value();
            let mode = self.params.mode.value();
            let ping_pong = self.params.ping_pong.value();

            // Calculate delay time
            let delay_time_ms = if tempo_sync {
                // Get tempo from host
                let tempo = context.transport().tempo.unwrap_or(120.0) as f32;
                // Convert BPM to ms per beat, then multiply by note division
                let ms_per_beat = 60_000.0 / tempo;
                ms_per_beat * note_division.as_beat_multiplier()
            } else {
                delay_time_ms
            };

            // Convert to samples
            let base_delay_samples = delay_time_ms * self.sample_rate / 1000.0;
            let offset_samples = stereo_offset_ms * self.sample_rate / 1000.0;

            // Base delay times for L/R
            let base_delay_l = base_delay_samples;
            let base_delay_r = base_delay_samples + offset_samples;

            // Apply modulation in analog mode
            let (delay_samples_l, delay_samples_r) = if mode == DelayMode::Analog && mod_depth > 0.001 {
                let max_mod_samples = mod_depth * 20.0; // Up to 20 samples of modulation
                self.modulator.get_modulated_delays(
                    base_delay_l,
                    base_delay_r,
                    max_mod_samples,
                    mod_rate,
                )
            } else {
                (base_delay_l, base_delay_r)
            };

            // Smooth delay times to avoid clicks
            let smooth_delay_l = self.delay_smoother_left.process(delay_samples_l);
            let smooth_delay_r = self.delay_smoother_right.process(delay_samples_r);

            // Update filters
            self.filter_left.update(self.sample_rate, lowpass_freq, highpass_freq);
            self.filter_right.update(self.sample_rate, lowpass_freq, highpass_freq);

            // Update saturators
            let effective_drive = if mode == DelayMode::Analog { drive } else { 0.0 };
            self.saturator_left.set_drive(effective_drive);
            self.saturator_right.set_drive(effective_drive);

            // Get input samples
            let input_left = *channel_samples.get_mut(0).unwrap();
            let input_right = if num_channels >= 2 {
                *channel_samples.get_mut(1).unwrap()
            } else {
                input_left
            };

            // Calculate ducking gain
            let duck_gain = if duck_amount > 0.001 {
                self.ducker.process_stereo(input_left, input_right, duck_threshold, duck_amount)
            } else {
                1.0
            };

            // Read from delay lines
            let delayed_left = self.delay_left.read(smooth_delay_l);
            let delayed_right = self.delay_right.read(smooth_delay_r);

            // Process through feedback filters and saturation
            let filtered_left = self.filter_left.process(delayed_left);
            let filtered_right = self.filter_right.process(delayed_right);
            let saturated_left = self.saturator_left.process(filtered_left);
            let saturated_right = self.saturator_right.process(filtered_right);

            // Calculate what goes into the delay lines
            let (delay_input_left, delay_input_right) = if ping_pong {
                // Ping-pong: cross-feed the feedback
                (
                    input_left + self.feedback_right * feedback,
                    input_right + self.feedback_left * feedback,
                )
            } else {
                // Normal stereo delay
                (
                    input_left + saturated_left * feedback,
                    input_right + saturated_right * feedback,
                )
            };

            // Write to delay lines
            self.delay_left.write(delay_input_left);
            self.delay_right.write(delay_input_right);

            // Store feedback for next sample (for ping-pong)
            self.feedback_left = saturated_left;
            self.feedback_right = saturated_right;

            // Mix dry and wet with ducking
            let wet_left = saturated_left * duck_gain;
            let wet_right = saturated_right * duck_gain;

            let output_left = input_left * (1.0 - mix) + wet_left * mix;
            let output_right = input_right * (1.0 - mix) + wet_right * mix;

            // Write output
            *channel_samples.get_mut(0).unwrap() = output_left;
            if num_channels >= 2 {
                *channel_samples.get_mut(1).unwrap() = output_right;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for GenXDelay {
    const CLAP_ID: &'static str = "com.trwolf.genx-delay";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Delay emulating 00s alternative/rock tones");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Delay,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for GenXDelay {
    const VST3_CLASS_ID: [u8; 16] = *b"TrwolfGenXDelay!";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Delay];
}

nih_export_clap!(GenXDelay);
nih_export_vst3!(GenXDelay);
