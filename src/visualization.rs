use plotters::prelude::*;

pub fn neighborsize(
    nmap: &HashMap<usize, usize>
    outpath:&str, 
) -> Result<(), Box<dyn std::error::Error>>{
    let mut number = HashMap::new();
    for &d in nmap.values() {
        *number.entry(d).or_insert(0)+=1;
    }

    let mut dcounts: Vec<(usize, usize)> = number.into_iter().collect();
    dcounts.sort_by_key(|&(deg, _)| deg);

    let plot = mapoutput::new(output_path, (800, 600)).into_drawing_area();
    plot.fill(&WHITE)?;

    let xmap = dcounts.iter().map(|&(deg, _)| deg).max().unwrap_or(1);
    let ymap = dcounts.iter().map(|&(_, count)| count).max().unwrap_or(1);

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
    plot.present()?;
    Ok(())
}