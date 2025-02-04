use crate::data::data_collection::does_file_need_headers;
use crate::project_consts::{APPLICATION_OUTPUT_DIRECTORY, MOOD_TRACKER_LOG_FILENAME};
use crate::utilities::get_timestamp;
use csv::WriterBuilder;
use serde::Serialize;
use std::error::Error;
use std::fs::{create_dir_all, OpenOptions};

#[derive(Serialize)]
struct MoodData {
    timestamp: String,
    mood_value: u8,
}

pub fn add_mood_record(mood_value: u8) -> Result<(), Box<dyn Error>> {
    let timestamp: String = get_timestamp();

    create_dir_all(APPLICATION_OUTPUT_DIRECTORY).unwrap();

    let mut mood_tracker_csv_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(format!(
            "{APPLICATION_OUTPUT_DIRECTORY}{MOOD_TRACKER_LOG_FILENAME}"
        ))?;

    let needs_headers: bool = does_file_need_headers(&mut mood_tracker_csv_file);
    let mut writer = WriterBuilder::new()
        .has_headers(needs_headers)
        .from_writer(mood_tracker_csv_file);

    let mood_data = MoodData {
        timestamp,
        mood_value,
    };
    writer.serialize(mood_data)?;

    writer.flush()?;
    Ok(())
}
