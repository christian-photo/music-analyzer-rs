use std::io::*;

use super::audio_file::AudioFile;

#[derive(PartialEq)]
pub enum WaveFormatType {
    Unknown = 0x0000,
    PCM = 0x0001,
    IEEEFloat = 0x0003,
    ALAW = 0x0006,
    MULAW = 0x0007,
    EXTENSIBLE = 0xFFFE,
}

impl AudioFile {
    pub fn load_wav_file(path: &str) -> Result<AudioFile> {
        let file: Result<std::fs::File> = std::fs::File::open(path); // Open the wave file

        if file.is_err() {
            // Check if the wave file was successfully loaded, if not return the error
            return Err(file.unwrap_err());
        }

        let mut channels: i16 = 0;
        let mut sample_rate: i32 = 0;
        let mut bits_per_sample: i16 = 0;
        let mut samples: Vec<f32> = Vec::new();

        let mut reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file.unwrap()); // Create a reader for the wave file

        let mut riff_buf: [u8; 4] = [0; 4];
        let mut res: std::result::Result<usize, Error> = reader.read(&mut riff_buf);
        if res.is_ok() {
            if &riff_buf != b"RIFF" {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "File is not a wave file",
                ));
            }
        } else {
            return Err(res.unwrap_err());
        }

        let mut format_buf: [u8; 2] = [0; 2];
        let mut format: WaveFormatType = WaveFormatType::Unknown;

        reader.seek(SeekFrom::Start(0x14)).unwrap(); // size -> format
        res = reader.read(&mut format_buf);
        if res.is_ok() {
            format = match i16::from_le_bytes(format_buf) {
                1 => WaveFormatType::PCM,
                3 => WaveFormatType::IEEEFloat,
                6 => WaveFormatType::ALAW,
                _ => WaveFormatType::Unknown,
            }
        }

        let mut n_channel_buf: [u8; 2] = [0; 2];
        reader.seek(SeekFrom::Start(0x16)).unwrap(); // size -> channels
        res = reader.read(&mut n_channel_buf);
        if res.is_ok() {
            channels = i16::from_le_bytes(n_channel_buf);
        }

        let mut sample_rate_buf: [u8; 4] = [0; 4];
        reader.seek(SeekFrom::Start(0x18)).unwrap(); // channels -> sample rate
        res = reader.read(&mut sample_rate_buf);
        if res.is_ok() {
            sample_rate = i32::from_le_bytes(sample_rate_buf);
        }

        let mut bits_per_sample_buf: [u8; 2] = [0; 2];
        reader.seek(SeekFrom::Start(0x22)).unwrap(); // sample rate -> bits per sample
        res = reader.read(&mut bits_per_sample_buf);
        if res.is_ok() {
            bits_per_sample = i16::from_le_bytes(bits_per_sample_buf);
        }

        let mut data_size_buf: [u8; 4] = [0; 4];
        let mut size_pos: u64 = 0;
        if format == WaveFormatType::PCM {
            size_pos = 0x28;
        } else if format == WaveFormatType::IEEEFloat {
            size_pos = 0x4C;
        }
        reader.seek(SeekFrom::Start(size_pos)).unwrap(); // bits per sample -> data
        res = reader.read(&mut data_size_buf);
        if res.is_ok() {
            let data_size = i32::from_le_bytes(data_size_buf); // first read the size of the data
            if data_size < 0 {
                // No data inside file
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "File is not a wave file",
                ));
            }
            let mut data: Vec<u8> = vec![0; data_size as usize]; // read to the end of the file -> all data
            reader.seek(SeekFrom::Start(size_pos + 4)).unwrap();
            res = reader.read(&mut data);
            if res.is_err() {
                return Err(res.unwrap_err());
            }

            if format == WaveFormatType::PCM {
                samples = AudioFile::pcm_to_samples(bits_per_sample, &data);
            } else if format == WaveFormatType::IEEEFloat {
                samples = AudioFile::ieee_float_to_samples(bits_per_sample, &data);
            }
        }
        return Ok(AudioFile::new(
            channels,
            sample_rate,
            bits_per_sample,
            samples,
        ));
    }

    fn ieee_float_to_samples(bits_per_sample: i16, data: &Vec<u8>) -> Vec<f32> {
        let mut samples: Vec<f32> = Vec::new();

        match bits_per_sample {
            32 => {
                // Convert 32-bit IEEE float samples to f32
                for i in 0..(data.len() / 4) {
                    let sample = f32::from_le_bytes([
                        data[i * 4],
                        data[i * 4 + 1],
                        data[i * 4 + 2],
                        data[i * 4 + 3],
                    ]);
                    samples.push(sample);
                }
            }
            64 => {
                // Convert 64-bit IEEE float samples to f32
                for i in 0..(data.len() / 8) {
                    let sample = f64::from_le_bytes([
                        data[i * 8],
                        data[i * 8 + 1],
                        data[i * 8 + 2],
                        data[i * 8 + 3],
                        data[i * 8 + 4],
                        data[i * 8 + 5],
                        data[i * 8 + 6],
                        data[i * 8 + 7],
                    ]) as f32;
                    samples.push(sample);
                }
            }
            _ => {}
        }

        return samples;
    }

    fn pcm_to_samples(bits_per_sample: i16, data: &Vec<u8>) -> Vec<f32> {
        let mut samples: Vec<f32> = Vec::new();

        match bits_per_sample {
            8 => {
                // Convert unsigned 8-bit samples to f32 and normalize to -1.0 to 1.0
                let norm_factor = 1f32 / 128f32; // 2^7
                for &byte in data.iter() {
                    // Convert 0-255 to -1.0 to 1.0
                    samples.push((byte as f32 - 128f32) * norm_factor);
                }
            }
            16 => {
                // Convert signed 16-bit samples to f32 and normalize to -1.0 to 1.0
                let norm_factor = 1f32 / 32768f32; // 2^15
                for i in 0..(data.len() / 2) {
                    let sample = i16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
                    samples.push(sample as f32 * norm_factor); // Normalize from -32768 to 32767 to -1.0 to 1.0
                }
            }
            32 => {
                // Assuming the data is 32-bit floating point samples, use from_le_bytes directly
                let norm_factor = 1f32 / 2147483648f32; // 2^31
                for i in 0..(data.len() / 4) {
                    let sample = i32::from_le_bytes([
                        data[i * 4],
                        data[i * 4 + 1],
                        data[i * 4 + 2],
                        data[i * 4 + 3],
                    ]);
                    samples.push(sample as f32 * norm_factor); // Directly use f32 samples
                }
            }
            _ => {
                // Unsupported bit depth
                eprintln!("Unsupported bits per sample: {}", bits_per_sample);
            } // TODO: Add support for 24-bit and 32-bit floating point samples
        }

        return samples;
    }
}
