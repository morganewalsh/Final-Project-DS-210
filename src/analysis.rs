use crate::data_structures::{ProcessedCrashRecord, IntersectionNode, CrashGraph};
use crate::helper_functions::round_coord;
use std::collections::HashMap;

/// Group crash records into approximate intersections based on rounded coordinates
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

/// Build a graph of intersections, connecting those within a threshold distance
pub fn build_crash_graph(
    nodes: Vec<IntersectionNode>,
    max_distance: f64,
) -> CrashGraph {
    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, node_a) in nodes.iter().enumerate() {
        for (j, node_b) in nodes.iter().enumerate().skip(i + 1) {
            let dx = node_a.x - node_b.x;
            let dy = node_a.y - node_b.y;
            let distance = (dx * dx + dy * dy).sqrt(); // Euclidean distance

            if distance <= max_distance {
                adjacency.entry(node_a.id).or_default().push(node_b.id);
                adjacency.entry(node_b.id).or_default().push(node_a.id);
            }
        }
    }

    CrashGraph { nodes, adjacency }
}


pub fn compute_degree_distribution(graph: &CrashGraph) -> HashMap<usize, usize> {
    graph
        .adjacency
        .iter()
        .map(|(&node_id, neighbors)| (node_id, neighbors.len()))
        .collect()
}


pub fn count_crashes_by_intersection(data: &[ProcessedCrashRecord]) -> Vec<(String, u32)> {
    let mut counts = HashMap::new();
    for crash in data {
        let key = crash.at_roadway_intersection.clone();
        *counts.entry(key).or_insert(0) += 1;
    }

    let mut results: Vec<_> = counts.into_iter().collect();
    results.sort_by(|a, b| b.1.cmp(&a.1));
    results
}