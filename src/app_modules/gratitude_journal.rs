use crate::project_consts::{APPLICATION_OUTPUT_DIRECTORY, GRATITUDE_JOURNAL_DIRECTORY_NAME, GRATITUDE_JOURNAL_PROMPTS};
use crate::utilities::get_date;
use inquire::{required, Editor};
use std::fs;
use rand::prelude::IndexedRandom;

pub(crate) fn run_gratitude_journal() {
    let prompt = GRATITUDE_JOURNAL_PROMPTS
        .choose(&mut rand::rng())
        .expect("Could not find a gratitude journal prompt.");

    let today: String = get_date();
    let journal_entry_header: &str = &format!("# Journal Entry - {}",today);

    // TODO Consider following possibility: (#11)
    //  making multiple entries a day should not add new ones with a date,
    //  but instead, pop-up the previously created one so the user can edit it.
    let journal_entry: String = Editor::new(prompt)
        .with_predefined_text(journal_entry_header)
        .with_file_extension(".md")
        // TODO validator to ensure default header is checked too (#11)
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