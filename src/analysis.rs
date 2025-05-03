use crate::data_structures::{ProcessedCrashRecord, IntersectionNode};
use crate::helpers::round_coord;
use std::collections::HashMap;


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