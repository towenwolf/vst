//! Delay line implementation with interpolation for smooth delay time changes.

/// A delay line with linear interpolation for sub-sample accuracy.
#[derive(Clone)]
pub struct DelayLine {
    buffer: Vec<f32>,
    write_pos: usize,
    max_delay_samples: usize,
}

impl Default for DelayLine {
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            write_pos: 0,
            max_delay_samples: 0,
        }
    }
}

impl DelayLine {
    /// Initialize the delay line for a given sample rate and max delay time in seconds.
    pub fn initialize(&mut self, sample_rate: f32, max_delay_seconds: f32) {
        self.max_delay_samples = (sample_rate * max_delay_seconds).ceil() as usize + 1;
        self.buffer = vec![0.0; self.max_delay_samples];
        self.write_pos = 0;
    }

    /// Clear the delay buffer.
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }

    /// Write a sample to the delay line and advance the write position.
    #[inline]
    pub fn write(&mut self, sample: f32) {
        if self.buffer.is_empty() {
            return;
        }
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.max_delay_samples;
    }

    /// Read from the delay line with linear interpolation.
    /// `delay_samples` is the delay time in samples (can be fractional).
    #[inline]
    pub fn read(&self, delay_samples: f32) -> f32 {
        if self.buffer.is_empty() {
            return 0.0;
        }

        let delay_samples = delay_samples.clamp(0.0, (self.max_delay_samples - 1) as f32);
        let delay_int = delay_samples.floor() as usize;
        let frac = delay_samples - delay_int as f32;

        // Calculate read positions (going backwards from write position)
        let read_pos_0 = (self.write_pos + self.max_delay_samples - delay_int - 1) % self.max_delay_samples;
        let read_pos_1 = (read_pos_0 + self.max_delay_samples - 1) % self.max_delay_samples;

        // Linear interpolation between two samples
        let sample_0 = self.buffer[read_pos_0];
        let sample_1 = self.buffer[read_pos_1];

        sample_0 + frac * (sample_1 - sample_0)
    }

    /// Read and write in one operation (tap-then-write).
    #[inline]
    pub fn process(&mut self, input: f32, delay_samples: f32) -> f32 {
        let output = self.read(delay_samples);
        self.write(input);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_line_basic() {
        let mut dl = DelayLine::default();
        dl.initialize(44100.0, 1.0);
        
        // Write some samples
        for i in 0..100 {
            dl.write(i as f32);
        }
        
        // Read with 10 sample delay
        let output = dl.read(10.0);
        assert!((output - 89.0).abs() < 0.001);
    }
}
