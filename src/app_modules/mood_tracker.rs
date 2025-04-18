use crate::data::mood_tracker::add_mood_record;
use inquire::Select;
use log::{error, info};
use std::fmt;

#[derive(Debug)]
struct Mood {
    emoji: &'static str,
    rating: u8,
}

impl fmt::Display for Mood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.emoji, self.rating)
    }
}

pub(crate) fn run_mood_tracker() {
    // TODO consider more info to track - what you were doing, short description of the day (#14)
    //  Idea: user is pestered with questions if empty line is sent then the program stops early

    let mood_options = vec![
        Mood {
            emoji: "😭",
            rating: 1,
        },
        Mood {
            emoji: "😔",
            rating: 2,
        },
        Mood {
            emoji: "😐",
            rating: 3,
        },
        Mood {
            emoji: "😊",
            rating: 4,
        },
        Mood {
            emoji: "😄",
            rating: 5,
        },
    ];

    let user_mood: Mood = Select::new("How would you rate your mood?\n", mood_options)
        .with_help_message("↑↓ to move, enter to select mood")
        .with_starting_cursor(2)
        .without_filtering()
        .prompt()
        .expect("Failed to select mood.");

    // TODO refactor into common code (#14)
    info!("Adding record to local CSV file...");
    match add_mood_record(user_mood.rating) {
        Ok(_) => info!("Mood data appended to CSV file."),
        Err(error) => {
            error!("Failed to add meditation record: {}", error);
            eprintln!("An error occurred while saving the meditation record to a CSV file.");
        }
    }
}
