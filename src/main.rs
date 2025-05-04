mod data_structures;
mod analysis;
mod data_loader;
mod visualization;
use std::time::Instant;
use data_loader::load_crash_data;
use analysis::{group_by_intersections, build_crash_graph, compute_degree_distribution, top_n_high_degree_nodes,};
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

    println!("Duration {:.2?}", start.elapsed());

    Ok(())
}



#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};
    use crate::data_structures::{CrashRecord, ProcessedCrashRecord};
    use crate::analysis::{group_by_intersections, build_crash_graph};

    #[test]
    fn test_struct_population() {
        let raw = CrashRecord { //populated with first row of crash data 
            crash_number: "4923964".to_string(),
            city_town_name: "BOSTON".to_string(),
            crash_date: "01-Jan-2021".to_string(),
            crash_time: "2:13 AM".to_string(),
            number_of_vehicles: Some(2.0),
            total_nonfatal_injuries: Some(0.0),
            total_fatal_injuries: Some(0.0),
            ambient_light: "Dark - lighted roadway".to_string(),
            road_surface_condition: "Dry".to_string(),
            weather_condition: "Clear".to_string(),
            at_roadway_intersection: "HUNTINGTON AVENUE / WAIT STREET".to_string(),
            x_coordinate: Some(232459.5312),
            y_coordinate: Some(898185.625),
        };

        let processed = ProcessedCrashRecord::from_raw(raw);
        assert!(processed.is_some());
        let crash = processed.unwrap();
        assert_eq!(crash.city_town_name, "BOSTON");
        assert_eq!(crash.x_coordinate, 232459.5312);
        assert_eq!(crash.crash_date, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());
    }


    #[test]
    fn test_grouping_and_graph_edges() {
        use crate::data_structures::ProcessedCrashRecord;

        let fakecrashes = vec![
            ProcessedCrashRecord {
                crash_number: "1".to_string(),
                city_town_name: "BOSTON".to_string(),
                crash_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                crash_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                number_of_vehicles: Some(1.0),
                total_nonfatal_injuries: None,
                total_fatal_injuries: None,
                ambient_light: "daylight".to_string(),
                road_surface_condition: "dry".to_string(),
                weather_condition: "clear".to_string(),
                at_roadway_intersection: "HUNTINGTON AVENUE / WAIT STREET".to_string(),
                x_coordinate: 2000.0,
                y_coordinate: 8000.0,
            },
            ProcessedCrashRecord { x_coordinate: 2004.0, y_coordinate: 8004.0, ..crate::data_structures::ProcessedCrashRecord {
                crash_number: "2".to_string(),
                city_town_name: "BOSTON".to_string(),
                crash_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                crash_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                number_of_vehicles: Some(1.0),
                total_nonfatal_injuries: None,
                total_fatal_injuries: None,
                ambient_light: "daylight".to_string(),
                road_surface_condition: "dry".to_string(),
                weather_condition: "clear".to_string(),
                at_roadway_intersection: "WAIT STREET / HUNTINGTON AVENUE".to_string(),
                x_coordinate: 0.0, // overwritten
                y_coordinate: 0.0, // overwritten
            }},
        ];

        let intersections = group_by_intersections(&fakecrashes, 10.0);
        assert_eq!(intersections.len(), 1); // Should group due to bin size

        let graph = build_crash_graph(intersections, 10.0);
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.adjacency.len(), 0); // Only one node, no edges
    }
}



