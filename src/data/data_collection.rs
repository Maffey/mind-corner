use std::fs::File;
use std::io;
use std::io::Seek;

pub(crate) fn does_file_need_headers(csv_file: &mut File) -> bool {
    csv_file
        .seek(io::SeekFrom::End(0))
        .expect("Failed to read CSV file length.")
        == 0
}
