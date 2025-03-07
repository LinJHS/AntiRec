use std::f32::consts::PI;
use rodio::{source::Source, buffer::SamplesBuffer};
use rand::Rng;

pub struct DisturbedAudioSource {
    source: Box<dyn Source<Item = f32> + Send>,
    disturbance_level: f32,
    sample_rate: u32,
}

impl DisturbedAudioSource {
    pub fn new(source: Box<dyn Source<Item = f32> + Send>, disturbance_level: f32) -> Self {
        let sample_rate = source.sample_rate();
        Self {
            source,
            disturbance_level,
            sample_rate,
        }
    }
}

impl Iterator for DisturbedAudioSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.source.next() {
            let mut rng = rand::thread_rng();
            let disturbance = rng.gen_range(-self.disturbance_level..self.disturbance_level);
            Some(sample + disturbance)
        } else {
            None
        }
    }
}

impl Source for DisturbedAudioSource {
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.total_duration()
    }
}

pub fn generate_sine_wave(frequency: f32, sample_rate: u32, duration: f32) -> SamplesBuffer<f32> {
    let mut samples = Vec::new();
    let num_samples = (duration * sample_rate as f32) as usize;
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (2.0 * PI * frequency * t).sin();
        samples.push(sample);
    }
    SamplesBuffer::new(1, sample_rate, samples)
}

pub fn process_audio(source: DisturbedAudioSource) -> Vec<f32> {
    let mut processed_samples = Vec::new();
    for sample in source {
        processed_samples.push(sample);
    }
    processed_samples
}