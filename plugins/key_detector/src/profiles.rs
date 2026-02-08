/// Krumhansl-Schmuckler key profiles
/// Based on Krumhansl & Kessler (1982) perceptual studies
///
/// Index: C=0, C#=1, D=2, D#=3, E=4, F=5, F#=6, G=7, G#=8, A=9, A#=10, B=11

/// Major key profile (C major as reference)
/// Tonic (6.35) > Fifth (5.19) > Major Third (4.38) > Fourth (4.09)
pub const MAJOR_PROFILE: [f32; 12] = [
    6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88,
];

/// Minor key profile (C minor as reference)
/// Tonic (6.33) > Minor Third (5.38) > Fifth (4.75) > Fourth (3.98)
pub const MINOR_PROFILE: [f32; 12] = [
    6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17,
];

/// Note names for display
pub const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// Z-score normalize a 12-element array
pub fn z_normalize(v: &[f32; 12]) -> [f32; 12] {
    let mean: f32 = v.iter().sum::<f32>() / 12.0;
    let variance: f32 = v.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / 12.0;
    let std_dev = variance.sqrt();

    let mut normalized = [0.0f32; 12];
    if std_dev > 1e-10 {
        for i in 0..12 {
            normalized[i] = (v[i] - mean) / std_dev;
        }
    }
    normalized
}

/// Rotate a profile by the given number of semitones
/// Used to transpose the reference profile to different keys
pub fn rotate_profile(profile: &[f32; 12], semitones: usize) -> [f32; 12] {
    let mut rotated = [0.0f32; 12];
    for i in 0..12 {
        rotated[i] = profile[(i + 12 - semitones) % 12];
    }
    rotated
}

/// Compute Pearson correlation between two z-normalized arrays
pub fn pearson_correlation(a: &[f32; 12], b: &[f32; 12]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>() / 12.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_normalize() {
        let input = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ];
        let normalized = z_normalize(&input);

        // Mean should be ~0
        let mean: f32 = normalized.iter().sum::<f32>() / 12.0;
        assert!(mean.abs() < 1e-6, "Mean should be ~0, got {}", mean);

        // Std dev should be ~1
        let variance: f32 = normalized.iter().map(|x| x.powi(2)).sum::<f32>() / 12.0;
        let std_dev = variance.sqrt();
        assert!(
            (std_dev - 1.0).abs() < 0.01,
            "Std dev should be ~1, got {}",
            std_dev
        );
    }

    #[test]
    fn test_rotate_profile() {
        let profile = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ];

        // Rotate by 0 should be identity
        let rotated = rotate_profile(&profile, 0);
        assert_eq!(rotated, profile);

        // Rotate by 1 (C# major would have profile starting from position 1)
        let rotated = rotate_profile(&profile, 1);
        assert_eq!(rotated[0], 12.0); // B becomes C position
        assert_eq!(rotated[1], 1.0); // C becomes C# position
    }

    #[test]
    fn test_self_correlation() {
        let normalized = z_normalize(&MAJOR_PROFILE);
        let corr = pearson_correlation(&normalized, &normalized);
        assert!(
            (corr - 1.0).abs() < 0.001,
            "Self-correlation should be 1.0, got {}",
            corr
        );
    }

    #[test]
    fn test_major_minor_different() {
        let major_norm = z_normalize(&MAJOR_PROFILE);
        let minor_norm = z_normalize(&MINOR_PROFILE);
        let corr = pearson_correlation(&major_norm, &minor_norm);

        // Major and minor should be somewhat correlated but not identical
        assert!(corr < 0.95, "Major and minor should differ");
        assert!(corr > 0.3, "Major and minor should have some similarity");
    }
}
