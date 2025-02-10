use indicatif::ProgressBar;
use std::{thread, time};
const SECONDS_OF_SLEEP_INTERVAL: u64 = 1;
const PROGRESS_BAR_INTERVAL: u64 = 1;

pub(crate) fn run_breathing_guide() {
    // TODO store data of type and duration, to csv

    // Add option to interrupt it.

    let breathing_indicator = ProgressBar::no_length();
    // TODO proper duration per segment
    // TODO instead of a single same bar,
    //  maybe construct new progress bar per step to have well divided steps?
    breathe_in(&breathing_indicator, 4);
    hold(&breathing_indicator, 7);
    breathe_out(&breathing_indicator, 8);
}

fn breathe_in(breathing_indicator: &ProgressBar, segment_duration: u64) {
    // TODO common function, operation as enum,
    //  based on enum print different stuff and either inc, dec or don't do anything at all.

    // TODO print with flush so the text is overriden instead of appended.
    // TODO some setup time for person to get ready.
    breathing_indicator.set_length(segment_duration);
    breathing_indicator.println("Breathe in.");
    for _ in 0..segment_duration {
        breathing_indicator.inc(PROGRESS_BAR_INTERVAL);
        sleep_tick();
    }
}

fn hold(breathing_indicator: &ProgressBar, segment_duration: u64) {
    // TODO this still has to show some kind of progress
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
