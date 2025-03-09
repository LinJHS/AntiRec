use std::f32::consts::PI;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct AudioSample {
    pub data: Vec<f32>,
    pub sample_rate: u32,
}

impl AudioSample {
    pub fn new(data: Vec<f32>, sample_rate: u32) -> Self {
        AudioSample { data, sample_rate }
    }

    pub fn add_noise(&mut self, noise_level: f32) {
        let mut rng = rand::thread_rng();
        for sample in &mut self.data {
            let noise: f32 = rng.gen_range(-noise_level..noise_level);
            *sample += noise;
        }
    }

    pub fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_filtered = self.data[0];
        for sample in &mut self.data {
            *sample = alpha * *sample + (1.0 - alpha) * prev_filtered;
            prev_filtered = *sample;
        }
    }

    pub fn normalize(&mut self) {
        let max_amplitude = self
            .data
            .iter()
            .fold(0.0, |max, &val| val.abs().max(max));

        if max_amplitude > 0.0 {
            for sample in &mut self.data {
                *sample /= max_amplitude;
            }
        }
    }

    pub fn mix(&self, other: &AudioSample) -> AudioSample {
        assert_eq!(self.sample_rate, other.sample_rate);
        let mixed_data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();

        AudioSample::new(mixed_data, self.sample_rate)
    }
}

pub fn generate_sinusoid(frequency: f32, duration: f32, sample_rate: u32) -> AudioSample {
    let num_samples = (duration * sample_rate as f32) as usize;
    let mut data = Vec::with_capacity(num_samples);
    let angular_frequency = 2.0 * PI * frequency;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        data.push((angular_frequency * t).sin());
    }

    AudioSample::new(data, sample_rate)
}

pub fn process_audio(audio: AudioSample) -> AudioSample {
    let mut processed_audio = audio.clone();
    processed_audio.add_noise(0.01);
    processed_audio.apply_low_pass_filter(1000.0);
    processed_audio.normalize();
    processed_audio
}