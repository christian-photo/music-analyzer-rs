#[derive(Debug)]
pub struct AudioFile {
    pub channels: i16,
    pub sample_rate: i32,
    pub bits_per_sample: i16,

    pub data: Vec<f64>,
}

impl AudioFile {
    pub fn new(channels: i16, sample_rate: i32, bits_per_sample: i16, data: Vec<f64>) -> AudioFile {
        return AudioFile {
            channels,
            sample_rate,
            bits_per_sample,
            data,
        };
    }

    pub fn duration(&self) -> f32 {
        self.data.len() as f32 / self.sample_rate as f32
    }
}
