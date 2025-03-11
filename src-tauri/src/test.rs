use std::f32::consts::PI;
use std::sync::Arc;

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
        let mut rng = rand::thread_rng();
        let noise: Vec<f32> = (0..self.buffer.len())
            .map(|_| (rng.gen::<f32>() * 2.0 - 1.0) * self.disturbance_level)
            .collect();

        for i in 0..self.buffer.len() {
            self.buffer[i] = (self.buffer[i] + noise[i]).max(-1.0).min(1.0);
        }
    }

    pub fn apply_low_pass_filter(&mut self, cutoff_freq: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_output = 0.0;
        for sample in &mut self.buffer {
            *sample = prev_output + alpha * (*sample - prev_output);
            prev_output = *sample;
        }
    }

    pub fn get_buffer(&self) -> &[f32] {
        &self.buffer
    }
}

pub fn process_audio_concurrently(audio_processors: Vec<Arc<AudioProcessor>>) {
    let handles: Vec<_> = audio_processors
        .into_iter()
        .map(|ap| {
            std::thread::spawn(move || {
                let mut ap = Arc::try_unwrap(ap).unwrap();
                ap.process();
                ap.apply_low_pass_filter(1000.0);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}