use inquire::{InquireError, Select};
use std::str::FromStr;

mod meditation_timer;
mod mood_tracker;

#[derive(Debug)]
enum AppModule {
    Timer,
    MoodTracker,
}

impl FromStr for AppModule {
    type Err = ();
    fn from_str(choice: &str) -> Result<AppModule, Self::Err> {
        match choice {
            "Timer" => Ok(AppModule::Timer),
            "Mood Tracker" => Ok(AppModule::MoodTracker),
            _ => Err(()),
        }
    }
}

pub fn select_module() {
    let module_choices = vec!["Timer", "Mood Tracker"];
    let module_answer: Result<&str, InquireError> =
        Select::new("What would you like to do?", module_choices).prompt();

    match module_answer {
        Ok(choice) => match AppModule::from_str(choice) {
            Ok(AppModule::Timer) => meditation_timer::run_meditation_timer(),
            Ok(AppModule::MoodTracker) => mood_tracker::run_mood_tracker(),
            Err(_) => panic!("This is impossible, how did this happen? We are smarter than this!"),
        },
        Err(_) => println!("There was an error, please try again"),
    }
}
