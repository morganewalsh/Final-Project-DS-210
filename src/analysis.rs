use crate::data_structures::{ProcessedCrashRecord, IntersectionNode, CrashGraph};
use std::collections::HashMap;

pub fn group_by_intersections(
    data: &[ProcessedCrashRecord],
    precision: f64,
) -> Vec<IntersectionNode> {
    let mut grouped: HashMap<(i32, i32), Vec<ProcessedCrashRecord>> = HashMap::new();

    for crash in data {
        let key = (
            (crash.x_coordinate / precision).round() as i32,
            (crash.y_coordinate / precision).round() as i32,
        );
        grouped.entry(key).or_default().push(crash.clone());
    }

    grouped
        .into_iter()
        .enumerate()
        .map(|(id, ((x, y), crashes))| IntersectionNode {
            id,
            x: x as f64,
            y: y as f64,
            crashes,
        })
        .collect()
}

pub fn euclidean_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

pub fn compute_degree_distribution(graph: &CrashGraph) -> HashMap<usize, usize> {
    graph.adjacency.iter().map(|(&id, neighbors)| (id, neighbors.len())).collect()
}

pub fn build_crash_graph(
    nodes: Vec<IntersectionNode>,
    max_distance: f64,
) -> CrashGraph {
    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();

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
