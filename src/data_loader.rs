use crate::models::{CrashRecord, ProcessedCrashRecord};
use chrono::{NaiveDate, NaiveTime};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::Reader;

pub fn load_crash_data<P: AsRef<Path>>(file_path: P) -> Result<Vec<ProcessedCrashRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);
    let mut processed_records = Vec::new();

    for result in rdr.deserialize() {
        let raw: CrashRecord = result?;

        // Skip if coordinates are missing
        let (x, y) = match (raw.x_coordinate, raw.y_coordinate) {
            (Some(x), Some(y)) => (x, y),
            _ => continue,
        };

        let date = match NaiveDate::parse_from_str(&raw.crash_date, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => continue,
        };

        let time = match NaiveTime::parse_from_str(&raw.crash_time, "%H:%M") {
            Ok(t) => t,
            Err(_) => continue,
        };

        processed_records.push(ProcessedCrashRecord {
            crash_number: raw.crash_number,
            city_town_name: raw.city_town_name,
            crash_date: date,
            crash_time: time,
            number_of_vehicles: raw.number_of_vehicles,
            total_nonfatal_injuries: raw.total_nonfatal_injuries,
            total_fatal_injuries: raw.total_fatal_injuries,
            ambient_light: raw.ambient_light.to_lowercase(),
            road_surface_condition: raw.road_surface_condition.to_lowercase(),
            weather_condition: raw.weather_condition.to_lowercase(),
            at_roadway_intersection: raw.at_roadway_intersection.to_lowercase(),
            x_coordinate: x,
            y_coordinate: y,
        });
    }

    Ok(processed_records)
}

