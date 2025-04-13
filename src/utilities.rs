use chrono::{DateTime, Local, TimeZone};
use std::fmt::Display;
use std::io::{Stdout, Write};

const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const DATE_FORMAT: &str = "%Y-%m-%d";

pub(crate) fn get_timestamp() -> String {
    get_formatted_datetime(Local::now(), DATE_TIME_FORMAT)
}

pub(crate) fn get_date() -> String {
    get_formatted_datetime(Local::now(), DATE_FORMAT)
}

fn get_formatted_datetime<T>(time: DateTime<T>, format: &str) -> String
where
    T: TimeZone,
    T::Offset: Display,
{
    time.format(format).to_string()
}

pub(crate) fn print_in_place(mut standard_output: &Stdout, text: String) {
    print!("\r{}", text);
    standard_output.flush().expect("Failed to flush stdout.");
}

#[cfg(test)]
mod tests {
    use crate::utilities::{get_formatted_datetime, DATE_FORMAT, DATE_TIME_FORMAT};
    use chrono::{TimeZone, Utc};

    #[test]
    fn formatted_date_time_is_correct() {
        let date_time = Utc.with_ymd_and_hms(2025, 2, 7, 18, 55, 17).unwrap();
        assert_eq!(
            get_formatted_datetime(date_time, DATE_TIME_FORMAT),
            "2025-02-07 18:55:17"
        );
    }

    #[test]
    fn formatted_date_is_correct() {
        let date = Utc.with_ymd_and_hms(2025, 2, 7, 0, 0, 0).unwrap();
        assert_eq!(get_formatted_datetime(date, DATE_FORMAT), "2025-02-07");
    }
}
