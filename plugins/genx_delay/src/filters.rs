//! Filter implementations for the delay effect.
//! Includes low-pass and high-pass filters for shaping the delayed signal.

use std::f32::consts::PI;

/// Simple one-pole low-pass filter for smooth parameter changes and tone shaping.
#[derive(Clone, Copy, Default)]
pub struct OnePoleLP {
    coeff: f32,
    state: f32,
}

impl OnePoleLP {
    /// Set the cutoff frequency.
    pub fn set_cutoff(&mut self, sample_rate: f32, freq: f32) {
        let freq = freq.clamp(20.0, sample_rate * 0.49);
        let w = 2.0 * PI * freq / sample_rate;
        self.coeff = 1.0 - (-w).exp();
    }

    /// Process a single sample.
    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        self.state += self.coeff * (input - self.state);
        self.state
    }

    /// Reset the filter state.
    pub fn reset(&mut self) {
        self.state = 0.0;
    }
}

/// Biquad filter for more precise filtering in the feedback loop.
#[derive(Clone, Copy, Default)]
pub struct Biquad {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    z1: f32,
    z2: f32,
}

impl Biquad {
    /// Set as low-pass filter using RBJ cookbook.
    pub fn set_lowpass(&mut self, sample_rate: f32, freq: f32, q: f32) {
        let freq = freq.clamp(20.0, sample_rate * 0.49);
        let w0 = 2.0 * PI * freq / sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * q);

        let b0 = (1.0 - cos_w0) / 2.0;
        let b1 = 1.0 - cos_w0;
        let b2 = (1.0 - cos_w0) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }

    /// Set as high-pass filter using RBJ cookbook.
    pub fn set_highpass(&mut self, sample_rate: f32, freq: f32, q: f32) {
        let freq = freq.clamp(20.0, sample_rate * 0.49);
        let w0 = 2.0 * PI * freq / sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * q);

        let b0 = (1.0 + cos_w0) / 2.0;
        let b1 = -(1.0 + cos_w0);
        let b2 = (1.0 + cos_w0) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }

    /// Process a single sample.
    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.z1;
        self.z1 = self.b1 * input - self.a1 * output + self.z2;
        self.z2 = self.b2 * input - self.a2 * output;
        output
    }

    /// Reset the filter state.
    pub fn reset(&mut self) {
        self.z1 = 0.0;
        self.z2 = 0.0;
    }
}

/// Feedback filter chain: high-pass to remove rumble, low-pass for warmth.
#[derive(Clone, Copy, Default)]
pub struct FeedbackFilter {
    highpass: Biquad,
    lowpass: Biquad,
}

impl FeedbackFilter {
    /// Update filter coefficients.
    /// `lowpass_freq`: the high-frequency rolloff (e.g., 3000-12000 Hz)
    /// `highpass_freq`: the low-frequency rolloff (e.g., 80-200 Hz)
    pub fn update(&mut self, sample_rate: f32, lowpass_freq: f32, highpass_freq: f32) {
        self.lowpass.set_lowpass(sample_rate, lowpass_freq, 0.707);
        self.highpass.set_highpass(sample_rate, highpass_freq, 0.707);
    }

    /// Process a sample through both filters.
    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let hp_out = self.highpass.process(input);
        self.lowpass.process(hp_out)
    }

    /// Reset both filters.
    pub fn reset(&mut self) {
        self.highpass.reset();
        self.lowpass.reset();
    }
}
