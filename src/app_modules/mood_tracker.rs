use std::fmt;
use inquire::Select;
use inquire::error::InquireResult;

pub(crate) fn run_mood_tracker() {
    // TODO This should be a simple 1-5 selector at the beginning (ideally with emojis) saved to CSV file.
    // TODO later, consider SQLite and more options.
    // TODO Idea: user is pestered with questions if empty line is sent then the program stops early
    
    let mood_options = vec![
        Mood { emoji: "😭", rating: 1 },
        Mood { emoji: "😔", rating: 2 },
        Mood { emoji: "😐", rating: 3 },
        Mood { emoji: "😊", rating: 4 },
        Mood { emoji: "😄", rating: 5 },
    ];
    
    let user_mood: InquireResult<Mood> =
        Select::new("How would you rate your mood?\n", mood_options)
            .with_help_message("↑↓ to move, enter to select")
            .with_starting_cursor(2)
            .without_filtering()
            .prompt();
    
    println!("{:?}", user_mood);

}


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