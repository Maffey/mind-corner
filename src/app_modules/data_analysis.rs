use log::info;
use crate::project_consts::{APPLICATION_OUTPUT_DIRECTORY, MEDITATION_TIMER_LOG_FILENAME};
use polars::prelude::*;

pub(crate) fn run_data_analysis() {
    let meditation_data_result = process_meditation_data();
}

fn process_meditation_data() {
    // TODO get rid of all the unwraps

    let meditation_filepath = format!(
        "{APPLICATION_OUTPUT_DIRECTORY}{MEDITATION_TIMER_LOG_FILENAME}"
    );

    let meditation_timer_df = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .with_parse_options(CsvParseOptions::default()
            .with_try_parse_dates(true))
        .try_into_reader_with_file_path(Some(meditation_filepath.into())).unwrap()
        .finish().unwrap();

    // TODO average duration, catch errors if csv doesnt exist or something
    let average_duration = meditation_timer_df
        .clone()
        .lazy()
        .select([col("duration").mean().alias("average_duration")])
        .collect()
        .unwrap()
        .column("average_duration")
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap();


    // TODO step average over time

    println!("{}", meditation_timer_df);
    info!("Average meditation duration: {:.2}", average_duration);
}