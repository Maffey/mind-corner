use crate::project_consts::{APPLICATION_OUTPUT_DIRECTORY, MEDITATION_TIMER_LOG_FILENAME};
use log::{error, info};
use polars::prelude::*;
use std::error::Error;
use std::path::Path;

pub(crate) fn run_data_analysis() {
    if let Err(e) = process_meditation_data() {
        error!("Error processing meditation data: {}", e);
    }
}

fn process_meditation_data() -> Result<(), Box<dyn Error>> {
    let meditation_file_path = format!(
        "{}{}",
        APPLICATION_OUTPUT_DIRECTORY, MEDITATION_TIMER_LOG_FILENAME
    );

    if !Path::new(&meditation_file_path).exists() {
        return Err(format!("File not found: {}", meditation_file_path).into());
    }

    let reader = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
        .try_into_reader_with_file_path(Some(meditation_file_path.into()))?;

    let meditation_timer_df = reader.finish()?;

    let average_duration = meditation_timer_df
        .clone()
        .lazy()
        .select([col("duration").mean().alias("average_duration")])
        .collect()?
        .column("average_duration")?
        .f64()?
        .get(0)
        .ok_or_else(|| PolarsError::ComputeError("Could not calculate average duration".into()))?;

    println!("{}", meditation_timer_df);
    info!("Average meditation duration: {:.2}", average_duration);

    let average_per_month = meditation_timer_df
        .lazy()
        .with_columns([
            col("timestamp").dt().year().alias("year"),
            col("timestamp").dt().month().alias("month"),
        ])
        .group_by([col("year"), col("month")])
        .agg([col("duration").mean().alias("average_duration")])
        .sort_by_exprs(vec![col("year"), col("month")], Default::default())
        .collect()?;

    println!("{}", average_per_month);
    // TODO Add a plot generation, someday. (#7)

    info!("Generated temporal mean DataFrame");

    Ok(())
}
