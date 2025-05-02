mod data_structures;
mod analysis;
mod data_loader;
mod helpers;
mod visualization;

use data_loader::load_crash_data;
use analysis::{
    group_by_intersections,
    build_crash_graph,
    compute_degree_distribution,
};
use visualization::{plot_degree_histogram, draw_crash_heatmap};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data/boston_crashes.csv";
    let precision = 0.0005;
    let max_distance = 0.0010;

    // Load and process crash data
    let crash_data = load_crash_data(file_path)?;
    println!("Loaded {} crash records", crash_data.len());

    // Build intersection graph
    let intersections = group_by_intersections(&crash_data, precision);
    let graph = build_crash_graph(intersections, max_distance);

    println!(
        "Constructed graph with {} intersections and {} edges",
        graph.nodes.len(),
        graph.adjacency.values().map(|v| v.len()).sum::<usize>() / 2
    );

    // Degree distribution visualization
    let degrees = compute_degree_distribution(&graph);
    plot_degree_histogram(&degrees, "output/degree_histogram.png")?;

    // Heatmap visualization
    draw_crash_heatmap(&crash_data, "output/crash_heatmap.png", 0.001)?;

    Ok(())
}
