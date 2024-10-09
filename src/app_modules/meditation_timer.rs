use crate::data_analysis::meditation_timer::add_record;
use inquire::Text;
use log::{error, info};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const SECONDS_IN_MINUTE: u32 = 60;

pub fn run_meditation_timer() {
    // TODO Add inquire here to use Select, timer in 10 or 30 second intervals (#1)
    let duration: String = Text::new("Enter the duration of your meditation in minutes:")
        .prompt()
        .expect("Failed to read duration time.");

    let duration: u32 = duration
        .trim()
        .parse()
        .expect("Please enter a valid number.");

    start_timer(duration * SECONDS_IN_MINUTE);

    // TODO Add generic retry mechanism
    // TODO Analyze data with polars (#2)
    info!("Adding record to local CSV file...");
    match add_record(duration) {
        Ok(_) => info!("Data appended to CSV file."),
        Err(error) => {
            error!("Failed to add meditation record: {}", error);
            eprintln!("An error occurred while saving the meditation record to a CSV file.");
        }
    }
}

pub fn start_timer(duration: u32) {
    println!("Starting meditation timer.");
    let mut standard_output = stdout();

    for seconds in 0..=duration {
        let minutes = seconds / SECONDS_IN_MINUTE;
        let seconds_in_minute = seconds % SECONDS_IN_MINUTE;

        print!("\r{:02}:{:02}", minutes, seconds_in_minute);
        standard_output.flush().unwrap();
        sleep(Duration::from_secs(1));
    }

    println!("\nMeditation complete!");
}
