mod data_structures;
mod analysis;
mod data_loader;
mod visualization;
use std::time::Instant;
use data_loader::load_crash_data;
use analysis::{group_by_intersections, build_crash_graph, compute_degree_distribution};
use visualization::plot_degree_histogram;
use std::error::Error;
use crate::data_structures::{CrashGraph, IntersectionNode, ProcessedCrashRecord};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now(); //tracking run time 

    let file_path = "data/crash_data.csv";
    let bin_precision = 25.0; 
    let max_connection_dist = 10.0; //parameteres for graphing 

    let crash_data = load_crash_data(file_path)?;
    println!("Loaded {} crash records.", crash_data.len());

    let intersections = group_by_intersections(&crash_data, bin_precision);
    let graph = build_crash_graph(intersections, max_connection_dist);

    println!(
        "Built graph with {} intersections and {} edges.",
        graph.nodes.len(),
        graph.adjacency.values().map(|v| v.len()).sum::<usize>() / 2
    );

    let degrees = compute_degree_distribution(&graph);
    println!("Computed degrees for {} nodes", degrees.len());
    plot_degree_histogram(&degrees, "histogram_output/degree_histogram.png")?;


    pub fn top_n_high_degree_nodes(graph: &CrashGraph, n: usize) -> Vec<(usize, String, f64, f64)> {
        let mut node_degrees: Vec<(&IntersectionNode, usize)> = graph.nodes.iter()
            .map(|node| {
                let degree = graph.adjacency.get(&node.id).map_or(0, |neighbors| neighbors.len());
                (node, degree)
            })
            .collect();
    
        node_degrees.sort_by(|a, b| b.1.cmp(&a.1));
    
        node_degrees
            .into_iter()
            .take(n)
            .map(|(node, degree)| {
                let name = most_common_intersection_name(&node.crashes);
                (degree, name, node.x, node.y)
            })
            .collect()
    }

    for (degree, name, x, y) in top_nodes {
        println!(
            "{} (Degree: {}) at approx. coords ({:.2}, {:.2})",
            name,
            degree,
            x,
            y
        );
    }
    


    println!("Duration {:.2?}", start.elapsed());

    Ok(())
}
