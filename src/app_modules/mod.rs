use inquire::error::InquireResult;
use inquire::Select;
use std::fmt;

mod breathing_guide;
pub(crate) mod data_analysis;
mod gratitude_journal;
mod meditation_timer;
mod mood_tracker;

#[derive(PartialEq, Eq)]
pub enum AppAction {
    Continue,
    Exit,
}

#[derive(Debug)]
enum AppModule {
    MeditationTimer,
    MoodTracker,
    GratitudeJournal,
    BreathingGuide,
    DataAnalysis,
    Exit,
}

impl fmt::Display for AppModule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppModule::MeditationTimer => write!(formatter, "Meditation Timer"),
            AppModule::MoodTracker => write!(formatter, "Mood Tracker"),
            AppModule::GratitudeJournal => write!(formatter, "Gratitude Journal"),
            AppModule::BreathingGuide => write!(formatter, "Breathing Guide"),
            AppModule::DataAnalysis => write!(formatter, "Data Analysis"),
            AppModule::Exit => write!(formatter, "Exit"),
        }
    }
}

pub fn select_module() -> AppAction {
    let module_choices = vec![
        AppModule::MeditationTimer,
        AppModule::MoodTracker,
        AppModule::GratitudeJournal,
        AppModule::BreathingGuide,
        AppModule::DataAnalysis,
        AppModule::Exit,
    ];
    let module_answer: InquireResult<AppModule> =
        Select::new("What would you like to do?", module_choices).prompt();

    match module_answer {
        Ok(choice) => match choice {
            AppModule::MeditationTimer => meditation_timer::run_meditation_timer(),
            AppModule::MoodTracker => mood_tracker::run_mood_tracker(),
            AppModule::GratitudeJournal => gratitude_journal::run_gratitude_journal(),
            AppModule::BreathingGuide => breathing_guide::run_breathing_guide(),
            AppModule::DataAnalysis => data_analysis::run_data_analysis(),
            AppModule::Exit => return AppAction::Exit,
        },
        Err(_) => println!("There was an error, please try again"),
    }

    AppAction::Continue
}
