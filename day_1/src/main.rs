use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::env;

const INPUT_DATA_FILE: &str = "./resources/input_data.txt";
const TEST_INPUT_DATA_FILE: &str = "./resources/min_input_data.txt";

fn main() {
    let env_arg: Option<String> = env::args().nth(1);
    let sonar_data = load_data(get_input_file_path(env_arg));

    let depth_increase_count = windowed_measurement_depth_increase_count(&sonar_data, 3);
    println!("Number of depth increments: {}", depth_increase_count);
}

fn windowed_measurement_depth_increase_count(sonar_data: &Vec<i32>, window_size: usize) -> i32 {
    if sonar_data.len() < window_size { return 0 }

    let mut depth_increase_count = 0;
    let mut previous_measure: i32 = sonar_data[..window_size].iter().sum();
    let last_valid_idx = sonar_data.len() - window_size + 1;
    for idx in 1..last_valid_idx {
        let measure: i32 = sonar_data[idx..idx+window_size].iter().sum();
        if measure > previous_measure {
            depth_increase_count += 1;
        }

        previous_measure = measure;
    }

    depth_increase_count
}

fn single_measurement_depth_increase_count(sonar_data: &Vec<i32>) -> i32 {

    let mut depth_increase_count = 0;
    let mut previous_measure = sonar_data[0];
    for measure in &sonar_data[1..] {
        if *measure > previous_measure {
            depth_increase_count += 1;
        }

        previous_measure = *measure;
    }

    depth_increase_count
}

fn load_data(path: &str) -> Vec<i32> {
    let file = File::open(path)
        .expect("Error loading file");
    let reader = BufReader::new(file);

    let mut data: Vec<i32> = Vec::new();
    for line in reader.lines() {
        if let Ok(d) = line {
            data.push(d.trim()
                .parse()
                .expect("Unable to parse input to integer")
            );
        };
    }

    data
}

fn get_input_file_path(env: Option<String>) -> &'static str {
    match env {
        Some(s) => if s == "test" {TEST_INPUT_DATA_FILE} else {INPUT_DATA_FILE},
        _ => INPUT_DATA_FILE
    }
}