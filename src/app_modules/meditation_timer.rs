use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const SECONDS_IN_MINUTE: u32 = 60;

pub fn run_meditation_timer() {
    // TODO Take time input
    let meditation_time: u32 = 5;

    let mut standard_output = stdout();

    for seconds in 0..=meditation_time {
        let minutes = seconds / SECONDS_IN_MINUTE;
        let seconds_in_minute = seconds % SECONDS_IN_MINUTE;

        print!("\r{:02}:{:02}", minutes, seconds_in_minute);
        standard_output.flush().unwrap();
        sleep(Duration::from_secs(1));
    }

    println!("\nMeditation complete!");
}
