use crate::utilities::print_in_place;
use indicatif::ProgressBar;
use inquire::Select;
use std::io::{stdout, Stdout};
use std::{fmt, thread, time};

const SECONDS_OF_SLEEP_INTERVAL: u64 = 1;
const PROGRESS_BAR_INTERVAL: u64 = 1;
const TIME_TO_GET_READY: u8 = 3;

#[derive(Clone)]
struct BreathingPattern {
    name: &'static str,
    breathe_in_duration: u64,
    hold_duration: u64,
    breathe_out_duration: u64,
}

impl BreathingPattern {
    fn run_breathing_guide(&self, breathing_indicator: &ProgressBar) {
        breathe_in(breathing_indicator, self.breathe_in_duration);
        hold(breathing_indicator, self.hold_duration);
        breathe_out(breathing_indicator, self.breathe_out_duration);
    }
}

impl fmt::Display for BreathingPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

const BREATHING_PATTERNS: &[BreathingPattern] = &[
    BreathingPattern {
        name: "4-7-8",
        breathe_in_duration: 4,
        hold_duration: 7,
        breathe_out_duration: 8,
    },
    BreathingPattern {
        name: "Box",
        breathe_in_duration: 4,
        hold_duration: 4,
        breathe_out_duration: 4,
    },
];

pub(crate) fn run_breathing_guide() {
    // TODO store data of type and duration, to csv, after refactor into common csv writing (#14)

    let breathing_pattern_options: Vec<BreathingPattern> = BREATHING_PATTERNS.to_vec();

    let breathing_pattern: BreathingPattern =
        Select::new("Select breathing pattern.", breathing_pattern_options)
            .prompt()
            .expect("Failed to get breathing pattern");

    let breathing_indicator = ProgressBar::new(4);
    get_ready();
    loop {
        breathing_pattern.run_breathing_guide(&breathing_indicator);
    }
}

fn get_ready() {
    println!("Get comfortable.");
    let stdout: Stdout = stdout();
    for count_down in (0u8..=TIME_TO_GET_READY).rev() {
        let countdown_text = format!("{}", count_down);
        print_in_place(&stdout, countdown_text);
        sleep_tick();
    }
    println!();
}

fn breathe_in(breathing_indicator: &ProgressBar, segment_duration: u64) {
    breathing_indicator.println("Breathe in.");
    for _ in 0..segment_duration {
        breathing_indicator.inc(PROGRESS_BAR_INTERVAL);
        sleep_tick();
    }
}

fn hold(breathing_indicator: &ProgressBar, segment_duration: u64) {
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
