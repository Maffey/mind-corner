use crate::data::meditation_timer::add_meditation_record;
use crate::utilities::print_in_place;
use inquire::validator::Validation;
use inquire::Text;
use log::{error, info};
use std::io::{stdout, Stdout};
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

    let duration_in_minutes: u32 = get_parsed_duration(duration);

    start_timer(duration_in_minutes * SECONDS_IN_MINUTE);

    info!("Adding record to local CSV file...");
    match add_meditation_record(duration_in_minutes) {
        Ok(_) => info!("Meditation data appended to CSV file."),
        Err(error) => {
            error!("Failed to add mood record: {}", error);
            eprintln!("An error occurred while saving the mood record to a CSV file.");
        }
    }
}

fn get_parsed_duration(duration: String) -> u32 {
    duration.trim().parse().unwrap_or(DEFAULT_MINUTES_TIMER)
}

fn start_timer(duration: u32) {
    println!("Starting meditation timer.");
    let stdout: Stdout = stdout();

    for seconds in 0..=duration {
        let minutes = seconds / SECONDS_IN_MINUTE;
        let seconds_in_minute = seconds % SECONDS_IN_MINUTE;

        print_in_place(&stdout, format!("{:02}:{:02}", minutes, seconds_in_minute));
        sleep(Duration::from_secs(1));
    }

    println!("\nMeditation complete!");
}
#[cfg(test)]
mod tests {
    use crate::app_modules::meditation_timer::{get_parsed_duration, DEFAULT_MINUTES_TIMER};
    use rstest::rstest;

    #[rstest]
    #[case::parse_number_properly("10", 10)]
    #[case::no_input_default_minutes("", DEFAULT_MINUTES_TIMER)]
    #[case::random_input_default_minutes("i dont understand numbers", DEFAULT_MINUTES_TIMER)]
    fn duration_is_parsed_correctly(#[case] duration_input: &str, #[case] expected_duration: u32) {
        assert_eq!(
            get_parsed_duration(duration_input.to_string()),
            expected_duration
        );
    }
}
