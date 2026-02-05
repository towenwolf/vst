//! Modulation oscillator for adding chorus/vibrato to the delay.

use std::f32::consts::PI;

/// Low-frequency oscillator for delay time modulation.
#[derive(Clone, Copy)]
pub struct Modulator {
    phase: f32,
    sample_rate: f32,
}

impl Default for Modulator {
    fn default() -> Self {
        Self {
            phase: 0.0,
            sample_rate: 44100.0,
        }
    }
}

impl Modulator {
    /// Initialize with sample rate.
    pub fn initialize(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.phase = 0.0;
    }

    /// Reset the oscillator phase.
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    /// Get the next modulation value.
    /// Returns a value between -1.0 and 1.0.
    /// 
    /// `rate`: modulation rate in Hz (typically 0.1 - 5.0 Hz)
    #[inline]
    pub fn next(&mut self, rate: f32) -> f32 {
        let phase_increment = rate / self.sample_rate;
        self.phase += phase_increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        
        // Sine wave modulation
        (self.phase * 2.0 * PI).sin()
    }

    /// Get modulated delay time in samples.
    /// 
    /// `base_delay_samples`: the base delay time in samples
    /// `depth_samples`: maximum modulation depth in samples
    /// `rate`: modulation rate in Hz
    #[inline]
    pub fn get_modulated_delay(&mut self, base_delay_samples: f32, depth_samples: f32, rate: f32) -> f32 {
        let mod_value = self.next(rate);
        base_delay_samples + mod_value * depth_samples
    }
}

/// Dual modulator for stereo with phase offset (creates width).
#[derive(Clone, Copy, Default)]
pub struct StereoModulator {
    left: Modulator,
    right: Modulator,
}

impl StereoModulator {
    /// Initialize both modulators.
    pub fn initialize(&mut self, sample_rate: f32) {
        self.left.initialize(sample_rate);
        self.right.initialize(sample_rate);
        // Offset right channel phase for stereo width
        self.right.phase = 0.5;
    }

    /// Reset both modulators.
    pub fn reset(&mut self) {
        self.left.reset();
        self.right.phase = 0.5; // Keep the offset
    }

    /// Get modulated delay times for both channels.
    #[inline]
    pub fn get_modulated_delays(
        &mut self,
        base_delay_samples_l: f32,
        base_delay_samples_r: f32,
        depth_samples: f32,
        rate: f32,
    ) -> (f32, f32) {
        let left = self.left.get_modulated_delay(base_delay_samples_l, depth_samples, rate);
        let right = self.right.get_modulated_delay(base_delay_samples_r, depth_samples, rate);
        (left, right)
    }
}
