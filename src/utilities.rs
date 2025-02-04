use chrono::Utc;

pub(crate) fn get_timestamp() -> String {
    get_now("%Y-%m-%d %H:%M:%S")
}

pub(crate) fn get_date() -> String {
    get_now("%Y-%m-%d")
}

fn get_now(format: &str) -> String {
    Utc::now().format(format).to_string()
}