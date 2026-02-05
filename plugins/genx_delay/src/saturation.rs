//! Soft saturation for analog warmth in the feedback path.

/// Soft clipper / saturation for adding subtle warmth.
#[derive(Clone, Copy, Default)]
pub struct Saturator {
    drive: f32,
}

impl Saturator {
    /// Set the drive amount (0.0 = clean, 1.0 = heavily saturated).
    pub fn set_drive(&mut self, drive: f32) {
        self.drive = drive.clamp(0.0, 1.0);
    }

    /// Process a sample with soft saturation.
    /// Uses tanh-style soft clipping.
    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        if self.drive < 0.001 {
            return input;
        }

        // Scale input by drive amount (more drive = more saturation)
        let drive_scale = 1.0 + self.drive * 4.0;
        let driven = input * drive_scale;

        // Soft clipping using tanh approximation (faster than actual tanh)
        let saturated = soft_clip(driven);

        // Compensate for volume increase
        let output_scale = 1.0 / (1.0 + self.drive * 0.5);
        saturated * output_scale
    }
}

/// Fast tanh approximation for soft clipping.
/// Pade approximant - accurate and fast.
#[inline]
fn soft_clip(x: f32) -> f32 {
    let x2 = x * x;
    x * (27.0 + x2) / (27.0 + 9.0 * x2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soft_clip_symmetry() {
        assert!((soft_clip(0.5) + soft_clip(-0.5)).abs() < 0.0001);
        assert!((soft_clip(1.0) + soft_clip(-1.0)).abs() < 0.0001);
    }

    #[test]
    fn test_soft_clip_limits() {
        // Should approach ±1 for large inputs
        assert!(soft_clip(10.0) < 1.1);
        assert!(soft_clip(-10.0) > -1.1);
    }
}
