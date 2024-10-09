use crate::project_consts::{APPLICATION_OUTPUT_DIRECTORY, MEDITATION_TIMER_LOG_FILENAME};
use chrono::Utc;
use csv::WriterBuilder;
use serde::Serialize;
use std::error::Error;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io;
use std::io::Seek;

#[derive(Serialize)]
struct MeditationData {
    timestamp: String,
    duration: u32,
}

pub fn add_record(duration: u32) -> Result<(), Box<dyn Error>> {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    create_dir_all(APPLICATION_OUTPUT_DIRECTORY).unwrap();

    let mut meditation_timer_csv_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(format!(
            "{APPLICATION_OUTPUT_DIRECTORY}{MEDITATION_TIMER_LOG_FILENAME}"
        ))?;

    let needs_headers = does_file_need_headers(&mut meditation_timer_csv_file);
    let mut writer = WriterBuilder::new()
        .has_headers(needs_headers)
        .from_writer(meditation_timer_csv_file);

    let meditation_data = MeditationData {
        timestamp,
        duration,
    };
    writer.serialize(meditation_data)?;

    writer.flush()?;
    Ok(())
}

fn does_file_need_headers(csv_file: &mut File) -> bool {
    csv_file.seek(io::SeekFrom::End(0)).unwrap() == 0
}
