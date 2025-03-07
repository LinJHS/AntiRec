mod audio_processing {
    use std::f32::consts::PI;
    use std::sync::Arc;
    use std::thread;

    #[derive(Clone)]
    pub struct AudioBuffer {
        samples: Vec<f32>,
        sample_rate: u32,
    }

    impl AudioBuffer {
        pub fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
            AudioBuffer { samples, sample_rate }
        }

        pub fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
            let mut rng = rand::thread_rng();
            for sample in &mut self.samples {
                let noise = amplitude * (2.0 * PI * frequency * *sample / self.sample_rate as f32).sin();
                *sample += noise * rng.gen_range(-1.0..1.0);
            }
        }

        pub fn process_in_parallel(&mut self, num_threads: usize, f: fn(&mut [f32])) {
            let chunk_size = self.samples.len() / num_threads;
            let mut handles = vec![];

            let samples_arc = Arc::new(std::mem::take(&mut self.samples));

            for i in 0..num_threads {
                let samples_arc = Arc::clone(&samples_arc);
                let start = i * chunk_size;
                let end = if i == num_threads - 1 {
                    samples_arc.len()
                } else {
                    start + chunk_size
                };

                handles.push(thread::spawn(move || {
                    let mut chunk = samples_arc[start..end].to_vec();
                    f(&mut chunk);
                    chunk
                }));
            }

            let mut processed_samples = Vec::with_capacity(self.samples.len());
            for handle in handles {
                processed_samples.extend(handle.join().unwrap());
            }

            self.samples = processed_samples;
        }

        pub fn normalize(&mut self) {
            let max_amplitude = self.samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
            if max_amplitude > 0.0 {
                for sample in &mut self.samples {
                    *sample /= max_amplitude;
                }
            }
        }
    }

    pub fn apply_low_pass_filter(buffer: &mut [f32], cutoff_frequency: f32, sample_rate: f32) {
        let dt = 1.0 / sample_rate;
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let alpha = dt / (rc + dt);

        let mut prev_output = buffer[0];
        for sample in buffer.iter_mut().skip(1) {
            *sample = prev_output + alpha * (*sample - prev_output);
            prev_output = *sample;
        }
    }
}

fn main() {
    let mut audio_buffer = audio_processing::AudioBuffer::new(vec![0.5, -0.3, 0.8, -0.1], 44100);
    audio_buffer.add_disturbance(1000.0, 0.1);
    audio_buffer.process_in_parallel(4, |chunk| {
        audio_processing::apply_low_pass_filter(chunk, 5000.0, 44100.0);
    });
    audio_buffer.normalize();
}