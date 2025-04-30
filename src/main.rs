use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct CrashData {
    #[serde(rename = "Distance_From_Nearest_Roadway_Intersection")]
    distance_from_nearest_roadway_intersection: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load CSV file
    let file_path = "/opt/app-root/src/Final_Project_210/boston_crashes/crash_data.csv";
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;

    // Initialize a map to store crash counts by intersection
    let mut intersection_counts: HashMap<String, u32> = HashMap::new();

    // Read the CSV data
    for result in rdr.deserialize() {
        let record: CrashData = result?;

        if record.distance_from_nearest_roadway_intersection.trim().is_empty() {
            continue;
        }

        let count = intersection_counts.entry(record.distance_from_nearest_roadway_intersection.clone()).or_insert(0);
        *count += 1;
    }

    // Sort intersections by the number of crashes in descending order
    let mut sorted_intersections: Vec<_> = intersection_counts.into_iter().collect();
    sorted_intersections.sort_by(|a, b| b.1.cmp(&a.1));

    // Output the top 10 intersections
    println!("Top 10 intersections with the most crashes:");
    for (i, (intersection, count)) in sorted_intersections.iter().take(10).enumerate() {
        println!("{}. {}: {} crashes", i + 1, intersection, count);
    }

    Ok(())
}
