use crate::data_structures::{ProcessedCrashRecord, IntersectionNode, CrashGraph};
use std::collections::HashMap;


//this module is the backbone of grouping data, building graphs, and ranking the intersection clusters 

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


pub fn build_crashgraph(
    nodes: Vec<IntersectionNode>,
    max_distance: f64,
) -> CrashGraph {
    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, a) in nodes.iter().enumerate() {
        for b in nodes.iter().skip(i + 1) {
            if edistance(a.x, a.y, b.x, b.y) <= max_distance {
                adjacency.entry(a.id).or_default().push(b.id);
                adjacency.entry(b.id).or_default().push(a.id);
            }
        }
    }

    CrashGraph { nodes, adjacency }
}


pub fn most_common_name(crashes: &[ProcessedCrashRecord]) -> String {
    let mut count_map = HashMap::new();

    for crash in crashes {
        let name = crash.at_roadway_intersection.trim().to_lowercase();
        if !name.is_empty() && name != "unknown" {
            *count_map.entry(name).or_insert(0) += 1;
        }
    }

    count_map
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(name, _)| name)
        .unwrap_or_else(|| "Unnamed intersection".to_string())
}


pub fn top_n_high_degree_nodes(graph: &CrashGraph, n: usize) -> Vec<(usize, String, f64, f64)> {
    let mut node_degrees: Vec<(&IntersectionNode, usize)> = graph
        .nodes
        .iter()
        .map(|node| {
            let degree = graph
                .adjacency
                .get(&node.id)
                .map_or(0, |neighbors| neighbors.len());
            (node, degree)
        })
        .collect();

    node_degrees.sort_by(|a, b| b.1.cmp(&a.1));

    node_degrees
        .into_iter()
        .take(n)
        .map(|(node, degree)| {
            let name = most_common_name(&node.crashes);
            (degree, name, node.x, node.y)
        })
        .collect()
}


pub fn is_severe(crash: &ProcessedCrashRecord) -> bool {
    crash.total_fatal_injuries.unwrap_or(0.0) > 0.0
        || crash.total_nonfatal_injuries.unwrap_or(0.0) > 0.0
}

pub fn edistance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

pub fn compute_degree_distribution(graph: &CrashGraph) -> HashMap<usize, usize> {
    graph.adjacency.iter().map(|(&id, neighbors)| (id, neighbors.len())).collect()
} 


pub fn print_top_severe_intersections(nodes: &[IntersectionNode], n: usize) {
    let mut nodes_by_severity: Vec<_> = nodes
        .iter()
        .map(|node| {
            let severe_count = node
                .crashes
                .iter()
                .filter(|crash| {
                    crash.total_fatal_injuries.unwrap_or(0.0) > 0.0
                        || crash.total_nonfatal_injuries.unwrap_or(0.0) > 0.0
                })
                .count();
            (node, severe_count)
        })
        .filter(|(_, count)| *count > 0)
        .collect();

    nodes_by_severity.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

    println!("Top {} intersections with most severe crashes:", n);
    for (i, (node, count)) in nodes_by_severity.iter().take(n).enumerate() {
        let name = node
            .crashes
            .first()
            .map(|crash| crash.at_roadway_intersection.clone())
            .unwrap_or("unknown".into());

        println!(
            "{}. {} (Severe crashes: {}) at approx. coords ({:.2}, {:.2})",
            i + 1,
            name,
            count,
            node.x,
            node.y
        );
    }
}
