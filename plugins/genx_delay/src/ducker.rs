//! Ducking envelope follower for dynamic delay mixing.
//! The wet signal ducks when the dry signal is loud, and swells up during quieter moments.

/// Envelope follower for ducking effect.
#[derive(Clone, Copy)]
pub struct Ducker {
    envelope: f32,
    attack_coeff: f32,
    release_coeff: f32,
    sample_rate: f32,
}

impl Default for Ducker {
    fn default() -> Self {
        Self {
            envelope: 0.0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
            sample_rate: 44100.0,
        }
    }
}

impl Ducker {
    /// Initialize the ducker with sample rate.
    pub fn initialize(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.envelope = 0.0;
        // Default: fast attack, slow release
        self.set_times(5.0, 200.0);
    }

    /// Set attack and release times in milliseconds.
    pub fn set_times(&mut self, attack_ms: f32, release_ms: f32) {
        // Convert ms to coefficient (exponential smoothing)
        self.attack_coeff = (-1.0 / (self.sample_rate * attack_ms / 1000.0)).exp();
        self.release_coeff = (-1.0 / (self.sample_rate * release_ms / 1000.0)).exp();
    }

    /// Reset the envelope.
    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }

    /// Process a sample and return the ducking gain (0.0 = fully ducked, 1.0 = full volume).
    ///
    /// `input`: the dry input signal (or its absolute value)
    /// `threshold`: signal level above which ducking starts (0.0 - 1.0)
    /// `amount`: how much to duck (0.0 = no ducking, 1.0 = full ducking)
    #[inline]
    pub fn process(&mut self, input: f32, threshold: f32, amount: f32) -> f32 {
        let input_level = input.abs();

        // Envelope follower
        let coeff = if input_level > self.envelope {
            self.attack_coeff
        } else {
            self.release_coeff
        };
        self.envelope = coeff * self.envelope + (1.0 - coeff) * input_level;

        // Calculate ducking gain
        if self.envelope > threshold {
            let excess = (self.envelope - threshold) / (1.0 - threshold + 0.001);
            let duck_factor = 1.0 - (excess * amount).clamp(0.0, 1.0);
            duck_factor
        } else {
            1.0
        }
    }

    /// Process stereo input and return ducking gain.
    #[inline]
    pub fn process_stereo(&mut self, left: f32, right: f32, threshold: f32, amount: f32) -> f32 {
        let input_level = (left.abs() + right.abs()) * 0.5;
        self.process_level(input_level, threshold, amount)
    }

    /// Process with a pre-computed level.
    #[inline]
    fn process_level(&mut self, input_level: f32, threshold: f32, amount: f32) -> f32 {
        // Envelope follower
        let coeff = if input_level > self.envelope {
            self.attack_coeff
        } else {
            self.release_coeff
        };
        self.envelope = coeff * self.envelope + (1.0 - coeff) * input_level;

        // Calculate ducking gain
        if self.envelope > threshold {
            let excess = (self.envelope - threshold) / (1.0 - threshold + 0.001);
            let duck_factor = 1.0 - (excess * amount).clamp(0.0, 1.0);
            duck_factor
        } else {
            1.0
        }
    }
}
