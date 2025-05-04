use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::Reader;
use crate::data_structures::{CrashRecord, ProcessedCrashRecord};


///loads and parses data from the crash_data.csv & populates the processed crash record struct

pub fn load_crash_data<P: AsRef<Path>>(file_path: P) -> Result<Vec<ProcessedCrashRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    let records = rdr
        .deserialize()
        .filter_map (|result| match result{
            Ok(raw) => ProcessedCrashRecord::from_raw(raw),
            Err(err) => {
                //eprintln!("skipping record: {}", err);
                None
            }
        })
        .collect();

    Ok(records)
}