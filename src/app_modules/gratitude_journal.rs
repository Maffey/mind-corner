use crate::project_consts::{
    APPLICATION_OUTPUT_DIRECTORY, GRATITUDE_JOURNAL_DIRECTORY_NAME, GRATITUDE_JOURNAL_PROMPTS,
};
use crate::utilities::get_date;
use inquire::{required, Editor};
use rand::prelude::IndexedRandom;
use std::fs;
use std::path::Path;

const JOURNAL_FILE_SUFFIX: &str = ".md";

pub(crate) fn run_gratitude_journal() {
    let prompt = GRATITUDE_JOURNAL_PROMPTS
        .choose(&mut rand::rng())
        .expect("Could not find a random gratitude journal prompt.");

    let today: String = get_date();
    let journal_filename: String = get_journal_filename(&today);
    let journal_entry_header: String = get_journal_entry_header(today);
    let journal_file_path: String = get_journal_file_path(journal_filename);
    let journal_file_path: &Path = Path::new(&journal_file_path);
    let journal_text: String = if journal_file_path.exists() {
        read_journal_entry(journal_file_path)
    } else {
        journal_entry_header
    };

    let new_journal_text: String = Editor::new(prompt)
        .with_predefined_text(&journal_text)
        .with_file_extension(JOURNAL_FILE_SUFFIX)
        .with_validator(required!())
        .prompt()
        .expect("Failed to get journal entry.");

    write_journal_entry_to_file(new_journal_text, journal_file_path);
}

fn write_journal_entry_to_file(journal_entry: String, journal_file_path: &Path) {
    let journal_directory = journal_file_path
        .parent()
        .expect("Could not get parent directory of journal file..");
    fs::create_dir_all(journal_directory).expect("Could not create journal directory.");
    fs::write(journal_file_path, journal_entry).expect("Unable to write journal entry file.");
}

fn read_journal_entry(journal_entry_path: &Path) -> String {
    fs::read_to_string(journal_entry_path).expect("Could not read journal entry file.")
}

fn get_journal_filename(file_name: &str) -> String {
    format!("{}{}", file_name, JOURNAL_FILE_SUFFIX)
}

fn get_journal_entry_header(header_text: String) -> String {
    format!("# Journal Entry - {}", header_text)
}

fn get_journal_file_path(journal_filename: String) -> String {
    format!(
        "{}{}{}",
        APPLICATION_OUTPUT_DIRECTORY, GRATITUDE_JOURNAL_DIRECTORY_NAME, journal_filename
    )
}

#[cfg(test)]
mod tests {
    use crate::app_modules::gratitude_journal::{get_journal_filename, JOURNAL_FILE_SUFFIX};

    #[test]
    fn journal_filename_is_correct() {
        let expected_filename = format!("file_name{}", JOURNAL_FILE_SUFFIX);
        assert_eq!(get_journal_filename("file_name"), expected_filename);
    }
}
