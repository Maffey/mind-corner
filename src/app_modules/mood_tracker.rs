use inquire::{InquireError, Select};

pub(crate) fn run_mood_tracker() {
    println!("WIP");
    // TODO This should be a simple 1-5 selector at the beginning (ideally with emojis) saved to CSV file.
    // TODO later, consider SQLite and more options.
    // TODO Idea: user is pestered with questions if empty line is sent then the program stops early
    let mood_options = vec!["😭","😔", "😐", "😊", "😄"];
    let user_mood: Result<&str, InquireError> =
        Select::new("How would you rate your mood?", mood_options).prompt();
    println!("{:?}", user_mood);

}
