/// Chroma feature extractor
/// Maps FFT magnitude bins to 12 pitch classes (C, C#, D, ..., B)
pub struct ChromaExtractor {
    sample_rate: f32,
    fft_size: usize,
    bin_to_pitch_class: Vec<Option<u8>>,
    current_chroma: [f32; 12],
    smoothed_chroma: [f32; 12],
    smoothing_alpha: f32,
}

impl ChromaExtractor {
    /// Create a new chroma extractor
    pub fn new(sample_rate: f32, fft_size: usize, smoothing_tau: f32) -> Self {
        let bin_to_pitch_class = Self::build_pitch_class_lut(sample_rate, fft_size);
        let hop_time = (fft_size as f32 / 4.0) / sample_rate; // 75% overlap
        let smoothing_alpha = Self::alpha_from_tau(smoothing_tau, hop_time);

        Self {
            sample_rate,
            fft_size,
            bin_to_pitch_class,
            current_chroma: [0.0; 12],
            smoothed_chroma: [0.0; 12],
            smoothing_alpha,
        }
    }

    /// Build lookup table mapping FFT bins to pitch classes
    fn build_pitch_class_lut(sample_rate: f32, fft_size: usize) -> Vec<Option<u8>> {
        let num_bins = fft_size / 2 + 1;
        (0..num_bins)
            .map(|bin| Self::bin_to_pitch_class(bin, sample_rate, fft_size))
            .collect()
    }

    /// Map a single FFT bin to its pitch class (0-11)
    /// Returns None if frequency is outside the analysis range (65-2000 Hz)
    fn bin_to_pitch_class(bin: usize, sample_rate: f32, fft_size: usize) -> Option<u8> {
        let freq = bin as f32 * sample_rate / fft_size as f32;

        // Filter: 65 Hz (C2) to 2000 Hz (B6)
        if freq < 65.0 || freq > 2000.0 {
            return None;
        }

        // Convert to MIDI note (A4 = 440 Hz = MIDI 69)
        let midi_note = 12.0 * (freq / 440.0).log2() + 69.0;

        // Pitch class = MIDI note mod 12
        Some((midi_note.round() as i32).rem_euclid(12) as u8)
    }

    /// Calculate smoothing alpha from time constant
    fn alpha_from_tau(tau: f32, hop_time: f32) -> f32 {
        if tau <= 0.0 {
            1.0 // Instant update
        } else {
            1.0 - (-hop_time / tau).exp()
        }
    }

    /// Update smoothing time constant
    pub fn set_smoothing(&mut self, tau: f32) {
        let hop_time = (self.fft_size as f32 / 4.0) / self.sample_rate;
        self.smoothing_alpha = Self::alpha_from_tau(tau, hop_time);
    }

    /// Extract chroma from magnitude spectrum and apply smoothing
    /// Returns the smoothed chroma vector
    pub fn process(&mut self, magnitude: &[f32]) -> &[f32; 12] {
        // Reset current chroma
        self.current_chroma.fill(0.0);

        // Accumulate magnitude into pitch class bins
        for (bin, &mag) in magnitude.iter().enumerate() {
            if let Some(pc) = self.bin_to_pitch_class.get(bin).copied().flatten() {
                self.current_chroma[pc as usize] += mag;
            }
        }

        // Normalize current chroma
        let sum: f32 = self.current_chroma.iter().sum();
        if sum > 1e-10 {
            for c in &mut self.current_chroma {
                *c /= sum;
            }
        }

        // Apply exponential smoothing
        for i in 0..12 {
            self.smoothed_chroma[i] = self.smoothing_alpha * self.current_chroma[i]
                + (1.0 - self.smoothing_alpha) * self.smoothed_chroma[i];
        }

        &self.smoothed_chroma
    }

    /// Get the current smoothed chroma
    pub fn chroma(&self) -> &[f32; 12] {
        &self.smoothed_chroma
    }

    /// Reset the chroma state
    pub fn reset(&mut self) {
        self.current_chroma.fill(0.0);
        self.smoothed_chroma.fill(0.0);
    }

    /// Reconfigure for new sample rate and FFT size
    pub fn reconfigure(&mut self, sample_rate: f32, fft_size: usize, smoothing_tau: f32) {
        self.sample_rate = sample_rate;
        self.fft_size = fft_size;
        self.bin_to_pitch_class = Self::build_pitch_class_lut(sample_rate, fft_size);
        self.set_smoothing(smoothing_tau);
        self.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_bin_to_pitch_class_a4() {
        // 440 Hz should map to A (pitch class 9)
        let sample_rate = 44100.0;
        let fft_size = 4096;
        let bin = (440.0 * fft_size as f32 / sample_rate).round() as usize;

        let pc = ChromaExtractor::bin_to_pitch_class(bin, sample_rate, fft_size);
        assert_eq!(pc, Some(9), "440 Hz should be pitch class A (9)");
    }

    #[test]
    fn test_bin_to_pitch_class_c4() {
        // 261.63 Hz (C4) should map to C (pitch class 0)
        let sample_rate = 44100.0;
        let fft_size = 4096;
        let bin = (261.63 * fft_size as f32 / sample_rate).round() as usize;

        let pc = ChromaExtractor::bin_to_pitch_class(bin, sample_rate, fft_size);
        assert_eq!(pc, Some(0), "261.63 Hz should be pitch class C (0)");
    }

    #[test]
    fn test_frequency_filtering() {
        let sample_rate = 44100.0;
        let fft_size = 4096;

        // DC (0 Hz) should be filtered
        assert_eq!(
            ChromaExtractor::bin_to_pitch_class(0, sample_rate, fft_size),
            None
        );

        // Very low frequency (< 65 Hz) should be filtered
        let low_bin = (50.0 * fft_size as f32 / sample_rate).round() as usize;
        assert_eq!(
            ChromaExtractor::bin_to_pitch_class(low_bin, sample_rate, fft_size),
            None
        );

        // Very high frequency (> 2000 Hz) should be filtered
        let high_bin = (3000.0 * fft_size as f32 / sample_rate).round() as usize;
        assert_eq!(
            ChromaExtractor::bin_to_pitch_class(high_bin, sample_rate, fft_size),
            None
        );
    }

    #[test]
    fn test_chroma_extraction() {
        let sample_rate = 44100.0;
        let fft_size = 4096;
        let mut extractor = ChromaExtractor::new(sample_rate, fft_size, 0.0); // No smoothing

        // Create a fake magnitude spectrum with a peak at A4 (440 Hz)
        let num_bins = fft_size / 2 + 1;
        let mut magnitude = vec![0.0f32; num_bins];

        let a4_bin = (440.0 * fft_size as f32 / sample_rate).round() as usize;
        magnitude[a4_bin] = 1.0;

        let chroma = extractor.process(&magnitude);

        // A (pitch class 9) should be dominant
        let max_pc = chroma
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        assert_eq!(max_pc, 9, "A should be dominant pitch class");
    }
}
