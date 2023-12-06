use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::env;

fn map_number(number: i64, mapping: &[(i64, i64, i64)]) -> i64 {
    for &(dest_start, source_start, range_length) in mapping {
        if number >= source_start && number < source_start + range_length {
            return dest_start + (number - source_start);
        }
    }
    number
}

fn process_seed(seed: i64, mappings: &HashMap<String, Vec<(i64, i64, i64)>>) -> i64 {
    let mut current_number = seed;
    let map_order = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    for map_key in map_order.iter() {
        if let Some(mapping) = mappings.get(*map_key) {
            current_number = map_number(current_number, mapping);
        }
    }

    current_number
}

fn main() -> io::Result<()> {
    // println!("Current directory: {:?}", env::current_dir()?);
    let path = Path::new("almanac.txt");
    // let path = Path::new("./almanac_test.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut mappings: HashMap<String, Vec<(i64, i64, i64)>> = HashMap::new();
    let mut seed_ranges = Vec::new();
    let mut current_map_key = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("seeds:") {
            seed_ranges = line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
        } else if line.contains("map") {
            current_map_key = line.split_whitespace().next().unwrap().to_string();
            mappings.insert(current_map_key.clone(), Vec::new());
        } else if !line.is_empty() {
            let values: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            mappings
                .get_mut(&current_map_key)
                .unwrap()
                .push((values[0], values[1], values[2]));
        }
    }

    let mut min_location = i64::MAX;

    for i in (0..seed_ranges.len()).step_by(2) {
        let start = seed_ranges[i];
        let length = seed_ranges[i + 1];
        for seed in start..start + length {
            let location = process_seed(seed, &mappings);
            if location < min_location {
                min_location = location;
            }
        }
    }

    println!("Lowest location number: {}", min_location);
    Ok(())
}
