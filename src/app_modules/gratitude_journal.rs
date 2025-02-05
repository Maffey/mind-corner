use std::fs;
use crate::project_consts::{APPLICATION_OUTPUT_DIRECTORY, GRATITUDE_JOURNAL_DIRECTORY_NAME, GRATITUDE_JOURNAL_PROMPTS};
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

    let today: String = get_date();
    let journal_entry_header: &str = &format!("# Journal Entry - {}",today);

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

    write_journal_entry_to_file(journal_entry, today);
}

fn write_journal_entry_to_file(journal_entry: String, date: String) {
    let journal_filename = format!("{}.md", date);
    let journal_directory_path = format!(
        "{}{}",
        APPLICATION_OUTPUT_DIRECTORY, GRATITUDE_JOURNAL_DIRECTORY_NAME
    );
    let journal_entry_path = format!(
        "{}{}",
        journal_directory_path, journal_filename
    );
    fs::create_dir_all(journal_directory_path).expect("Could not create journal directory.");
    fs::write(journal_entry_path, journal_entry).expect("Unable to write journal entry file.");
}