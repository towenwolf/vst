//! Reverse delay line using two overlapping grains with Hann window crossfade.
//!
//! Captures chunks of audio (chunk length = delay time) and plays them backwards.
//! Two grains staggered by half a chunk ensure click-free output.

use std::f32::consts::PI;

/// A single reverse-reading grain.
#[derive(Clone)]
struct Grain {
    /// Buffer position this grain reads backwards from.
    start: usize,
    /// Progress within the current chunk (0..chunk_size).
    counter: usize,
    /// Chunk size when this grain was triggered.
    chunk_size: usize,
}

impl Default for Grain {
    fn default() -> Self {
        Self {
            start: 0,
            counter: 0,
            chunk_size: 1,
        }
    }
}

/// Reverse delay line with two overlapping grains for click-free crossfade.
#[derive(Clone)]
pub struct ReverseDelayLine {
    buffer: Vec<f32>,
    write_pos: usize,
    max_samples: usize,
    grains: [Grain; 2],
}

impl Default for ReverseDelayLine {
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            write_pos: 0,
            max_samples: 0,
            grains: [Grain::default(), Grain::default()],
        }
    }
}

impl ReverseDelayLine {
    /// Initialize the reverse delay line for a given sample rate and max delay.
    pub fn initialize(&mut self, sample_rate: f32, max_delay_seconds: f32) {
        self.max_samples = (sample_rate * max_delay_seconds).ceil() as usize + 1;
        self.buffer = vec![0.0; self.max_samples];
        self.write_pos = 0;

        // Initialize grains with a default chunk size
        let default_chunk = (sample_rate * 0.3) as usize; // 300ms default
        let half_chunk = default_chunk / 2;

        self.grains[0] = Grain {
            start: 0,
            counter: 0,
            chunk_size: default_chunk,
        };
        self.grains[1] = Grain {
            start: 0,
            counter: half_chunk,
            chunk_size: default_chunk,
        };
    }

    /// Clear the buffer and reset grain state.
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;

        let chunk_size = self.grains[0].chunk_size;
        let half_chunk = chunk_size / 2;
        self.grains[0] = Grain {
            start: 0,
            counter: 0,
            chunk_size,
        };
        self.grains[1] = Grain {
            start: 0,
            counter: half_chunk,
            chunk_size,
        };
    }

    /// Write a sample to the circular buffer and advance the write position.
    #[inline]
    pub fn write(&mut self, sample: f32) {
        if self.buffer.is_empty() {
            return;
        }
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.max_samples;
    }

    /// Read reverse output and advance grain state.
    ///
    /// `delay_samples` sets the chunk size (how much audio is reversed at a time).
    /// Must be called exactly once per sample, paired with `write`.
    #[inline]
    pub fn read(&mut self, delay_samples: f32) -> f32 {
        if self.buffer.is_empty() {
            return 0.0;
        }

        let chunk_size = (delay_samples as usize).max(2);
        let mut output = 0.0;

        for grain in &mut self.grains {
            // Read from buffer in reverse: start position minus counter
            let read_pos = if grain.start >= grain.counter {
                grain.start - grain.counter
            } else {
                grain.start + self.max_samples - grain.counter
            };
            let sample = self.buffer[read_pos % self.max_samples];

            // Hann window: 0.5 * (1 - cos(2*PI*t)) where t = counter / chunk_size
            let t = grain.counter as f32 / grain.chunk_size as f32;
            let window = 0.5 * (1.0 - (2.0 * PI * t).cos());

            output += sample * window;

            // Advance grain
            grain.counter += 1;
            if grain.counter >= grain.chunk_size {
                // Reset grain: start reading from the most recent sample
                grain.counter = 0;
                grain.chunk_size = chunk_size;
                // Point to the last written sample (write_pos - 1)
                grain.start = if self.write_pos == 0 {
                    self.max_samples - 1
                } else {
                    self.write_pos - 1
                };
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_without_panic() {
        let mut rdl = ReverseDelayLine::default();
        rdl.initialize(44100.0, 2.5);
        assert_eq!(rdl.buffer.len(), rdl.max_samples);
    }

    #[test]
    fn reset_clears_buffer() {
        let mut rdl = ReverseDelayLine::default();
        rdl.initialize(44100.0, 1.0);
        rdl.write(1.0);
        rdl.write(0.5);
        rdl.reset();
        assert!(rdl.buffer.iter().all(|&s| s == 0.0));
    }

    #[test]
    fn produces_output_after_filling() {
        let mut rdl = ReverseDelayLine::default();
        rdl.initialize(44100.0, 1.0);

        // Fill with a ramp
        let chunk = 4410; // 100ms at 44.1kHz
        for i in 0..(chunk * 2) {
            rdl.write((i as f32) / (chunk as f32));
            let _ = rdl.read(chunk as f32);
        }

        // After two full chunks, grains should be producing non-zero output
        rdl.write(1.0);
        let out = rdl.read(chunk as f32);
        // Output may be small due to Hann window but should eventually be non-zero
        // across many samples
        let mut has_nonzero = false;
        for _ in 0..chunk {
            rdl.write(1.0);
            let o = rdl.read(chunk as f32);
            if o.abs() > 0.001 {
                has_nonzero = true;
                break;
            }
        }
        assert!(has_nonzero || out.abs() > 0.0, "Reverse delay should produce non-zero output");
    }

    #[test]
    fn hann_windows_sum_to_unity() {
        // Two Hann windows offset by half a period should sum to 1.0
        let n = 1000;
        let half = n / 2;
        for i in 0..n {
            let t0 = i as f32 / n as f32;
            let t1 = ((i + half) % n) as f32 / n as f32;
            let w0 = 0.5 * (1.0 - (2.0 * PI * t0).cos());
            let w1 = 0.5 * (1.0 - (2.0 * PI * t1).cos());
            let sum = w0 + w1;
            assert!(
                (sum - 1.0).abs() < 0.001,
                "Hann windows should sum to 1.0, got {sum} at i={i}"
            );
        }
    }
}
