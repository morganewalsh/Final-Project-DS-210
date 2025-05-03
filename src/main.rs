mod data_structures;
mod analysis;
mod data_loader;
mod helpers;
mod visualization;
use std::time::Instant;
use data_loader::load_crash_data;
use analysis::{
    group_by_intersections,
    build_crash_graph,
    compute_degree_distribution,
};
use visualization::{plot_degree_histogram, draw_crash_heatmap};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
}
