mod data_structures;
mod analysis;
mod data_loader;
mod visualization;
use std::time::Instant;
use data_loader::load_crash_data;
use analysis::{group_by_intersections, build_crash_graph, compute_degree_distribution};
use visualization::plot_degree_histogram;
use std::error::Error;

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

    println!("Duration {:.2?}", start.elapsed());

    Ok(())
}
