use std::f32::consts::PI;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer: Vec<f32>,
    disturbance_level: f32,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer_size: usize, disturbance_level: f32) -> Self {
        AudioProcessor {
            sample_rate,
            buffer: vec![0.0; buffer_size],
            disturbance_level,
        }
    }

    pub fn process(&mut self) {
        let mut phase = 0.0;
        let phase_increment = 440.0 * 2.0 * PI / self.sample_rate as f32;

        for sample in &mut self.buffer {
            *sample = (phase.sin() + self.disturbance_level * fast_rand() as f32 / i32::MAX as f32).clamp(-1.0, 1.0);
            phase = (phase + phase_increment) % (2.0 * PI);
        }
    }

    pub fn apply_disturbance(&mut self, disturbance: f32) {
        self.disturbance_level = disturbance;
    }

    pub fn get_buffer(&self) -> &[f32] {
        &self.buffer
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.iter_mut().for_each(|x| *x = 0.0);
    }
}

fn fast_rand() -> i32 {
    static mut SEED: u32 = 42;
    unsafe {
        SEED = SEED.wrapping_mul(1664525).wrapping_add(1013904223);
        SEED as i32
    }
}