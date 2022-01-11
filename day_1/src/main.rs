use std::fs::File;
use std::io::{BufReader, prelude::*};

const INPUT_DATA_FILE: &str = "./resources/input_data.txt";
const TEST_INPUT_DATA_FILE: &str = "./resources/min_input_data.txt";

fn main() {
    let sonar_data = load_data(INPUT_DATA_FILE);

    let mut depth_increase_count = 0;
    let mut previous_measure = sonar_data[0];
    for measure in &sonar_data[1..] {
        if *measure > previous_measure {
            depth_increase_count += 1;
        }

        previous_measure = *measure;
    }

    println!("Number of depth increments: {}", depth_increase_count);
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