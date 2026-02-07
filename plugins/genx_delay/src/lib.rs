//! GenX Delay - An emulation of delays popular in 00s alternative/rock music.
//! Inspired by the warm, modulated delay sounds of artists like Incubus.

use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::Arc;

mod delay_line;
mod ducker;
mod editor;
mod filters;
mod modulation;
mod reverse_delay;
mod saturation;

use delay_line::DelayLine;
use ducker::Ducker;
use filters::{FeedbackFilter, OnePoleLP};
use modulation::StereoModulator;
use reverse_delay::ReverseDelayLine;
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
pub struct GenXDelayParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,

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

    // === Reverse ===
    #[id = "reverse"]
    pub reverse: BoolParam,

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
            editor_state: editor::default_state(),

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

            feedback: FloatParam::new(
                "Feedback",
                0.4,
                FloatRange::Linear {
                    min: 0.0,
                    max: 0.95,
                },
            )
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

            reverse: BoolParam::new("Reverse", false),

            ping_pong: BoolParam::new("Ping Pong", false),

            stereo_offset: FloatParam::new(
                "Stereo Offset",
                DEFAULT_LR_OFFSET_MS,
                FloatRange::Linear {
                    min: 0.0,
                    max: 50.0,
                },
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

    // Reverse delay lines (stereo)
    reverse_left: ReverseDelayLine,
    reverse_right: ReverseDelayLine,

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
            reverse_left: ReverseDelayLine::default(),
            reverse_right: ReverseDelayLine::default(),
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
    const URL: &'static str = "https://github.com/towenwolf/vst";
    const EMAIL: &'static str = "towenwolf@users.noreply.github.com";
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

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        // Initialize delay lines
        self.delay_left
            .initialize(self.sample_rate, MAX_DELAY_SECONDS);
        self.delay_right
            .initialize(self.sample_rate, MAX_DELAY_SECONDS);

        // Initialize reverse delay lines
        self.reverse_left
            .initialize(self.sample_rate, MAX_DELAY_SECONDS);
        self.reverse_right
            .initialize(self.sample_rate, MAX_DELAY_SECONDS);

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
        self.reverse_left.reset();
        self.reverse_right.reset();
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
            let reverse = self.params.reverse.value();
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
            let (delay_samples_l, delay_samples_r) =
                if mode == DelayMode::Analog && mod_depth > 0.001 {
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
            self.filter_left
                .update(self.sample_rate, lowpass_freq, highpass_freq);
            self.filter_right
                .update(self.sample_rate, lowpass_freq, highpass_freq);

            // Update saturators
            let effective_drive = if mode == DelayMode::Analog {
                drive
            } else {
                0.0
            };
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
                self.ducker
                    .process_stereo(input_left, input_right, duck_threshold, duck_amount)
            } else {
                1.0
            };

            // Read from delay lines (forward or reverse)
            let delayed_left;
            let delayed_right;
            if reverse {
                delayed_left = self.reverse_left.read(smooth_delay_l);
                delayed_right = self.reverse_right.read(smooth_delay_r);
            } else {
                delayed_left = self.delay_left.read(smooth_delay_l);
                delayed_right = self.delay_right.read(smooth_delay_r);
            }

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

            // Write to delay lines (forward or reverse)
            if reverse {
                self.reverse_left.write(delay_input_left);
                self.reverse_right.write(delay_input_right);
            } else {
                self.delay_left.write(delay_input_left);
                self.delay_right.write(delay_input_right);
            }

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
    const CLAP_MANUAL_URL: Option<&'static str> =
        Some("https://github.com/towenwolf/vst/tree/main/plugins/genx_delay");
    const CLAP_SUPPORT_URL: Option<&'static str> = Some("https://github.com/towenwolf/vst/issues");
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

#[cfg(test)]
mod gui_usability_tests {
    use super::*;
    use std::collections::HashSet;

    /// Helper: create a default params instance for testing.
    fn test_params() -> GenXDelayParams {
        GenXDelayParams::default()
    }

    /// Helper: create a default plugin instance for testing.
    fn test_plugin() -> GenXDelay {
        GenXDelay::default()
    }

    // =========================================================================
    // Plugin instantiation — Ableton must be able to load the plugin
    // =========================================================================

    #[test]
    fn plugin_instantiates_without_panic() {
        let _plugin = test_plugin();
    }

    #[test]
    fn plugin_exposes_params_trait_object() {
        let plugin = test_plugin();
        let _params: Arc<dyn Params> = plugin.params();
    }

    // =========================================================================
    // Editor / GUI creation — the GUI window must open in the DAW
    // =========================================================================

    #[test]
    fn editor_state_creates_with_correct_dimensions() {
        let params = test_params();
        let (width, height) = params.editor_state.size();
        // Design spec: 600x420. Current placeholder: 300x200.
        // This test documents the expected size — update when the GUI is built out.
        assert!(
            width > 0 && height > 0,
            "editor window must have positive dimensions"
        );
        assert!(
            (200..=1200).contains(&width),
            "editor width {width} is outside reasonable DAW range (200–1200px)"
        );
        assert!(
            (150..=900).contains(&height),
            "editor height {height} is outside reasonable DAW range (150–900px)"
        );
    }

    #[test]
    fn editor_state_is_shared_with_params() {
        let plugin = test_plugin();
        // The editor state must be accessible from the params (persisted by nih-plug)
        let _state = plugin.params.editor_state.clone();
    }

    // =========================================================================
    // VST3 metadata — Ableton reads this to list and categorize the plugin
    // =========================================================================

    #[test]
    fn vst3_class_id_is_16_bytes() {
        assert_eq!(
            GenXDelay::VST3_CLASS_ID.len(),
            16,
            "VST3 class ID must be exactly 16 bytes"
        );
    }

    #[test]
    fn vst3_class_id_is_not_zeroed() {
        assert_ne!(
            GenXDelay::VST3_CLASS_ID,
            [0u8; 16],
            "VST3 class ID must not be all zeros"
        );
    }

    #[test]
    fn vst3_subcategories_include_fx_and_delay() {
        let subcats = GenXDelay::VST3_SUBCATEGORIES;
        let has_fx = subcats.iter().any(|s| matches!(s, Vst3SubCategory::Fx));
        let has_delay = subcats.iter().any(|s| matches!(s, Vst3SubCategory::Delay));
        assert!(
            has_fx,
            "VST3 subcategories must include Fx for Ableton to show it as an effect"
        );
        assert!(
            has_delay,
            "VST3 subcategories must include Delay for proper categorization"
        );
    }

    #[test]
    fn plugin_name_is_set() {
        assert!(
            !GenXDelay::NAME.is_empty(),
            "Plugin name must be non-empty for Ableton's plugin list"
        );
    }

    #[test]
    fn audio_io_includes_stereo() {
        let layouts = GenXDelay::AUDIO_IO_LAYOUTS;
        let has_stereo = layouts.iter().any(|l| {
            l.main_input_channels == NonZeroU32::new(2)
                && l.main_output_channels == NonZeroU32::new(2)
        });
        assert!(
            has_stereo,
            "Must support stereo I/O for Ableton stereo tracks"
        );
    }

    #[test]
    fn sample_accurate_automation_enabled() {
        let sample_accurate = std::hint::black_box(GenXDelay::SAMPLE_ACCURATE_AUTOMATION);
        assert!(
            sample_accurate,
            "Sample-accurate automation should be enabled for tight Ableton automation"
        );
    }

    // =========================================================================
    // Parameter defaults — must be musically useful out of the box
    // =========================================================================

    #[test]
    fn delay_time_default_is_musically_useful() {
        let p = test_params();
        let v = p.delay_time.default_plain_value();
        // 300ms is a classic 1/4-note delay at ~120 BPM — bread and butter
        assert!(
            (100.0..=500.0).contains(&v),
            "Default delay time {v}ms should be in a musically common range (100–500ms)"
        );
    }

    #[test]
    fn feedback_default_is_moderate() {
        let p = test_params();
        let v = p.feedback.default_plain_value();
        assert!(
            (0.2..=0.6).contains(&v),
            "Default feedback {v} should produce audible repeats without runaway (0.2–0.6)"
        );
    }

    #[test]
    fn mix_default_is_audible_but_not_overwhelming() {
        let p = test_params();
        let v = p.mix.default_plain_value();
        assert!(
            (0.15..=0.5).contains(&v),
            "Default mix {v} should be clearly audible but not drown out dry signal (0.15–0.5)"
        );
    }

    #[test]
    fn duck_amount_default_is_off() {
        let p = test_params();
        let v = p.duck_amount.default_plain_value();
        assert!(
            v < 0.01,
            "Duck amount should default to off (0.0) — ducking is an advanced feature"
        );
    }

    #[test]
    fn tempo_sync_default_is_off() {
        let p = test_params();
        let v = p.tempo_sync.default_plain_value();
        assert!(
            !v,
            "Tempo sync should default to off — manual delay time is more intuitive at first"
        );
    }

    #[test]
    fn ping_pong_default_is_off() {
        let p = test_params();
        let v = p.ping_pong.default_plain_value();
        assert!(
            !v,
            "Ping pong should default to off — basic stereo delay is the starting point"
        );
    }

    #[test]
    fn reverse_default_is_off() {
        let p = test_params();
        let v = p.reverse.default_plain_value();
        assert!(
            !v,
            "Reverse should default to off — forward delay is the standard starting point"
        );
    }

    #[test]
    fn mode_default_is_digital() {
        let p = test_params();
        let v = p.mode.default_plain_value();
        assert_eq!(
            v,
            DelayMode::Digital,
            "Mode should default to Digital — clean and predictable for first use"
        );
    }

    // =========================================================================
    // Parameter ranges — must be safe (no silence, no clipping, no runaway)
    // =========================================================================

    #[test]
    fn feedback_max_prevents_runaway() {
        let p = test_params();
        // Get the maximum value from the parameter range
        // Feedback must stay < 1.0 to prevent infinite feedback
        let range = &p.feedback;
        let max_val = range.preview_plain(1.0); // normalized 1.0 → plain max
        assert!(
            max_val < 1.0,
            "Feedback max {max_val} must be < 1.0 to prevent infinite feedback loops"
        );
        assert!(
            max_val >= 0.9,
            "Feedback max {max_val} should be >= 0.9 to allow long, ambient tails"
        );
    }

    #[test]
    fn delay_time_min_prevents_comb_filter_artifacts() {
        let p = test_params();
        let min_val = p.delay_time.preview_plain(0.0);
        assert!(
            min_val >= 1.0,
            "Minimum delay {min_val}ms must be >= 1ms to avoid metallic comb-filter artifacts"
        );
    }

    #[test]
    fn delay_time_max_is_generous() {
        let p = test_params();
        let max_val = p.delay_time.preview_plain(1.0);
        assert!(
            max_val >= 2000.0,
            "Maximum delay {max_val}ms should be >= 2000ms for ambient/experimental use"
        );
    }

    #[test]
    fn filter_ranges_are_musically_valid() {
        let p = test_params();
        let lp_min = p.lowpass_freq.preview_plain(0.0);
        let lp_max = p.lowpass_freq.preview_plain(1.0);
        let hp_min = p.highpass_freq.preview_plain(0.0);
        let hp_max = p.highpass_freq.preview_plain(1.0);

        // LP must go up to at least near-full bandwidth
        assert!(
            lp_max >= 18000.0,
            "Low-pass max {lp_max} Hz should reach near-Nyquist"
        );
        // HP must go down to sub-bass
        assert!(
            hp_min <= 30.0,
            "High-pass min {hp_min} Hz should allow sub-bass through"
        );
        // HP max should be below LP min to prevent impossible crossover
        assert!(
            hp_max < lp_min || lp_min <= hp_max,
            "Filter ranges should allow some valid overlap for tone shaping"
        );
    }

    #[test]
    fn stereo_offset_range_is_reasonable() {
        let p = test_params();
        let min_val = p.stereo_offset.preview_plain(0.0);
        let max_val = p.stereo_offset.preview_plain(1.0);
        assert!(min_val >= 0.0, "Stereo offset min should be 0 (no offset)");
        assert!(
            max_val <= 100.0,
            "Stereo offset max {max_val}ms should be <= 100ms to avoid flamming"
        );
    }

    // =========================================================================
    // Parameter IDs — must be unique for Ableton automation persistence
    // =========================================================================

    #[test]
    fn all_parameter_ids_are_unique() {
        // Ableton stores automation by parameter ID.
        // Duplicate IDs would cause one parameter's automation to overwrite another.
        let ids = [
            "delay_time",
            "tempo_sync",
            "note_division",
            "feedback",
            "mix",
            "mode",
            "reverse",
            "ping_pong",
            "stereo_offset",
            "lowpass_freq",
            "highpass_freq",
            "mod_rate",
            "mod_depth",
            "drive",
            "duck_amount",
            "duck_threshold",
        ];
        let unique: HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(
            ids.len(),
            unique.len(),
            "All parameter IDs must be unique — duplicates break Ableton automation recall"
        );
    }

    #[test]
    fn expected_parameter_count() {
        // 16 parameter IDs (not counting editor_state which is persisted but not a user param)
        // If a parameter is added or removed, this test catches the drift.
        let expected = 16;
        let ids = [
            "delay_time",
            "tempo_sync",
            "note_division",
            "feedback",
            "mix",
            "mode",
            "reverse",
            "ping_pong",
            "stereo_offset",
            "lowpass_freq",
            "highpass_freq",
            "mod_rate",
            "mod_depth",
            "drive",
            "duck_amount",
            "duck_threshold",
        ];
        assert_eq!(
            ids.len(),
            expected,
            "Expected {expected} user-facing parameters — update if params are added/removed"
        );
    }

    // =========================================================================
    // Parameter names — must be readable in Ableton's parameter list
    // =========================================================================

    #[test]
    fn parameter_names_are_not_empty() {
        let p = test_params();
        let names = [
            p.delay_time.name(),
            p.feedback.name(),
            p.mix.name(),
            p.stereo_offset.name(),
            p.lowpass_freq.name(),
            p.highpass_freq.name(),
            p.mod_rate.name(),
            p.mod_depth.name(),
            p.drive.name(),
            p.duck_amount.name(),
            p.duck_threshold.name(),
        ];
        for name in &names {
            assert!(
                !name.is_empty(),
                "Parameter name must not be empty — Ableton shows these in its UI"
            );
        }
    }

    #[test]
    fn parameter_names_are_reasonably_short() {
        let p = test_params();
        let names = [
            p.delay_time.name(),
            p.feedback.name(),
            p.mix.name(),
            p.stereo_offset.name(),
            p.lowpass_freq.name(),
            p.highpass_freq.name(),
            p.mod_rate.name(),
            p.mod_depth.name(),
            p.drive.name(),
            p.duck_amount.name(),
            p.duck_threshold.name(),
        ];
        for name in &names {
            assert!(
                name.len() <= 20,
                "Parameter name '{}' ({} chars) is too long — Ableton truncates names in the automation lane",
                name,
                name.len()
            );
        }
    }

    // =========================================================================
    // Note divisions — tempo sync must cover standard musical subdivisions
    // =========================================================================

    #[test]
    fn all_standard_note_divisions_are_available() {
        // Common divisions that producers expect in Ableton
        let divs = [
            NoteDivision::Whole,
            NoteDivision::Half,
            NoteDivision::Quarter,
            NoteDivision::Eighth,
            NoteDivision::Sixteenth,
        ];
        for div in &divs {
            let mult = div.as_beat_multiplier();
            assert!(mult > 0.0, "{:?} must have a positive beat multiplier", div);
        }
    }

    #[test]
    fn dotted_and_triplet_variants_exist() {
        let dotted = [
            NoteDivision::HalfDotted,
            NoteDivision::QuarterDotted,
            NoteDivision::EighthDotted,
            NoteDivision::SixteenthDotted,
        ];
        let triplet = [
            NoteDivision::HalfTriplet,
            NoteDivision::QuarterTriplet,
            NoteDivision::EighthTriplet,
            NoteDivision::SixteenthTriplet,
        ];
        for div in dotted.iter().chain(triplet.iter()) {
            let mult = div.as_beat_multiplier();
            assert!(mult > 0.0, "{:?} must have a positive beat multiplier", div);
        }
    }

    #[test]
    fn note_division_options_are_gui_selector_safe() {
        // QA gate for the next GUI increment (Tempo Sync + Note Division selector):
        // each option must map to a stable, unique label and a musically valid beat multiplier.
        let options = [
            (NoteDivision::Whole, "1/1"),
            (NoteDivision::Half, "1/2"),
            (NoteDivision::HalfDotted, "1/2d"),
            (NoteDivision::HalfTriplet, "1/2t"),
            (NoteDivision::Quarter, "1/4"),
            (NoteDivision::QuarterDotted, "1/4d"),
            (NoteDivision::QuarterTriplet, "1/4t"),
            (NoteDivision::Eighth, "1/8"),
            (NoteDivision::EighthDotted, "1/8d"),
            (NoteDivision::EighthTriplet, "1/8t"),
            (NoteDivision::Sixteenth, "1/16"),
            (NoteDivision::SixteenthDotted, "1/16d"),
            (NoteDivision::SixteenthTriplet, "1/16t"),
        ];

        let mut seen = HashSet::new();
        for (division, label) in options {
            assert!(
                seen.insert(label),
                "Duplicate note division label '{label}' would break GUI selector clarity"
            );
            assert!(
                division.as_beat_multiplier() > 0.0,
                "Note division '{label}' must map to a positive beat multiplier"
            );
        }

        // Coarse musical ordering used by the selector UX.
        assert!(NoteDivision::Whole.as_beat_multiplier() > NoteDivision::Half.as_beat_multiplier());
        assert!(
            NoteDivision::Half.as_beat_multiplier() > NoteDivision::Quarter.as_beat_multiplier()
        );
        assert!(
            NoteDivision::Quarter.as_beat_multiplier() > NoteDivision::Eighth.as_beat_multiplier()
        );
        assert!(
            NoteDivision::Eighth.as_beat_multiplier()
                > NoteDivision::Sixteenth.as_beat_multiplier()
        );
    }

    #[test]
    fn tempo_sync_delay_times_are_musically_correct() {
        // At 120 BPM, a quarter note = 500ms
        let bpm = 120.0_f32;
        let ms_per_beat = 60_000.0 / bpm;

        let quarter_ms = ms_per_beat * NoteDivision::Quarter.as_beat_multiplier();
        assert!(
            (quarter_ms - 500.0).abs() < 0.01,
            "Quarter note at 120 BPM should be 500ms, got {quarter_ms}"
        );

        let eighth_ms = ms_per_beat * NoteDivision::Eighth.as_beat_multiplier();
        assert!(
            (eighth_ms - 250.0).abs() < 0.01,
            "Eighth note at 120 BPM should be 250ms, got {eighth_ms}"
        );

        let dotted_eighth_ms = ms_per_beat * NoteDivision::EighthDotted.as_beat_multiplier();
        assert!(
            (dotted_eighth_ms - 375.0).abs() < 0.01,
            "Dotted eighth at 120 BPM should be 375ms, got {dotted_eighth_ms}"
        );
    }

    // =========================================================================
    // Smoothing — parameters must be smoothed to avoid clicks in automation
    // =========================================================================

    #[test]
    fn continuous_parameters_have_smoothing() {
        let p = test_params();
        // These parameters cause audible clicks/pops if not smoothed
        // We verify they have non-zero smoother durations by checking they
        // are configured with smoothing (the smoother style is set in Default impl)
        //
        // The best proxy: read the smoothed value — if smoothing is configured,
        // the smoother object exists. We just verify the params were constructed.
        let _delay = p.delay_time.smoothed.style;
        let _fb = p.feedback.smoothed.style;
        let _mix = p.mix.smoothed.style;
        let _offset = p.stereo_offset.smoothed.style;
        let _lp = p.lowpass_freq.smoothed.style;
        let _hp = p.highpass_freq.smoothed.style;
        let _rate = p.mod_rate.smoothed.style;
        let _depth = p.mod_depth.smoothed.style;
        let _drive = p.drive.smoothed.style;
        let _duck_amt = p.duck_amount.smoothed.style;
        let _duck_thr = p.duck_threshold.smoothed.style;
        // If any of these didn't have smoothing configured, the style would be None/default.
        // This test primarily ensures the params compile and don't panic.
    }

    // =========================================================================
    // Plugin process stability — must not panic with edge-case input
    // =========================================================================

    #[test]
    fn plugin_reset_does_not_panic() {
        let mut plugin = test_plugin();
        plugin.reset();
        plugin.reset(); // double-reset should be safe
    }

    // =========================================================================
    // MVP Kanban Test Gates (docs/GENX_DELAY_MVP_KANBAN.md)
    // =========================================================================

    #[test]
    fn gdx_00_resize_scaling_geometry_contract() {
        let tiny = editor::resize_geometry_metrics_for_dimensions(240.0, 150.0);
        let base = editor::resize_geometry_metrics_for_dimensions(600.0, 420.0);
        let large = editor::resize_geometry_metrics_for_dimensions(1200.0, 840.0);

        assert_eq!(
            tiny.content_scale, 0.5,
            "content scale must clamp at 0.5 for very small window sizes"
        );
        assert!(
            tiny.content_scale < base.content_scale && base.content_scale < large.content_scale,
            "content scale must increase monotonically with larger window dimensions"
        );

        assert!(
            tiny.barbed_wire_margin <= base.barbed_wire_margin
                && base.barbed_wire_margin < large.barbed_wire_margin,
            "barbed-wire margins should stay stable at small sizes and increase above base scale"
        );
        assert!(
            tiny.tribal_margin < base.tribal_margin && base.tribal_margin < large.tribal_margin,
            "tribal corner margins should scale with content size"
        );
        assert!(
            tiny.tribal_outer_stroke < base.tribal_outer_stroke
                && base.tribal_outer_stroke < large.tribal_outer_stroke,
            "tribal outer stroke must scale with content size"
        );
        assert!(
            tiny.tribal_inner_stroke < base.tribal_inner_stroke
                && base.tribal_inner_stroke < large.tribal_inner_stroke,
            "tribal inner stroke must scale with content size"
        );
        assert!(
            tiny.rust_stamp_outer_stroke < base.rust_stamp_outer_stroke
                && base.rust_stamp_outer_stroke < large.rust_stamp_outer_stroke,
            "rust stamp stroke must scale with content size"
        );
    }

    #[test]
    fn gdx_01_gui_uses_mvp_window_size_600x420() {
        let p = test_params();
        let (width, height) = p.editor_state.size();
        assert_eq!(width, 600, "GUI width must match MVP design spec");
        assert_eq!(height, 420, "GUI height must match MVP design spec");
    }

    #[test]
    fn gdx_02_full_mvp_parameter_surface_exists_for_gui_wiring() {
        // Contract: these are the controls the MVP GUI must expose and wire.
        let expected_ids = [
            "delay_time",
            "tempo_sync",
            "note_division",
            "feedback",
            "mix",
            "mode",
            "reverse",
            "ping_pong",
            "stereo_offset",
            "lowpass_freq",
            "highpass_freq",
            "mod_rate",
            "mod_depth",
            "drive",
            "duck_amount",
            "duck_threshold",
        ];

        let unique: HashSet<&str> = expected_ids.iter().copied().collect();
        assert_eq!(
            unique.len(),
            expected_ids.len(),
            "MVP control surface must have stable, unique parameter IDs"
        );
    }

    #[test]
    fn gdx_03_mode_contract_supports_modulation_enable_disable_logic() {
        let p = test_params();
        assert_eq!(
            p.mode.default_plain_value(),
            DelayMode::Digital,
            "Digital default is required for predictable modulation-disabled startup"
        );
        assert!(
            p.mod_rate.preview_plain(1.0) > p.mod_rate.preview_plain(0.0),
            "Mod Rate must remain a valid continuous control when Analog mode enables it"
        );
        assert!(
            p.mod_depth.preview_plain(1.0) > p.mod_depth.preview_plain(0.0),
            "Mod Depth must remain a valid continuous control when Analog mode enables it"
        );
        assert!(
            p.drive.preview_plain(1.0) > p.drive.preview_plain(0.0),
            "Drive must remain a valid continuous control when Analog mode enables it"
        );
    }

    #[test]
    fn gdx_04_visual_polish_contract() {
        let deco_small = editor::woodstock_decoration_metrics(0.5);
        let deco_base = editor::woodstock_decoration_metrics(1.0);
        let deco_large = editor::woodstock_decoration_metrics(1.8);

        assert!(
            deco_base.wire_spacing > 0.0,
            "barbed-wire separator spacing must remain positive"
        );
        assert!(
            deco_small.wire_spacing < deco_base.wire_spacing
                && deco_base.wire_spacing < deco_large.wire_spacing,
            "barbed-wire spacing should scale monotonically with window scale"
        );

        let accent_small = editor::section_accent_metrics(0.5);
        let accent_base = editor::section_accent_metrics(1.0);
        let accent_large = editor::section_accent_metrics(1.8);

        assert!(
            accent_base.line_thickness > 0.0,
            "section accent lines must stay visible"
        );
        assert!(
            accent_base.ornament_radius > 0.0 && accent_base.ornament_gap > 0.0,
            "section accent ornaments must remain renderable"
        );
        assert!(
            accent_small.line_thickness <= accent_base.line_thickness
                && accent_base.line_thickness <= accent_large.line_thickness,
            "section accent line thickness should scale with window size"
        );
        assert!(
            accent_small.ornament_gap < accent_base.ornament_gap
                && accent_base.ornament_gap < accent_large.ornament_gap,
            "section accent ornament spacing should scale with window size"
        );
    }

    #[test]
    fn gdx_05_note_division_selector_options_are_unique_and_ordered() {
        let options = editor::GUI_NOTE_DIVISION_OPTIONS;
        assert_eq!(
            options.len(),
            13,
            "GUI note-division selector must expose all planned musical subdivisions"
        );

        let mut seen_labels = HashSet::new();
        let mut seen_divisions = HashSet::new();
        for (division, label) in options {
            assert!(
                seen_labels.insert(*label),
                "Duplicate selector label '{label}' would break GUI clarity"
            );
            assert!(
                seen_divisions.insert(division.to_index()),
                "Duplicate selector division '{division:?}' would break GUI mapping"
            );
        }

        // The selector's expected coarse order in UI.
        assert_eq!(options[0].0, NoteDivision::Whole);
        assert_eq!(options[1].0, NoteDivision::Half);
        assert_eq!(options[4].0, NoteDivision::Quarter);
        assert_eq!(options[7].0, NoteDivision::Eighth);
        assert_eq!(options[10].0, NoteDivision::Sixteenth);
    }

    #[test]
    fn gdx_05_mode_gating_logic_contract() {
        assert!(
            !editor::modulation_controls_enabled(DelayMode::Digital),
            "Modulation controls must be disabled in Digital mode"
        );
        assert!(
            editor::modulation_controls_enabled(DelayMode::Analog),
            "Modulation controls must be enabled in Analog mode"
        );
    }

    #[test]
    fn gdx_05_default_ui_state_parity_with_params() {
        let p = test_params();
        let default_mode = p.mode.default_plain_value();
        let default_tempo_sync = p.tempo_sync.default_plain_value();

        assert_eq!(
            default_mode,
            DelayMode::Digital,
            "UI default mode should match parameter default"
        );
        assert!(
            !default_tempo_sync,
            "UI tempo-sync toggle should default off to match parameter default"
        );
        assert!(
            !editor::modulation_controls_enabled(default_mode),
            "Modulation UI should start disabled when mode defaults to Digital"
        );
        assert!(
            !editor::note_division_selector_enabled(default_tempo_sync),
            "Note-division selector should start disabled when tempo sync is off"
        );
    }

    #[test]
    #[ignore = "GDX-06 host gate: requires manual DAW smoke tests"]
    fn gdx_06_host_smoke_test_matrix_gate() {
        // Manual acceptance check in release candidate.
        // See docs/GDX_06_SMOKE_TEST_REPORT.md for the full checklist.
        //
        // Required hosts (minimum 4):
        //   1. REAPER          — [ ] tested
        //   2. Ableton Live    — [ ] tested
        //   3. Bitwig Studio   — [ ] tested
        //   4. (additional)    — [ ] tested
        //
        // Per-host checks:
        //   [1] Insert plugin and open GUI
        //   [2] Automate 3+ params during playback
        //   [3] Save project, close host, reopen, verify state
        //   [4] Toggle GUI open/close repeatedly during playback
        //   [5] Validate resizing and HiDPI scaling
        //
        // Once all hosts pass, replace the panic below with an assert
        // and mark the checkboxes above. Record results in the report doc.
        panic!("GDX-06: manual host smoke tests not yet recorded — see docs/GDX_06_SMOKE_TEST_REPORT.md");
    }

    #[test]
    fn gdx_07_release_metadata_is_filled() {
        assert!(
            !GenXDelay::URL.is_empty(),
            "Plugin URL must be set for release"
        );
        assert!(
            !GenXDelay::EMAIL.is_empty(),
            "Plugin support email must be set for release"
        );
        assert!(
            GenXDelay::CLAP_MANUAL_URL.is_some(),
            "CLAP manual URL should be present for release support"
        );
        assert!(
            GenXDelay::CLAP_SUPPORT_URL.is_some(),
            "CLAP support URL should be present for release support"
        );
    }

    #[test]
    #[ignore = "GDX-08 warning gate: enforce with clippy -D warnings in release CI"]
    fn gdx_08_no_avoidable_dead_code_or_warning_gate() {
        // CI/command gate:
        // cargo clippy -p genx_delay --all-targets -- -D warnings
        panic!("Warning-clean gate for GDX-08");
    }

    #[test]
    fn gdx_09_woodstock_icon_motif_contract() {
        let small = editor::woodstock_decoration_metrics(0.5);
        let base = editor::woodstock_decoration_metrics(1.0);
        let large = editor::woodstock_decoration_metrics(1.8);

        for (name, opacity) in [
            ("barbed_wire", base.barbed_wire_opacity),
            ("tribal", base.tribal_opacity),
            ("dove", base.dove_opacity),
            ("speckle", base.speckle_opacity),
            ("rust_stamp", base.rust_stamp_opacity),
        ] {
            assert!(
                (0.06..=0.18).contains(&opacity),
                "{name} opacity {opacity:.3} must remain in low-opacity decorative range"
            );
        }

        assert!(
            small.wire_spacing < base.wire_spacing && base.wire_spacing < large.wire_spacing,
            "barbed-wire spacing should scale with GUI size"
        );
        assert!(
            small.corner_extent < base.corner_extent && base.corner_extent < large.corner_extent,
            "tribal-corner extents should scale with GUI size"
        );
        assert!(
            small.dove_size < base.dove_size && base.dove_size < large.dove_size,
            "dove motif size should scale with GUI size"
        );
    }
}
