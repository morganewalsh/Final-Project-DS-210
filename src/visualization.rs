use plotters::prelude::*;
use std::collections::HashMap;

pub fn plot_degree_histogram(
    degree_map: &HashMap<usize, usize>,
    output_path:&str, 
) -> Result<(), Box<dyn std::error::Error>>{
    let mut frequency = HashMap::new();
    for &d in degree_map.values() {
        *frequency.entry(d).or_insert(0)+=1;
    }

    let mut degree_counts: Vec<(usize, usize)> = frequency.into_iter().collect();
    degree_counts.sort_by_key(|&(deg, _)| deg);

    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_max = degree_counts.iter().map(|&(deg, _)| deg).max().unwrap_or(1);
    let y_max = degree_counts.iter().map(|&(_, count)| count).max().unwrap_or(1);

    let mut chart = ChartBuilder::on(&root)
        .caption("Intersection Degree", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0usize..(x_max + 1), 0usize..(y_max + 5))?;
    
    chart
        .configure_mesh()
        .x_desc("Degree")
        .y_desc("Frequency")
        .disable_mesh()
        .draw()?;
    chart.draw_series(
        degree_counts.iter().map(|&(deg, count)| {
             Rectangle::new(
                [(deg, 0), (deg, count)],
                BLUE.mix(0.7).filled(),
            )
        }),
    )?;
    root.present()?;
    Ok(())
}