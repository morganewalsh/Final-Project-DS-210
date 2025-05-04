use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::Reader;
use crate::data_structures::{CrashRecord, ProcessedCrashRecord};

pub fn load_crash_data<P: AsRef<Path>>(file_path: P) -> Result<Vec<ProcessedCrashRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    let processed_records = rdr
        .deserialize()
        .filter_map(|result| {
            result.ok().and_then(|raw: CrashRecord| ProcessedCrashRecord::from_raw(raw))
        })
        .collect();

    Ok(processed_records)
}