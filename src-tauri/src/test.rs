use std::f32::consts::PI;

// Enhanced audio processing functions
pub struct AudioProcessor;

impl AudioProcessor {
    // Apply a low-pass filter to the audio signal
    pub fn low_pass_filter(signal: &mut [f32], cutoff_freq: f32, sample_rate: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / sample_rate;
        let alpha = dt / (rc + dt);

        for i in 1..signal.len() {
            signal[i] = signal[i - 1] + alpha * (signal[i] - signal[i - 1]);
        }
    }

    // Add white noise to the audio signal
    pub fn add_white_noise(signal: &mut [f32], noise_level: f32) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for sample in signal.iter_mut() {
            let noise: f32 = rng.gen_range(-noise_level..noise_level);
            *sample += noise;
        }
    }

    // Normalize the audio signal to the range [-1.0, 1.0]
    pub fn normalize(signal: &mut [f32]) {
        let max_amplitude = signal
            .iter()
            .map(|&x| x.abs())
            .fold(f32::MIN, |a, b| a.max(b));

        if max_amplitude > 0.0 {
            for sample in signal.iter_mut() {
                *sample /= max_amplitude;
            }
        }
    }

    // Efficiently compute the RMS (Root Mean Square) of the signal
    pub fn rms(signal: &[f32]) -> f32 {
        let sum_squares: f32 = signal.iter().map(|&x| x * x).sum();
        (sum_squares / signal.len() as f32).sqrt()
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processing() {
        let mut signal = vec![0.5, -0.3, 0.8, -0.7];
        AudioProcessor::low_pass_filter(&mut signal, 1000.0, 44100.0);
        AudioProcessor::add_white_noise(&mut signal, 0.05);
        AudioProcessor::normalize(&mut signal);
        let rms_value = AudioProcessor::rms(&signal);
        assert!(rms_value > 0.0);
    }
}