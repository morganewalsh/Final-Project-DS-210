use plotters::prelude::*;
use std::collections::HashMap;

pub fn plot_degree_histogram(
    degree_map: &HashMap<usize, usize>,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut freq: HashMap<usize, usize> = HashMap::new();

    // Count how many nodes have each degree
    for &deg in degree_map.values() {
        *freq.entry(deg).or_insert(0) += 1;
    }

    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_degree = *freq.keys().max().unwrap_or(&1);
    let max_count = *freq.values().max().unwrap_or(&1);

    let mut chart = ChartBuilder::on(&root)
        .caption("Intersection Degree Distribution", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0usize..(max_degree + 1), 0usize..(max_count + 5))?;

    chart.configure_mesh().x_desc("Degree").y_desc("Frequency").draw()?;

    chart.draw_series(freq.iter().map(|(&deg, &count)| {
        Rectangle::new(
            [(deg, 0), (deg, count)],
            RED.filled(),
        )
    }))?;

    root.present()?;
    Ok(())
}


use plotters::prelude::*;
use crate::data_structures::ProcessedCrashRecord;

/// Draws a heatmap of crash density across a 2D grid (based on X/Y coordinates)
pub fn draw_crash_heatmap(
    data: &[ProcessedCrashRecord],
    output_path: &str,
    bin_size: f64, // e.g., 0.001
) -> Result<(), Box<dyn std::error::Error>> {
    // Group crashes into grid bins
    use std::collections::HashMap;

    let mut heatmap: HashMap<(i32, i32), usize> = HashMap::new();

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for crash in data {
        let bx = (crash.x_coordinate / bin_size).floor() as i32;
        let by = (crash.y_coordinate / bin_size).floor() as i32;

        *heatmap.entry((bx, by)).or_insert(0) += 1;

        min_x = min_x.min(crash.x_coordinate);
        max_x = max_x.max(crash.x_coordinate);
        min_y = min_y.min(crash.y_coordinate);
        max_y = max_y.max(crash.y_coordinate);
    }

    // Configure drawing
    let image_width = 800;
    let image_height = 800;

    let root = BitMapBackend::new(output_path, (image_width, image_height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Crash Density Heatmap", ("sans-serif", 30))
        .margin(10)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().disable_mesh().draw()?;

    // Normalize color intensity
    let max_count = heatmap.values().copied().max().unwrap_or(1) as f64;

    for ((bx, by), count) in heatmap.iter() {
        let x = *bx as f64 * bin_size;
        let y = *by as f64 * bin_size;

        let rect = Rectangle::new(
            [(x, y), (x + bin_size, y + bin_size)],
            HSLColor(0.0, 1.0, (1.0 - (*count as f64 / max_count)) * 0.8).filled(),
        );

        chart.draw_series(std::iter::once(rect))?;
    }

    root.present()?;
    Ok(())
}