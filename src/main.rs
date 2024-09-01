use std::{fmt::Result, path::Path};

use io::{audio_file::AudioFile, sample_player::SamplesPlayer};
use plotters::prelude::*;

mod io;

fn main() {
    // let path: &Path = Path::new("./test-files/Test-16bit-PCM-Mono-96khz.wav");
    // let path: &Path = Path::new("./test-files/Test-32bit-float-Mono.wav");
    // let path: &Path = Path::new("./test-files/Test-32bit-PCM-Mono.wav");
    let path: &Path = Path::new("./test-files/Test-8bit-PCM-Mono-unsigned.wav");

    let file: AudioFile = AudioFile::load_wav_file(path.to_str().unwrap()).unwrap();

    make_chart(&file.data).unwrap();

    SamplesPlayer::play(&file);
}

fn make_chart(data: &Vec<f32>) -> Result {
    // Use plotters to plot file.data
    let root = BitMapBackend::new("output.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Audio Sample Data", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..data.len(), -1.0..1.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            data.iter().enumerate().map(|(x, y)| (x, *y as f64)),
            &RED,
        ))
        .unwrap();

    root.present().unwrap();

    Ok(())
}
