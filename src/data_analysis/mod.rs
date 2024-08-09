use chrono::Utc;
use csv::Writer;
use std::error::Error;
use std::fs::OpenOptions;

pub fn add_record(duration: u32) -> Result<(), Box<dyn Error>> {
    // Get the current date and time
    let now = Utc::now();
    let date_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    // Path to the CSV file
    let file_path = "target/app_output/meditation_timer_log.csv";

    let meditation_timer_csv_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let mut writer = Writer::from_writer(meditation_timer_csv_file);

    // TODO Check if file exists before attaching headers?
    writer.write_record(&["timestamp", "duration"])?;
    writer.write_record(&[date_time_str, duration.to_string()])?;
    writer.flush()?;

    println!("Data appended to CSV file.");

    Ok(())
}
