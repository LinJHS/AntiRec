use std::f32::consts::PI;

// Audio processing module
pub mod audio_processing {
    use super::*;

    // Generate white noise
    pub fn white_noise(sample_rate: u32, duration_secs: u32) -> Vec<f32> {
        let num_samples = sample_rate * duration_secs;
        let mut noise = Vec::with_capacity(num_samples as usize);
        for _ in 0..num_samples {
            noise.push((rand::random::<f32>() - 0.5) * 2.0);
        }
        noise
    }

    // Add disturbance to audio signal
    pub fn add_disturbance(signal: &[f32], noise_level: f32) -> Vec<f32> {
        let noise = white_noise(44100, (signal.len() as u32) / 44100);
        signal.iter()
            .zip(noise.iter())
            .map(|(&s, &n)| s + n * noise_level)
            .collect()
    }

    // Fast Fourier Transform (FFT)
    pub fn fft(signal: &[f32]) -> Vec<f32> {
        let n = signal.len();
        let mut output = vec![0.0; n];
        let mut phase = vec![0.0; n];

        for k in 0..n {
            for t in 0..n {
                let angle = 2.0 * PI * (k as f32) * (t as f32) / (n as f32);
                output[k] += signal[t] * angle.cos();
                phase[k] += signal[t] * angle.sin();
            }
        }

        output.iter()
            .zip(phase.iter())
            .map(|(&re, &im)| (re * re + im * im).sqrt())
            .collect()
    }
}

// Efficient audio buffer processing
pub struct AudioBuffer {
    data: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    pub fn new(data: Vec<f32>, sample_rate: u32) -> Self {
        Self { data, sample_rate }
    }

    pub fn process<F>(&mut self, mut processor: F)
    where
        F: FnMut(f32) -> f32,
    {
        self.data = self.data.iter().map(|&sample| processor(sample)).collect();
    }

    pub fn normalize(&mut self) {
        let max_amplitude = self.data.iter()
            .fold(0.0, |max, &val| val.abs().max(max));
        
        if max_amplitude > 0.0 {
            self.data.iter_mut().for_each(|val| *val /= max_amplitude);
        }
    }
}