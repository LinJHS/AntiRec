use std::f32::consts::PI;
use std::sync::Arc;
use rayon::prelude::*;

#[derive(Clone)]
pub struct AudioProcessor {
    sample_rate: u32,
    noise_level: f32,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, noise_level: f32) -> Self {
        AudioProcessor { sample_rate, noise_level }
    }

    pub fn process_audio(&self, samples: &mut [f32]) {
        samples.par_iter_mut().for_each(|sample| {
            *sample = self.add_disturbance(*sample);
            *sample = self.apply_compression(*sample);
        });
    }

    fn add_disturbance(&self, sample: f32) -> f32 {
        let noise = rand::random::<f32>() * self.noise_level;
        sample + (noise - self.noise_level / 2.0)
    }

    fn apply_compression(&self, sample: f32) -> f32 {
        let threshold = 0.5;
        let ratio = 4.0;
        if sample.abs() > threshold {
            threshold + (sample.abs() - threshold) / ratio
        } else {
            sample
        }
    }

    pub fn generate_sine_wave(&self, frequency: f32, duration: f32) -> Vec<f32> {
        let num_samples = (self.sample_rate as f32 * duration) as usize;
        let mut sine_wave = vec![0.0; num_samples];
        for i in 0..num_samples {
            let t = i as f32 / self.sample_rate as f32;
            sine_wave[i] = (2.0 * PI * frequency * t).sin();
        }
        sine_wave
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processor() {
        let processor = AudioProcessor::new(44100, 0.1);
        let mut samples = vec![0.5, -0.3, 0.8, -0.9];
        processor.process_audio(&mut samples);
        assert_ne!(samples[0], 0.5);
    }
}