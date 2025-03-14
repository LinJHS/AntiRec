use std::f32::consts::PI;

#[derive(Debug)]
struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer { samples, sample_rate }
    }
    
    fn apply_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let phase_shift = 2.0 * PI * frequency / self.sample_rate as f32;
        for (i, sample) in self.samples.iter_mut().enumerate() {
            let disturbance = amplitude * (phase_shift * i as f32).sin();
            *sample += disturbance;
        }
    }

    fn normalize(&mut self) {
        let max_amplitude = self.samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in self.samples.iter_mut() {
                *sample /= max_amplitude;
            }
        }
    }
    
    fn resample(&self, new_sample_rate: u32) -> AudioBuffer {
        let ratio = new_sample_rate as f32 / self.sample_rate as f32;
        let new_samples: Vec<f32> = (0..(self.samples.len() as f32 * ratio) as usize)
            .map(|i| {
                let pos = i as f32 / ratio;
                let prev = pos.floor() as usize;
                let next = (prev + 1).min(self.samples.len() - 1);
                let alpha = pos - pos.floor();
                self.samples[prev] * (1.0 - alpha) + self.samples[next] * alpha
            })
            .collect();
        AudioBuffer::new(new_samples, new_sample_rate)
    }
}

fn main() {
    let samples = vec![0.0, 0.5, 1.0, 0.5, 0.0, -0.5, -1.0, -0.5];
    let mut buffer = AudioBuffer::new(samples, 44100);
    
    buffer.apply_disturbance(1000.0, 0.1);
    buffer.normalize();
    
    let resampled_buffer = buffer.resample(48000);
    println!("{:?}", resampled_buffer);
}