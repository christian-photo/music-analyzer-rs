#[derive(Debug)]
pub struct AudioFile {
    pub channels: i16,
    pub sample_rate: i32,
    pub bits_per_sample: i16,
    pub duration: f32,

    pub data: Vec<f32>,
}

impl AudioFile {
    pub fn new(channels: i16, sample_rate: i32, bits_per_sample: i16, data: Vec<f32>) -> AudioFile {
        let duration: f32 = data.len() as f32 / sample_rate as f32;

        return AudioFile {
            channels,
            sample_rate,
            bits_per_sample,
            data,
            duration,
        };
    }
}
