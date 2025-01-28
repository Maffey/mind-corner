use crate::data_analysis::meditation_timer::add_record;
use inquire::validator::Validation;
use inquire::Text;
use log::{error, info};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const SECONDS_IN_MINUTE: u32 = 60;
const DEFAULT_MINUTES_TIMER: u32 = 5;

pub(crate) fn run_meditation_timer() {
    let is_number_validator = |input: &str| match input.chars().all(|c| c.is_numeric()) {
        true => Ok(Validation::Valid),
        false => Ok(Validation::Invalid(
            "Please enter a valid natural number.".into(),
        )),
    };

    let duration: String = Text::new("Enter the duration of your meditation in minutes:")
        .with_default(&DEFAULT_MINUTES_TIMER.to_string())
        .with_validator(is_number_validator)
        .prompt()
        .expect("Failed to read duration time.");

    let duration_in_minutes: u32 = duration
        .trim()
        .parse()
        .unwrap_or_else(|_| DEFAULT_MINUTES_TIMER);

    start_timer(duration_in_minutes * SECONDS_IN_MINUTE);

    // TODO Analyze data with polars (#2)
    info!("Adding record to local CSV file...");
    match add_record(duration_in_minutes) {
        Ok(_) => info!("Data appended to CSV file."),
        Err(error) => {
            error!("Failed to add meditation record: {}", error);
            eprintln!("An error occurred while saving the meditation record to a CSV file.");
        }
    }
}

fn start_timer(duration: u32) {
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
