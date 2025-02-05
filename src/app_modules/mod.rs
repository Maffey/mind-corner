use inquire::error::InquireResult;
use inquire::Select;
use std::fmt;

pub(crate) mod data_analysis;
mod gratitude_journal;
mod meditation_timer;
mod mood_tracker;

#[derive(PartialEq, Eq)]
pub enum AppAction {
    Continue,
    Exit
}

#[derive(Debug)]
enum AppModule {
    Timer,
    MoodTracker,
    GratitudeJournal,
    DataAnalysis,
    Exit,
}



impl fmt::Display for AppModule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppModule::Timer => write!(formatter, "Timer"),
            AppModule::MoodTracker => write!(formatter, "Mood Tracker"),
            AppModule::GratitudeJournal => write!(formatter, "Gratitude Journal"),
            AppModule::DataAnalysis => write!(formatter, "Data Analysis"),
            AppModule::Exit => write!(formatter, "Exit"),
        }
    }
}

pub fn select_module() -> AppAction {
    let module_choices = vec![
        AppModule::Timer,
        AppModule::MoodTracker,
        AppModule::GratitudeJournal,
        AppModule::DataAnalysis,
        AppModule::Exit,
    ];
    let module_answer: InquireResult<AppModule> =
        Select::new("What would you like to do?", module_choices).prompt();

    match module_answer {
        Ok(choice) => match choice {
            AppModule::Timer => meditation_timer::run_meditation_timer(),
            AppModule::MoodTracker => mood_tracker::run_mood_tracker(),
            AppModule::GratitudeJournal => gratitude_journal::run_gratitude_journal(),
            AppModule::DataAnalysis => data_analysis::run_data_analysis(),
            AppModule::Exit => return AppAction::Exit,
        },
        Err(_) => println!("There was an error, please try again"),
    }
    
    AppAction::Continue
}
