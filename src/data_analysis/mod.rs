use crate::project_consts::{APPLICATION_OUTPUT_DIRECTORY, MEDITATION_TIMER_LOG_FILENAME};
use chrono::Utc;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Seek;
use serde::Serialize;
use log::info;

#[derive(Serialize)]
struct MeditationData {
    timestamp: String,
    duration: u32,
}

pub fn add_record(duration: u32) -> Result<(), Box<dyn Error>> {
    info!("Adding record to local CSV file...");
    let now = Utc::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut meditation_timer_csv_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(format!(
            "{APPLICATION_OUTPUT_DIRECTORY}{MEDITATION_TIMER_LOG_FILENAME}"
        ))?;

    let needs_headers = meditation_timer_csv_file.seek(std::io::SeekFrom::End(0))? == 0;
    let mut writer = WriterBuilder::new()
        .has_headers(needs_headers)
        .from_writer(meditation_timer_csv_file);

    writer.serialize(MeditationData {
        timestamp,
        duration,
    })?;

    writer.flush()?;
    info!("Data appended to CSV file.");
    Ok(())
}
