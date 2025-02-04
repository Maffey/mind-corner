use crate::project_consts::GRATITUDE_JOURNAL_PROMPTS;
use crate::utilities::get_date;
use inquire::{required, Editor};
use rand::seq::SliceRandom;

pub(crate) fn run_gratitude_journal() {
    // TODO Follow the same flow for now - store data into a csv file(??)
    //  Ask a nice question "What are you grateful for today?" Have 5 random qusetions like that.
    // TODO Consider following possibility:
    //  making multiple entries a day should not add new ones with a timestamp,
    //  but instead, pop-up the previously created one so the user can edit it. Something like editing git commit message:
    //  https://stackoverflow.com/questions/56011927/how-do-i-use-rust-to-open-the-users-default-editor-and-get-the-edited-content

    // TODO inquire::Editor?
    //  If the user presses enter without ever modyfing the temporary file, it will be treated as an empty submission.
    //  If this is unwanted behavior, you can control the user input by using validators.

    let prompt = GRATITUDE_JOURNAL_PROMPTS
        .choose(&mut rand::thread_rng())
        .expect("Could not find a gratitude journal prompt.");

    let journal_entry_header: &str = &format!("# Journal Entry - {}", get_date());

    let journal_entry = Editor::new(prompt)
        .with_predefined_text(journal_entry_header)
        .with_file_extension(".md")
        .with_validator(required!())
        .prompt()
        .expect("Failed to get journal entry.");

    println!("{}", journal_entry);
}
