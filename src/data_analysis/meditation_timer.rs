use crate::data_analysis::data_collection::{does_file_need_headers, get_timestamp};
use crate::project_consts::{APPLICATION_OUTPUT_DIRECTORY, MEDITATION_TIMER_LOG_FILENAME};
use csv::WriterBuilder;
use serde::Serialize;
use std::error::Error;
use std::fs::{create_dir_all, OpenOptions};

#[derive(Serialize)]
struct MeditationData {
    timestamp: String,
    duration: u32,
}

pub fn add_meditation_record(duration: u32) -> Result<(), Box<dyn Error>> {
    let timestamp: String = get_timestamp();

    create_dir_all(APPLICATION_OUTPUT_DIRECTORY).unwrap();

    let mut meditation_timer_csv_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(format!(
            "{APPLICATION_OUTPUT_DIRECTORY}{MEDITATION_TIMER_LOG_FILENAME}"
        ))?;

    let needs_headers: bool = does_file_need_headers(&mut meditation_timer_csv_file);
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
