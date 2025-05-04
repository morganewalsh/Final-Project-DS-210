use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

///this module saves all the structs for this project as well as the crash record processing function


#[derive(Debug, Deserialize)]
pub struct CrashRecord {
    pub crash_number: String,
    pub crash_date: String,
    pub crash_time: String,
    pub total_nonfatal_injuries: Option<f64>,
    pub total_fatal_injuries: Option<f64>,
    pub at_roadway_intersection: String,
    pub x_coordinate: Option<f64>,
    pub y_coordinate: Option<f64>,
}


#[derive(Debug, Clone)]
pub struct ProcessedCrashRecord {
    pub crash_number: String,
    pub crash_date: NaiveDate,
    pub crash_time: NaiveTime,
    pub total_nonfatal_injuries: Option<f64>,
    pub total_fatal_injuries: Option<f64>,
    pub at_roadway_intersection: String,
    pub x_coordinate: f64,
    pub y_coordinate: f64,
}

impl ProcessedCrashRecord {
    pub fn from_raw(raw: CrashRecord) -> Option<Self> {
        let (x, y) = match (raw.x_coordinate, raw.y_coordinate) {
            (Some(x), Some(y)) => (x, y),
            _ => {
                //eprintln!("Skipping: missing coordinates");
                return None;
            }
        };

        // Clean up and parse the date
        let date_str = raw.crash_date.trim();
        let date = NaiveDate::parse_from_str(date_str, "%d-%b-%Y")
            .or_else(|_| NaiveDate::parse_from_str(date_str, "%d-%B-%Y"))
            .ok()
            .or_else(|| {
                //eprintln!("Skipping: invalid date '{}'", date_str);
                None
            })?;

    
        let time = NaiveTime::parse_from_str(&raw.crash_time.trim(), "%I:%M %p").ok()
            .or_else(|| {
                //eprintln!("Skipping: invalid time '{}'", raw.crash_time);
                None
            })?;

        Some(Self {
            crash_number: raw.crash_number,
            crash_date: date,
            crash_time: time,
            total_nonfatal_injuries: raw.total_nonfatal_injuries,
            total_fatal_injuries: raw.total_fatal_injuries,
            at_roadway_intersection: raw.at_roadway_intersection.to_lowercase(),
            x_coordinate: x,
            y_coordinate: y,
        })
    }
}


#[derive(Debug, Clone)]
pub struct IntersectionNode {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub crashes: Vec<ProcessedCrashRecord>,
}

#[derive(Debug)]
pub struct CrashGraph {
    pub nodes: Vec<IntersectionNode>,
    pub adjacency: std::collections::HashMap<usize, Vec<usize>>,
}









