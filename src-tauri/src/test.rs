use std::f32::consts::PI;

pub fn analyze_audio(samples: &[f32]) -> (f32, f32) {
    let mut sum = 0.0;
    let mut sum_of_squares = 0.0;

    for &sample in samples {
        sum += sample;
        sum_of_squares += sample * sample;
    }

    let mean = sum / samples.len() as f32;
    let rms = (sum_of_squares / samples.len() as f32).sqrt();

    (mean, rms)
}

pub fn calculate_frequency(samples: &[f32], sample_rate: u32) -> f32 {
    let mut max_correlation = 0.0;
    let mut best_period = 0;

    for period in 1..samples.len() / 2 {
        let mut correlation = 0.0;
        for i in 0..samples.len() - period {
            correlation += samples[i] * samples[i + period];
        }
        if correlation > max_correlation {
            max_correlation = correlation;
            best_period = period;
        }
    }

    if best_period == 0 {
        return 0.0;
    }

    sample_rate as f32 / best_period as f32
}

pub fn apply_fft(samples: &mut [f32]) {
    let n = samples.len();
    if n <= 1 {
        return;
    }

    let mut even = Vec::new();
    let mut odd = Vec::new();

    for i in 0..n {
        if i % 2 == 0 {
            even.push(samples[i]);
        } else {
            odd.push(samples[i]);
        }
    }

    apply_fft(&mut even);
    apply_fft(&mut odd);

    for k in 0..n / 2 {
        let t = (2.0 * PI * k as f32 / n as f32).cos() * odd[k] - (2.0 * PI * k as f32 / n as f32).sin() * even[k];
        samples[k] = even[k] + t;
        samples[k + n / 2] = even[k] - t;
    }
}