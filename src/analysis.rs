use crate::data_structures::{ProcessedCrashRecord, IntersectionNode};
use crate::helpers::round_coord;
use std::collections::HashMap,
use crate::data_structures::{CrashGraph, ProcessedCrashRecord, IntersectionNode};



///intersections that have the most crashes 
pub fn group_by_intersections(
    data: &[ProcessedCrashRecord],
    precision: f64,
) -> Vec<IntersectionNode> {
    let mut grouped: HashMap<(f64, f64), Vec<ProcessedCrashRecord>> = HashMap::new();

    for crash in data {
        let key = (
            round_coord(crash.x_coordinate, precision),
            round_coord(crash.y_coordinate, precision),
        );
        grouped.entry(key).or_default().push(crash.clone());
    }

    grouped
        .into_iter()
        .enumerate()
        .map(|(id, ((x, y), crashes))| IntersectionNode { id, x, y, crashes })
        .collect()
}

pub fn crash_merge(value: f64, precision: f64) -> f64 {
    (value / precision).round() * precision
}

pub fn node_connection(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

pub fn build_crash_graph(
    nodes: Vec<IntersectionNode>,
    max_distance: f64,
) -> CrashGraph {
    let mut adjacency = HashMap::new();

    for (i, a) in nodes.iter().enumerate() {
        for (j, b) in nodes.iter().enumerate().skip(i + 1) {
            if euclidean_distance(a.x, a.y, b.x, b.y) <= max_distance {
                adjacency.entry(a.id).or_default().push(b.id);
                adjacency.entry(b.id).or_default().push(a.id);
            }
        }
    }

    CrashGraph { nodes, adjacency }
}
