use chrono::{NaiveDate, NaiveTime};
use std::collections::HashMap;

/// A crash record with parsed date/time and validated coordinates
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

/// A graph node representing a unique intersection
#[derive(Debug, Clone)]
pub struct IntersectionNode {
    pub id: usize, // unique node ID
    pub x: f64,
    pub y: f64,
    pub crashes: Vec<ProcessedCrashRecord>, // all crashes at or near this intersection
}

/// A graph structure for intersection connectivity
#[derive(Debug)]
pub struct CrashGraph {
    pub nodes: Vec<IntersectionNode>,
    pub adjacency: HashMap<usize, Vec<usize>>, // node_id -> neighbor node_ids
}









