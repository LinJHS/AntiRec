use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;

#[derive(Clone)]
struct AudioBuffer {
    samples: Arc<Vec<f32>>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer {
            samples: Arc::new(samples),
            sample_rate,
        }
    }

    fn apply_disturbance(&self, frequency: f32, amplitude: f32) -> Self {
        let mut disturbed_samples = Vec::with_capacity(self.samples.len());
        for (i, sample) in self.samples.iter().enumerate() {
            let t = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * t).sin();
            disturbed_samples.push(sample + disturbance);
        }
        AudioBuffer::new(disturbed_samples, self.sample_rate)
    }

    fn process_in_parallel(&self, num_threads: usize, process_fn: fn(f32) -> f32) -> Self {
        let chunks = self.samples.chunks(self.samples.len() / num_threads);
        let handles: Vec<_> = chunks
            .map(|chunk| {
                let chunk = chunk.to_vec();
                thread::spawn(move || chunk.into_iter().map(process_fn).collect::<Vec<_>>())
            })
            .collect();

        let processed_samples: Vec<f32> = handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap())
            .collect();

        AudioBuffer::new(processed_samples, self.sample_rate)
    }

    fn normalize(&self, target_level: f32) -> Self {
        let max_sample = self.samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_sample == 0.0 {
            return self.clone();
        }
        let gain = target_level / max_sample;
        let normalized_samples = self.samples.iter().map(|x| x * gain).collect();
        AudioBuffer::new(normalized_samples, self.sample_rate)
    }
}

fn main() {
    let samples = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let audio_buffer = AudioBuffer::new(samples, 44100);

    let disturbed_buffer = audio_buffer.apply_disturbance(1000.0, 0.05);
    let processed_buffer = disturbed_buffer.process_in_parallel(4, |x| x * 2.0);
    let normalized_buffer = processed_buffer.normalize(1.0);

    println!("{:?}", normalized_buffer.samples);
}