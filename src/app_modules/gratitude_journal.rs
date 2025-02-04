use crate::project_consts::GRATITUDE_JOURNAL_PROMPTS;
use crate::utilities::get_date;
use inquire::{required, Editor};
use rand::seq::SliceRandom;

pub(crate) fn run_gratitude_journal() {

    // TODO inquire::Editor?
    //  If the user presses enter without ever modyfing the temporary file, it will be treated as an empty submission.
    //  If this is unwanted behavior, you can control the user input by using validators.

    let prompt = GRATITUDE_JOURNAL_PROMPTS
        .choose(&mut rand::thread_rng())
        .expect("Could not find a gratitude journal prompt.");

    let journal_entry_header: &str = &format!("# Journal Entry - {}", get_date());

    // TODO Consider following possibility:
    //  making multiple entries a day should not add new ones with a date,
    //  but instead, pop-up the previously created one so the user can edit it. Something like editing git commit message:
    //  https://stackoverflow.com/questions/56011927/how-do-i-use-rust-to-open-the-users-default-editor-and-get-the-edited-content

    let journal_entry: String = Editor::new(prompt)
        .with_predefined_text(journal_entry_header)
        .with_file_extension(".md")
        // TODO validator to ensure default header is checked too
        .with_validator(required!())
        .prompt()
        .expect("Failed to get journal entry.");

    println!("{}", journal_entry);
    // TODO store to a text file in subdirectory, instead of csv.
}
