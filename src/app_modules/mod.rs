use std::fmt;
use inquire::{InquireError, Select};
use std::str::FromStr;
use inquire::error::InquireResult;

mod meditation_timer;
mod mood_tracker;
pub(crate) mod data_analysis;

#[derive(Debug)]
enum AppModule {
    Timer,
    MoodTracker,
    DataAnalysis,
}

impl fmt::Display for AppModule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppModule::Timer => write!(formatter, "Timer"),
            AppModule::MoodTracker => write!(formatter, "Mood Tracker"),
            AppModule::DataAnalysis => write!(formatter, "Data Analysis"),
        }
    }
}


pub fn select_module() {
    let module_choices = vec![AppModule::Timer, AppModule::MoodTracker, AppModule::DataAnalysis];
    let module_answer: InquireResult<AppModule> =
        Select::new("What would you like to do?", module_choices).prompt();

    match module_answer {
        Ok(choice) => match choice {
            AppModule::Timer => meditation_timer::run_meditation_timer(), 
            AppModule::MoodTracker => mood_tracker::run_mood_tracker(),
            AppModule::DataAnalysis => data_analysis::run_data_analysis(),
        },
        Err(_) => println!("There was an error, please try again"),
    }
}
