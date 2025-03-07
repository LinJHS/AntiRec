use std::f32::consts::PI;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer: Vec<f32>,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer: Vec<f32>) -> Self {
        AudioProcessor { sample_rate, buffer }
    }

    pub fn process(&mut self) {
        let mut phase = 0.0;
        let phase_increment = 440.0 * 2.0 * PI / self.sample_rate as f32;

        for sample in &mut self.buffer {
            *sample += 0.1 * phase.sin(); // Add a sine wave disturbance
            phase += phase_increment;
            if phase >= 2.0 * PI {
                phase -= 2.0 * PI;
            }
        }
    }

    pub fn optimize(&mut self) {
        self.buffer = self.buffer.iter().map(|&x| x * 0.5).collect(); // Reduce amplitude for efficiency
    }

    pub fn get_buffer(&self) -> &Vec<f32> {
        &self.buffer
    }
}