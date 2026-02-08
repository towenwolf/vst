use nih_plug::prelude::*;
use std::sync::Arc;

mod filter;
use filter::FilterChain;

/// Filter slope options
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FilterSlope {
    #[id = "6db"]
    #[name = "6 dB/oct"]
    Slope6dB,
    #[id = "12db"]
    #[name = "12 dB/oct"]
    #[default]
    Slope12dB,
    #[id = "18db"]
    #[name = "18 dB/oct"]
    Slope18dB,
    #[id = "24db"]
    #[name = "24 dB/oct"]
    Slope24dB,
}

/// Plugin parameters
#[derive(Params)]
struct HighPassParams {
    #[id = "cutoff"]
    pub cutoff: FloatParam,

    #[id = "resonance"]
    pub resonance: FloatParam,

    #[id = "slope"]
    pub slope: EnumParam<FilterSlope>,
}

impl Default for HighPassParams {
    fn default() -> Self {
        Self {
            cutoff: FloatParam::new(
                "Cutoff",
                200.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz())
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            resonance: FloatParam::new(
                "Resonance",
                0.707,
                FloatRange::Linear {
                    min: 0.5,
                    max: 10.0,
                },
            )
            .with_unit(" Q")
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_smoother(SmoothingStyle::Linear(50.0)),

            slope: EnumParam::new("Slope", FilterSlope::Slope12dB),
        }
    }
}

/// High-pass filter plugin
struct HighPassFilter {
    params: Arc<HighPassParams>,
    sample_rate: f32,
    filters: [FilterChain; 2],
}

impl Default for HighPassFilter {
    fn default() -> Self {
        Self {
            params: Arc::new(HighPassParams::default()),
            sample_rate: 44100.0,
            filters: [FilterChain::default(); 2],
        }
    }
}

impl Plugin for HighPassFilter {
    const NAME: &'static str = "High-Pass Filter";
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

        let cutoff = self.params.cutoff.value();
        let resonance = self.params.resonance.value();
        let slope = self.params.slope.value();

        for filter in &mut self.filters {
            filter.update_coefficients(self.sample_rate, cutoff, resonance, to_filter_slope(slope));
        }

        true
    }

    fn reset(&mut self) {
        for filter in &mut self.filters {
            filter.reset();
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let num_channels = buffer.channels();

        for mut channel_samples in buffer.iter_samples() {
            let cutoff = self.params.cutoff.smoothed.next();
            let resonance = self.params.resonance.smoothed.next();
            let slope = self.params.slope.value();

            for i in 0..num_channels.min(2) {
                self.filters[i].update_coefficients(
                    self.sample_rate,
                    cutoff,
                    resonance,
                    to_filter_slope(slope),
                );
            }

            for (channel_idx, sample) in channel_samples.iter_mut().enumerate() {
                if channel_idx < 2 {
                    *sample = self.filters[channel_idx].process(*sample);
                }
            }
        }

        ProcessStatus::Normal
    }
}

/// Convert plugin enum to filter module enum
fn to_filter_slope(slope: FilterSlope) -> filter::FilterSlope {
    match slope {
        FilterSlope::Slope6dB => filter::FilterSlope::Slope6dB,
        FilterSlope::Slope12dB => filter::FilterSlope::Slope12dB,
        FilterSlope::Slope18dB => filter::FilterSlope::Slope18dB,
        FilterSlope::Slope24dB => filter::FilterSlope::Slope24dB,
    }
}

impl ClapPlugin for HighPassFilter {
    const CLAP_ID: &'static str = "com.trwolf.highpass-filter";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A variable-slope high-pass filter");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Filter,
        ClapFeature::Stereo,
        ClapFeature::Mono,
    ];
}

impl Vst3Plugin for HighPassFilter {
    const VST3_CLASS_ID: [u8; 16] = *b"TrwolfHPFilterPl";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Filter];
}

nih_export_clap!(HighPassFilter);
nih_export_vst3!(HighPassFilter);
