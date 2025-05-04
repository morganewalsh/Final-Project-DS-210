mod data_structures;
mod analysis;
mod data_loader;
mod visualization;

use std::time::Instant;
use std::error::Error;
use std::collections::HashMap;

use data_loader::load_crash_data;
use analysis::{group_by_intersections, build_crashgraph, compute_degree_distribution, print_top_severe_intersections, top_n_high_degree_nodes, is_severe};
use visualization::plot_degree_histogram;
use crate::data_structures::{CrashGraph, IntersectionNode, ProcessedCrashRecord};


/// Main function in crash data analysis
// Constructs graph of intersection nodes and highlights top connected and most severe intersections

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now(); //tracking run time 

    let file_path = "data/crash_data.csv"; /// Loads and processes crash records
    let bin_precision = 25.0;  //rounding precision
    let max_connection_dist = 10.0; //parameteres for max distance to connect nodes 

    let crash_data = load_crash_data(file_path)?;
    println!("Loaded {} crash records.", crash_data.len()); //loading data 

    //building graph 
    let intersections = group_by_intersections(&crash_data, bin_precision);
    let graph = build_crashgraph(intersections.clone(), max_connection_dist); /// Constructs a graph of intersection nodes

    println!(
        "Built graph with {} intersections & {} edges.",
        graph.nodes.len(),
        graph.adjacency.values().map(|v| v.len()).sum::<usize>() / 2 //deals with undirected edges 
    );

    //degree counts 
    let degrees = compute_degree_distribution(&graph);
    println!("Computed degrees for {} nodes", degrees.len());

    plot_degree_histogram(&degrees, "histogram_output/degree_histogram.png")?;

    //print top 5 intersections with most crashes 
    let top_nodes = top_n_high_degree_nodes(&graph, 5);
    println!("Top 5 highest-degree intersections:");
    for (degree, name, x, y) in top_nodes {
        let label = if name == "Unnamed intersection" || name.trim().is_empty() {
            format!("Intersection at ({:.2}, {:.2})", x, y)
        } else {
            name
        };
        println!("{} (Degree: {}) at approx. coords ({:.2}, {:.2})", label, degree, x, y);

    }
    
    //prints top 5 intersections with the most severe crashes 
    let severe_crashes: Vec<_> = crash_data.iter().filter(|crash| is_severe(crash)).cloned().collect();
    println!("Filtered to {} severe crashes", severe_crashes.len());
    let severe_intersections = group_by_intersections(&severe_crashes, bin_precision);

    let severe_graph = build_crashgraph(severe_intersections.clone(), max_connection_dist);
    print_top_severe_intersections(&severe_intersections, 5); //generating severe graph 

    let severe_degrees = compute_degree_distribution(&severe_graph);
    plot_degree_histogram(&severe_degrees, "histogram_output/severe_crash_degree_histogram.png")?;

    println!("Duration {:.2?}", start.elapsed()); //final time output 

    Ok(())
}



#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};
    use crate::data_structures::{CrashRecord, ProcessedCrashRecord};
    use crate::analysis::{group_by_intersections, build_crashgraph};

    #[test]
    fn test_struct_population() {
        let raw = CrashRecord { //populated with first row of crash data 
            crash_number: "4923964".to_string(),
            crash_date: "01-Jan-2021".to_string(),
            crash_time: "2:13 AM".to_string(),
            total_nonfatal_injuries: Some(0.0),
            total_fatal_injuries: Some(0.0),
            at_roadway_intersection: "HUNTINGTON AVENUE / WAIT STREET".to_string(),
            x_coordinate: Some(232459.5312),
            y_coordinate: Some(898185.625),
        };

        let processed = ProcessedCrashRecord::from_raw(raw);
        assert!(processed.is_some());
        let crash = processed.unwrap();
        assert_eq!(crash.x_coordinate, 232459.5312);
        assert_eq!(crash.crash_date, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());
    }


    #[test]
    fn test_grouping() {
        use crate::data_structures::ProcessedCrashRecord;

        let fakecrashes = vec![
            ProcessedCrashRecord {
                crash_number: "1".to_string(),
                crash_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                crash_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                total_nonfatal_injuries: None,
                total_fatal_injuries: None,
                at_roadway_intersection: "HUNTINGTON AVENUE / WAIT STREET".to_string(),
                x_coordinate: 2000.0,
                y_coordinate: 8000.0,
            },
            ProcessedCrashRecord { x_coordinate: 2004.0, y_coordinate: 8004.0, ..crate::data_structures::ProcessedCrashRecord {
                crash_number: "2".to_string(),
                crash_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                crash_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                total_nonfatal_injuries: None,
                total_fatal_injuries: None,
                at_roadway_intersection: "WAIT STREET / HUNTINGTON AVENUE".to_string(),
                x_coordinate: 2004.0, 
                y_coordinate: 8004.0, 
            }},
        ];

        let intersections = group_by_intersections(&fakecrashes, 10.0);
        assert_eq!(intersections.len(), 1); 

        let graph = build_crashgraph(intersections, 10.0);
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.adjacency.len(), 0);
    }
}



