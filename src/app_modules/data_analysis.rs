use std::fs::File;
use chrono::prelude::*;
use polars::prelude::*;

pub(crate) fn run_data_analysis() {
    let dt1 = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(15, 4, 51).unwrap();
    let dt2 = NaiveDate::from_ymd_opt(2025, 1, 21).unwrap().and_hms_opt(12, 37, 44).unwrap();
    let mut df: DataFrame = df!(
        "timestamp" => [dt1, dt2],
        "duration" => [4, 5]
    ).unwrap();
    
    println!("{}", df);
    
    let mut file = File::create("/home/maffey/RustroverProjects/mind-corner/target/app_output/polars_meditation_timer_log.csv").expect("Unable to create file");
    
    CsvWriter::new(&mut file).include_header(true).with_separator(b',').finish(&mut df).unwrap();
    
    let df_csv = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .with_parse_options(CsvParseOptions::default()
            .with_try_parse_dates(true))
        .try_into_reader_with_file_path(Some("/home/maffey/RustroverProjects/mind-corner/target/app_output/polars_meditation_timer_log.csv".into())).unwrap()
        .finish().unwrap();
    
    println!("{}", df_csv);
}