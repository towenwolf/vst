/// A simple ring buffer for accumulating audio samples
pub struct RingBuffer {
    data: Vec<f32>,
    write_pos: usize,
    capacity: usize,
}

impl RingBuffer {
    /// Create a new ring buffer with the given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0.0; capacity],
            write_pos: 0,
            capacity,
        }
    }

    /// Push a single sample into the buffer
    #[inline]
    pub fn push(&mut self, sample: f32) {
        self.data[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.capacity;
    }

    /// Copy the most recent `len` samples to the output slice
    /// Output will be in chronological order (oldest first)
    pub fn copy_to_slice(&self, output: &mut [f32]) {
        let len = output.len().min(self.capacity);

        // Calculate the start position (oldest sample we need)
        let start = (self.write_pos + self.capacity - len) % self.capacity;

        if start + len <= self.capacity {
            // No wrap-around needed
            output[..len].copy_from_slice(&self.data[start..start + len]);
        } else {
            // Handle wrap-around
            let first_part = self.capacity - start;
            output[..first_part].copy_from_slice(&self.data[start..]);
            output[first_part..len].copy_from_slice(&self.data[..len - first_part]);
        }
    }

    /// Reset the buffer to all zeros
    pub fn reset(&mut self) {
        self.data.fill(0.0);
        self.write_pos = 0;
    }

    /// Resize the buffer (clears all data)
    pub fn resize(&mut self, new_capacity: usize) {
        self.data.resize(new_capacity, 0.0);
        self.data.fill(0.0);
        self.capacity = new_capacity;
        self.write_pos = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_copy() {
        let mut rb = RingBuffer::new(4);

        rb.push(1.0);
        rb.push(2.0);
        rb.push(3.0);
        rb.push(4.0);

        let mut output = [0.0f32; 4];
        rb.copy_to_slice(&mut output);

        assert_eq!(output, [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_wrap_around() {
        let mut rb = RingBuffer::new(4);

        // Fill buffer
        for i in 1..=4 {
            rb.push(i as f32);
        }

        // Overwrite first two
        rb.push(5.0);
        rb.push(6.0);

        let mut output = [0.0f32; 4];
        rb.copy_to_slice(&mut output);

        assert_eq!(output, [3.0, 4.0, 5.0, 6.0]);
    }
}
