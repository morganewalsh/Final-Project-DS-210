use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CrashRecord {
    pub crash_number: String,
    pub city_town_name: String,
    pub crash_date: String,
    pub crash_time: String,
    pub number_of_vehicles: Option<f64>,
    pub total_nonfatal_injuries: Option<f64>,
    pub total_fatal_injuries: Option<f64>,
    pub ambient_light: String,
    pub road_surface_condition: String,
    pub weather_condition: String,
    pub at_roadway_intersection: String,
    pub x_coordinate: Option<f64>,
    pub y_coordinate: Option<f64>,
}


#[derive(Debug, Clone)]
pub struct ProcessedCrashRecord {
    pub crash_number: String,
    pub city_town_name: String,
    pub crash_date: NaiveDate,
    pub crash_time: NaiveTime,
    pub number_of_vehicles: Option<f64>,
    pub total_nonfatal_injuries: Option<f64>,
    pub total_fatal_injuries: Option<f64>,
    pub ambient_light: String,
    pub road_surface_condition: String,
    pub weather_condition: String,
    pub at_roadway_intersection: String,
    pub x_coordinate: f64,
    pub y_coordinate: f64,
}

impl ProcessedCrashRecord {
    pub fn from_raw(raw: CrashRecord) -> Option<Self> {
        let (x, y) = (raw.x_coordinate?, raw.y_coordinate?);
        let date = NaiveDate::parse_from_str(&raw.crash_date, "%Y-%m-%d").ok()?;
        let time = NaiveTime::parse_from_str(&raw.crash_time, "%H:%M").ok()?;

        Some(Self {
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









