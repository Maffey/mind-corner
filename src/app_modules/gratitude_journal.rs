use crate::project_consts::{APPLICATION_OUTPUT_DIRECTORY, GRATITUDE_JOURNAL_DIRECTORY_NAME, GRATITUDE_JOURNAL_PROMPTS};
use crate::utilities::get_date;
use inquire::{required, Editor};
use std::fs;
use std::path::Path;
use rand::prelude::IndexedRandom;

const JOURNAL_FILE_SUFFIX: &str = ".md";

pub(crate) fn run_gratitude_journal() {
    let prompt = GRATITUDE_JOURNAL_PROMPTS
        .choose(&mut rand::rng())
        .expect("Could not find a random gratitude journal prompt.");

    let today: String = get_date();
    let journal_filename: String = get_journal_filename(&today);
    let journal_entry_header: String = format!("# Journal Entry - {}", today);
    let journal_file_path = format!(
        "{}{}{}",
        APPLICATION_OUTPUT_DIRECTORY, GRATITUDE_JOURNAL_DIRECTORY_NAME, journal_filename
    );
    let journal_file_path = Path::new(&journal_file_path);
    let journal_text: String;
    if journal_file_path.exists() {
        journal_text = read_journal_entry(journal_file_path);
    } else {
        journal_text = journal_entry_header;
    }
    
    let new_journal_text: String = Editor::new(prompt)
        .with_predefined_text(&journal_text)
        .with_file_extension(JOURNAL_FILE_SUFFIX)
        .with_validator(required!())
        .prompt()
        .expect("Failed to get journal entry.");

    write_journal_entry_to_file(new_journal_text, journal_file_path);
}

fn write_journal_entry_to_file(journal_entry: String, journal_file_path: &Path) {
    let journal_directory = journal_file_path.parent().expect("Could not get parent directory of journal file..");
    fs::create_dir_all(journal_directory).expect("Could not create journal directory.");
    fs::write(journal_file_path, journal_entry).expect("Unable to write journal entry file.");
}

fn read_journal_entry(journal_entry_path: &Path) -> String {
    fs::read_to_string(journal_entry_path).expect("Could not read journal entry file.")
}

fn get_journal_filename(date: &str) -> String {
    format!("{}{}", date, JOURNAL_FILE_SUFFIX)
}