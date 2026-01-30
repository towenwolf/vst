use num_complex::Complex32;
use realfft::{RealFftPlanner, RealToComplex};
use std::f32::consts::PI;
use std::sync::Arc;

/// FFT processor with pre-computed Hann window
pub struct FftProcessor {
    fft: Arc<dyn RealToComplex<f32>>,
    fft_size: usize,
    hann_window: Vec<f32>,
    input_buffer: Vec<f32>,
    output_buffer: Vec<Complex32>,
    scratch: Vec<Complex32>,
}

impl FftProcessor {
    /// Create a new FFT processor with the given size
    pub fn new(fft_size: usize) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(fft_size);

        let input_buffer = fft.make_input_vec();
        let output_buffer = fft.make_output_vec();
        let scratch = fft.make_scratch_vec();

        // Pre-compute Hann window
        let hann_window: Vec<f32> = (0..fft_size)
            .map(|n| 0.5 * (1.0 - (2.0 * PI * n as f32 / (fft_size - 1) as f32).cos()))
            .collect();

        Self {
            fft,
            fft_size,
            hann_window,
            input_buffer,
            output_buffer,
            scratch,
        }
    }

    /// Process input samples and return magnitude spectrum
    /// Input must have exactly fft_size samples
    pub fn process(&mut self, input: &[f32]) -> &[f32] {
        debug_assert_eq!(input.len(), self.fft_size);

        // Apply Hann window
        for (i, (&sample, &window)) in input.iter().zip(self.hann_window.iter()).enumerate() {
            self.input_buffer[i] = sample * window;
        }

        // Perform FFT
        self.fft
            .process_with_scratch(&mut self.input_buffer, &mut self.output_buffer, &mut self.scratch)
            .expect("FFT processing failed");

        // Compute magnitude spectrum (reuse input_buffer for output)
        // Only need first half + 1 bins (DC to Nyquist)
        let num_bins = self.fft_size / 2 + 1;
        for i in 0..num_bins {
            let c = self.output_buffer[i];
            self.input_buffer[i] = (c.re * c.re + c.im * c.im).sqrt();
        }

        &self.input_buffer[..num_bins]
    }

    /// Get the FFT size
    pub fn fft_size(&self) -> usize {
        self.fft_size
    }

    /// Resize the FFT processor (recreates internal buffers)
    pub fn resize(&mut self, new_size: usize) {
        if new_size != self.fft_size {
            *self = Self::new(new_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_dc() {
        let mut fft = FftProcessor::new(1024);

        // DC signal windowed with Hann will have energy in DC bin
        // Note: Hann window causes spectral leakage, so DC won't fully dominate
        let input: Vec<f32> = vec![1.0; 1024];
        let magnitude = fft.process(&input);

        // DC bin should have the most energy
        let (max_bin, _) = magnitude
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();

        assert_eq!(max_bin, 0, "DC signal should have peak at bin 0");
    }

    #[test]
    fn test_fft_sine() {
        let mut fft = FftProcessor::new(1024);
        let sample_rate = 44100.0;
        let freq = 440.0;

        // Generate 440 Hz sine wave
        let input: Vec<f32> = (0..1024)
            .map(|i| (2.0 * PI * freq * i as f32 / sample_rate).sin())
            .collect();

        let magnitude = fft.process(&input);

        // Find the bin with maximum energy
        let expected_bin = (freq * 1024.0 / sample_rate).round() as usize;
        let (max_bin, _) = magnitude
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();

        // Should be within 1 bin of expected
        assert!(
            (max_bin as i32 - expected_bin as i32).abs() <= 1,
            "Peak at bin {} but expected ~{}",
            max_bin,
            expected_bin
        );
    }
}
