use std::f32::consts::PI;
use ndarray::{Array1, ArrayView1};
use rustfft::{Fft, FftDirection, num_complex::Complex};

pub struct AudioProcessor {
    sample_rate: u32,
    fft: Fft<f32>,
    window_size: usize,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, window_size: usize) -> Self {
        let fft = Fft::new(window_size, FftDirection::Forward);
        AudioProcessor {
            sample_rate,
            fft,
            window_size,
        }
    }

    pub fn add_disturbance(&self, signal: &mut Array1<f32>, frequency: f32, amplitude: f32) {
        let omega = 2.0 * PI * frequency / self.sample_rate as f32;
        signal.iter_mut().enumerate().for_each(|(i, x)| {
            *x += amplitude * (omega * i as f32).sin();
        });
    }

    pub fn apply_fft(&self, signal: ArrayView1<f32>) -> Array1<Complex<f32>> {
        let mut buffer = signal.mapv(|x| Complex::new(x, 0.0)).into_raw_vec();
        self.fft.process(&mut buffer);
        Array1::from(buffer)
    }

    pub fn spectral_subtraction(&self, noisy_signal: ArrayView1<f32>, noise_spectrum: ArrayView1<f32>) -> Array1<f32> {
        let noisy_spectrum = self.apply_fft(noisy_signal);
        let mut cleaned_spectrum = noisy_spectrum.zip(&noise_spectrum).map(|(ns, n)| {
            let magnitude = (ns.re * ns.re + ns.im * ns.im).sqrt();
            let cleaned_magnitude = (magnitude - n.re).max(0.0);
            Complex::new(cleaned_magnitude * (ns.re / magnitude), cleaned_magnitude * (ns.im / magnitude))
        });
        self.apply_inverse_fft(cleaned_spectrum.view())
    }

    fn apply_inverse_fft(&self, spectrum: ArrayView1<Complex<f32>>) -> Array1<f32> {
        let mut buffer = spectrum.to_vec();
        let ifft = Fft::new(self.window_size, FftDirection::Inverse);
        ifft.process(&mut buffer);
        buffer.iter().map(|c| c.re).collect()
    }
}