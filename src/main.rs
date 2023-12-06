use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn map_range_boundaries(range: (i64, i64), mapping: &[(i64, i64, i64)]) -> Vec<(i64, i64)> {
    let (start, end) = range;
    let mut current_start = start;
    let mut new_ranges = Vec::new();

    while current_start <= end {
        let mut was_mapped = false;

        for &(dest_start, source_start, range_length) in mapping {
            let source_end = source_start + range_length - 1;
            let dest_end = dest_start + range_length - 1;

            // Check if the current start is within this mapping range
            if current_start >= source_start && current_start <= source_end {
                // Determine the end of the current mapped range
                let current_end = std::cmp::min(end, source_end);

                // Map the current range
                new_ranges.push((
                    dest_start + (current_start - source_start),
                    dest_end - (source_end - current_end)
                ));

                // Update the current start for the next iteration
                current_start = source_end + 1;
                was_mapped = true;
                break;
            }
        }

        // If the current start wasn't mapped, move to the next possible mapping
        if !was_mapped {
            // Add the remaining range as is and break
            new_ranges.push((current_start, end));
            break;
        }
    }

    new_ranges
}

fn process_ranges(seed_ranges: Vec<(i64, i64)>, mappings: &HashMap<String, Vec<(i64, i64, i64)>>) -> Vec<(i64, i64)> {
    let mut current_ranges = seed_ranges;
    let map_order = vec!["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];

    for map_key in map_order.iter() {
        if let Some(mapping) = mappings.get(*map_key) {
            let mut new_ranges = Vec::new();
            for range in current_ranges.iter() {
                new_ranges.append(&mut map_range_boundaries(*range, mapping));
            }
            current_ranges = new_ranges;
        }
    }

    current_ranges
}

fn main() -> io::Result<()> {
    let path = Path::new("almanac.txt");
    // let path = Path::new("almanac_test.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut mappings: HashMap<String, Vec<(i64, i64, i64)>> = HashMap::new();
    let mut seed_ranges = Vec::new();
    let mut current_map_key = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("seeds:") {
            seed_ranges = line.split(':')
                              .nth(1)
                              .unwrap()
                              .trim()
                              .split_whitespace()
                              .map(|s| s.parse::<i64>().unwrap())
                              .collect::<Vec<i64>>()
                              .chunks(2)
                              .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
                              .collect();
        } else if line.contains("map") {
            current_map_key = line.split_whitespace().next().unwrap().to_string();
            mappings.insert(current_map_key.clone(), Vec::new());
        } else if !line.is_empty() {
            let values: Vec<i64> = line.split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();
            mappings.get_mut(&current_map_key).unwrap().push((values[0], values[1], values[2]));
        }
    }

    let final_ranges = process_ranges(seed_ranges, &mappings);

    let min_location = final_ranges.iter()
                                   .map(|&(start, end)| std::cmp::min(start, end))
                                   .min()
                                   .unwrap_or(i64::MAX);

    println!("Lowest location number: {}", min_location);
    Ok(())
}
