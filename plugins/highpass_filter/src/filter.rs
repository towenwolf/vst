use std::f32::consts::PI;

/// Filter slope options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FilterSlope {
    Slope6dB,
    #[default]
    Slope12dB,
    Slope18dB,
    Slope24dB,
}

/// Biquad filter section using Direct Form 2 Transposed
#[derive(Clone, Copy, Default)]
pub struct BiquadState {
    // Normalized coefficients
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    // Filter state
    z1: f32,
    z2: f32,
}

impl BiquadState {
    /// Calculate high-pass filter coefficients using RBJ cookbook
    pub fn set_highpass(&mut self, sample_rate: f32, freq: f32, q: f32) {
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

        // Normalize by a0
        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }

    /// Process single sample
    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.z1;
        self.z1 = self.b1 * input - self.a1 * output + self.z2;
        self.z2 = self.b2 * input - self.a2 * output;
        output
    }

    /// Reset filter state
    pub fn reset(&mut self) {
        self.z1 = 0.0;
        self.z2 = 0.0;
    }
}

/// First-order high-pass filter for 6dB and 18dB slopes
#[derive(Clone, Copy, Default)]
pub struct FirstOrderHPState {
    b0: f32,
    b1: f32,
    a1: f32,
    x1: f32,
    y1: f32,
}

impl FirstOrderHPState {
    /// Set first-order high-pass coefficients
    pub fn set_highpass(&mut self, sample_rate: f32, freq: f32) {
        let wc = 2.0 * PI * freq;
        let t = 1.0 / sample_rate;

        // Pre-warped frequency via bilinear transform
        let wa = (2.0 / t) * (wc * t / 2.0).tan();
        let g = wa * t / 2.0;

        self.b0 = 1.0 / (1.0 + g);
        self.b1 = -self.b0;
        self.a1 = (g - 1.0) / (g + 1.0);
    }

    /// Process single sample
    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 - self.a1 * self.y1;
        self.x1 = input;
        self.y1 = output;
        output
    }

    /// Reset filter state
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}

/// Complete filter chain for one channel
#[derive(Clone, Copy, Default)]
pub struct FilterChain {
    biquad_stages: [BiquadState; 2],
    first_order: FirstOrderHPState,
    active_biquads: usize,
    use_first_order: bool,
}

impl FilterChain {
    /// Update filter coefficients based on current parameters
    pub fn update_coefficients(&mut self, sample_rate: f32, freq: f32, q: f32, slope: FilterSlope) {
        match slope {
            FilterSlope::Slope6dB => {
                self.first_order.set_highpass(sample_rate, freq);
                self.active_biquads = 0;
                self.use_first_order = true;
            }
            FilterSlope::Slope12dB => {
                self.biquad_stages[0].set_highpass(sample_rate, freq, q);
                self.active_biquads = 1;
                self.use_first_order = false;
            }
            FilterSlope::Slope18dB => {
                self.biquad_stages[0].set_highpass(sample_rate, freq, q);
                self.first_order.set_highpass(sample_rate, freq);
                self.active_biquads = 1;
                self.use_first_order = true;
            }
            FilterSlope::Slope24dB => {
                self.biquad_stages[0].set_highpass(sample_rate, freq, q);
                self.biquad_stages[1].set_highpass(sample_rate, freq, q);
                self.active_biquads = 2;
                self.use_first_order = false;
            }
        }
    }

    /// Process single sample through the filter chain
    #[inline]
    pub fn process(&mut self, mut sample: f32) -> f32 {
        if self.use_first_order {
            sample = self.first_order.process(sample);
        }

        for i in 0..self.active_biquads {
            sample = self.biquad_stages[i].process(sample);
        }

        sample
    }

    /// Reset all filter states
    pub fn reset(&mut self) {
        self.first_order.reset();
        for stage in &mut self.biquad_stages {
            stage.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biquad_coefficients() {
        let mut biquad = BiquadState::default();
        biquad.set_highpass(44100.0, 1000.0, 0.707);

        assert!(biquad.b0.is_finite());
        assert!(biquad.b1.is_finite());
        assert!(biquad.b0 > 0.0);
        assert!(biquad.b1 < 0.0);
    }

    #[test]
    fn test_dc_rejection() {
        let mut filter = FilterChain::default();
        filter.update_coefficients(44100.0, 100.0, 0.707, FilterSlope::Slope12dB);

        let mut output = 0.0;
        for _ in 0..10000 {
            output = filter.process(1.0);
        }
        assert!(output.abs() < 0.001, "HPF should reject DC");
    }
}
