use crate::utilities::print_in_place;
use indicatif::ProgressBar;
use std::io::{stdout, Stdout};
use std::{thread, time};

const SECONDS_OF_SLEEP_INTERVAL: u64 = 1;
const PROGRESS_BAR_INTERVAL: u64 = 1;
const TIME_TO_GET_READY: u8 = 3;

// TODO enum for operations and struct or fancy enum for different breathing techniques?

pub(crate) fn run_breathing_guide() {
    // TODO store data of type and duration, to csv

    // TODO Add option to interrupt it.
    // TODO clear text every stage, construct separate progress bars?

    let breathing_indicator = ProgressBar::new(4);
    // TODO proper duration per segment
    // TODO instead of a single same bar,
    //  maybe construct new progress bar per step to have well divided steps?
    get_ready();
    breathe_in(&breathing_indicator, 4);
    hold(&breathing_indicator, 7);
    breathe_out(&breathing_indicator, 8);
}

fn get_ready() {
    println!("Get comfortable and ready.");
    let stdout: Stdout = stdout();
    for count_down in (0u8..=TIME_TO_GET_READY).rev() {
        let countdown_text = format!("{}", count_down);
        print_in_place(&stdout, countdown_text);
        sleep_tick();
    }
    println!();
}

fn breathe_in(breathing_indicator: &ProgressBar, segment_duration: u64) {
    // TODO common function, operation as enum,
    //  based on enum print different stuff and either inc, dec or don't do anything at all.
    breathing_indicator.println("Breathe in.");
    for _ in 0..segment_duration {
        breathing_indicator.inc(PROGRESS_BAR_INTERVAL);
        sleep_tick();
    }
}

fn hold(breathing_indicator: &ProgressBar, segment_duration: u64) {
    // TODO this still has to show some kind of progress bar
    breathing_indicator.println("Hold...");
    for _ in 0..segment_duration {
        sleep_tick();
    }
}

fn breathe_out(breathing_indicator: &ProgressBar, segment_duration: u64) {
    breathing_indicator.set_length(segment_duration);
    breathing_indicator.set_position(segment_duration);
    breathing_indicator.println("Breathe out.");
    for _ in 0..segment_duration {
        breathing_indicator.dec(PROGRESS_BAR_INTERVAL);
        sleep_tick();
    }
}

fn sleep_tick() {
    thread::sleep(time::Duration::from_secs(SECONDS_OF_SLEEP_INTERVAL));
}
