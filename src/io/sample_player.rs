use rodio::{OutputStream, Source};
use std::time::Duration;

use super::audio_file::AudioFile;

// Implement a custom source that produces samples from an array
pub struct SamplesPlayer {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    pub current_index: usize,
}

impl Iterator for SamplesPlayer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.samples.len() {
            None
        } else {
            let sample = self.samples[self.current_index];
            self.current_index += 1;
            Some(sample)
        }
    }
}

impl Source for SamplesPlayer {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.samples.len() - self.current_index)
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.samples.len() as f32 / self.sample_rate as f32,
        ))
    }
}

impl SamplesPlayer {
    pub fn play(file: &AudioFile) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Create a custom source from the borrowed samples
        let source = SamplesPlayer {
            samples: file.data.clone(),
            sample_rate: file.sample_rate as u32,
            channels: file.channels as u16,
            current_index: 0,
        };

        // Play the custom source
        std::thread::sleep(Duration::from_millis(500)); // neeeded for some reason, because rodio doesn't play properly otherwise
        stream_handle.play_raw(source).unwrap();
        std::thread::sleep(Duration::from_secs_f32(file.duration));
    }
}
