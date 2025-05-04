use plotters::prelude::*; //uses plotters crate 
use std::collections::HashMap;

///this module holds the functions that populate the histograms from degree map and crash severity

pub fn plot_degree_histogram( //takes in hashmap of node ids and number of neighbors each node has 
    //output_path returns a file path to saved the plot png  
    degree_map: &HashMap<usize, usize>,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if degree_map.is_empty() {
        //eprintln!("No degree data to plot.");
        return Ok(());
    }

    // Convert to sorted vector of degree and count pairs
    let mut degree_counts: HashMap<usize, usize> = HashMap::new();
    for &deg in degree_map.values() {
        *degree_counts.entry(deg).or_insert(0) += 1;
    }

    //sorts degrees in ascending order 
    let mut sorted: Vec<_> = degree_counts.into_iter().collect();
    sorted.sort_by_key(|&(deg, _)| deg);

    if sorted.is_empty() {
        //eprintln!("Degree frequency list is empty.");
        return Ok(());
    }
    
    // bounds for axes
    let x_max = sorted.iter().map(|&(deg, _)| deg).max().unwrap_or(1);
    let y_max = sorted.iter().map(|&(_, count)| count).max().unwrap_or(1);

    // Set up background for graph 
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    //println!("Degree Frequency Distribution:");
    // for (deg, count) in &sorted {
    //     println!("Degree {}: {} nodes", deg, count);
    //}

    //chart outline 
    let mut chart = ChartBuilder::on(&root)
        .caption("Node Degree Distribution", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0usize..(x_max + 1), 0usize..(y_max + 5))?;

    chart
        .configure_mesh()
        .x_desc("Node Degree")
        .y_desc("Count")
        .disable_mesh()
        .draw()?;

    chart.draw_series(  // Draws the bars
        sorted.iter().map(|&(deg, count)| {
            Rectangle::new([(deg.saturating_sub(1), 0), (deg + 1, count)], BLUE.filled())
        }),
    )?;

    root.present()?; 
    println!("Histogram saved to {}", output_path);
    Ok(())
}
