use crate::profiles::{
    pearson_correlation, rotate_profile, z_normalize, MAJOR_PROFILE, MINOR_PROFILE,
};

/// Result of key detection
#[derive(Debug, Clone, Copy)]
pub struct KeyResult {
    /// Root note (0=C, 1=C#, ..., 11=B)
    pub root: usize,
    /// True if major, false if minor
    pub is_major: bool,
    /// Correlation coefficient (-1.0 to 1.0)
    pub correlation: f32,
}

impl Default for KeyResult {
    fn default() -> Self {
        Self {
            root: 0,
            is_major: true,
            correlation: 0.0,
        }
    }
}

impl KeyResult {
    /// Convert correlation to confidence percentage (0-100)
    pub fn confidence(&self) -> f32 {
        ((self.correlation + 1.0) / 2.0 * 100.0).clamp(0.0, 100.0)
    }
}

/// Key detector with hysteresis to prevent output flickering
pub struct KeyDetector {
    current_key: KeyResult,
    hold_counter: u32,
    min_hold_frames: u32,
    correlation_threshold: f32,
}

impl KeyDetector {
    /// Create a new key detector
    /// - min_hold_frames: minimum frames before allowing key change
    /// - correlation_threshold: minimum improvement needed to change key
    pub fn new(min_hold_frames: u32, correlation_threshold: f32) -> Self {
        Self {
            current_key: KeyResult::default(),
            hold_counter: 0,
            min_hold_frames,
            correlation_threshold,
        }
    }

    /// Detect the key from a chroma vector
    pub fn detect(&self, chroma: &[f32; 12]) -> KeyResult {
        let chroma_norm = z_normalize(chroma);

        let mut best = KeyResult {
            root: 0,
            is_major: true,
            correlation: -1.0,
        };

        for root in 0..12 {
            // Test major key
            let major_rotated = rotate_profile(&MAJOR_PROFILE, root);
            let major_norm = z_normalize(&major_rotated);
            let major_corr = pearson_correlation(&chroma_norm, &major_norm);

            if major_corr > best.correlation {
                best = KeyResult {
                    root,
                    is_major: true,
                    correlation: major_corr,
                };
            }

            // Test minor key
            let minor_rotated = rotate_profile(&MINOR_PROFILE, root);
            let minor_norm = z_normalize(&minor_rotated);
            let minor_corr = pearson_correlation(&chroma_norm, &minor_norm);

            if minor_corr > best.correlation {
                best = KeyResult {
                    root,
                    is_major: false,
                    correlation: minor_corr,
                };
            }
        }

        best
    }

    /// Update the detector with a new chroma reading
    /// Applies hysteresis to prevent flickering
    pub fn update(&mut self, chroma: &[f32; 12]) -> KeyResult {
        let new_key = self.detect(chroma);

        let same_key =
            new_key.root == self.current_key.root && new_key.is_major == self.current_key.is_major;

        if same_key {
            self.hold_counter = self.hold_counter.saturating_add(1);
            self.current_key.correlation = new_key.correlation;
        } else if new_key.correlation > self.current_key.correlation + self.correlation_threshold
            || self.hold_counter >= self.min_hold_frames
        {
            self.current_key = new_key;
            self.hold_counter = 0;
        }

        self.current_key
    }

    /// Get the current detected key
    pub fn current(&self) -> KeyResult {
        self.current_key
    }

    /// Reset the detector state
    pub fn reset(&mut self) {
        self.current_key = KeyResult::default();
        self.hold_counter = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::MAJOR_PROFILE;

    #[test]
    fn test_detect_c_major() {
        let detector = KeyDetector::new(10, 0.1);

        // Use the C major profile as input (should detect C major)
        let mut chroma = [0.0f32; 12];
        for (i, &v) in MAJOR_PROFILE.iter().enumerate() {
            chroma[i] = v;
        }

        let result = detector.detect(&chroma);

        assert_eq!(result.root, 0, "Should detect C as root");
        assert!(result.is_major, "Should detect major mode");
        assert!(result.correlation > 0.95, "Should have high correlation");
    }

    #[test]
    fn test_detect_a_minor() {
        let detector = KeyDetector::new(10, 0.1);

        // Create A minor chroma (rotate minor profile to A)
        let a_minor = rotate_profile(&MINOR_PROFILE, 9);

        let result = detector.detect(&a_minor);

        assert_eq!(result.root, 9, "Should detect A as root");
        assert!(!result.is_major, "Should detect minor mode");
        assert!(result.correlation > 0.95, "Should have high correlation");
    }

    #[test]
    fn test_detect_all_major_keys() {
        let detector = KeyDetector::new(10, 0.1);

        for expected_root in 0..12 {
            let chroma = rotate_profile(&MAJOR_PROFILE, expected_root);
            let result = detector.detect(&chroma);

            assert_eq!(
                result.root, expected_root,
                "Should detect root {} for rotation {}",
                expected_root, expected_root
            );
            assert!(
                result.is_major,
                "Should detect major for root {}",
                expected_root
            );
        }
    }

    #[test]
    fn test_confidence_mapping() {
        let result = KeyResult {
            root: 0,
            is_major: true,
            correlation: 1.0,
        };
        assert!((result.confidence() - 100.0).abs() < 0.01);

        let result = KeyResult {
            root: 0,
            is_major: true,
            correlation: 0.0,
        };
        assert!((result.confidence() - 50.0).abs() < 0.01);

        let result = KeyResult {
            root: 0,
            is_major: true,
            correlation: -1.0,
        };
        assert!(result.confidence().abs() < 0.01);
    }

    #[test]
    fn test_hysteresis() {
        // Hysteresis with high threshold (0.5) means we need much better correlation to switch
        let mut detector = KeyDetector::new(5, 0.5);

        // First detect C major with perfect correlation
        let c_major = MAJOR_PROFILE;
        detector.update(&c_major);
        assert_eq!(
            detector.current().root,
            0,
            "Should detect C major initially"
        );

        // Feed G major - same correlation, so shouldn't switch (not significantly better)
        let g_major = rotate_profile(&MAJOR_PROFILE, 7);
        detector.update(&g_major);
        assert_eq!(
            detector.current().root,
            0,
            "Should stay at C (G not significantly better)"
        );

        // Now test with low threshold (0.0) - any different key with equal/better correlation switches
        let mut detector2 = KeyDetector::new(0, 0.0);
        detector2.update(&c_major);
        assert_eq!(detector2.current().root, 0);

        // With 0 threshold, G major (equal correlation) should switch
        detector2.update(&g_major);
        assert_eq!(
            detector2.current().root,
            7,
            "With 0 threshold, should switch to G"
        );
    }
}
