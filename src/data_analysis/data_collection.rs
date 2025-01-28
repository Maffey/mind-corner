use chrono::Utc;
use std::fs::File;
use std::io;
use std::io::Seek;

pub(crate) fn get_timestamp() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub(crate) fn does_file_need_headers(csv_file: &mut File) -> bool {
    csv_file.seek(io::SeekFrom::End(0)).unwrap() == 0
}
