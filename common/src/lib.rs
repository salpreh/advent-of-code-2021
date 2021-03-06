use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const INPUT_DATA_FILE: &str = "resources/input_data.txt";
const TEST_INPUT_DATA_FILE: &str = "resources/min_input_data.txt";

pub fn load_data(path: &str) -> Vec<String> {
    let file = File::open(path)
        .expect("Error loading file");
    let reader = BufReader::new(file);

    let mut data: Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(d) = line {
            data.push(String::from(d.trim()));
        };
    }

    data
}

pub fn load_data_with_type<T: FromStr>(path: &str) -> Vec<T> {
    let file = File::open(path)
        .expect("Error loading file");
    let reader = BufReader::new(file);

    let mut data: Vec<T> = Vec::new();
    for line in reader.lines() {
        if let Ok(d) = line {
            let parsed = match d.trim().parse() {
                Ok(d) => d,
                Err(_) => panic!("Unable to parse data")
            };
            data.push(parsed);
        };
    }

    data
}

pub fn get_input_file_path(config: &FileConfig) -> &str {
    let env: Option<String> = env::args().nth(1);

    match env {
        Some(s) => if s == "test" {&config.test_file} else {&config.prod_file},
        _ => &config.prod_file
    }
}

pub struct FileConfig {
    pub test_file: String,
    pub prod_file: String
}

impl FileConfig {

    pub fn new(prod_file: &str, test_file: &str) -> FileConfig {
        FileConfig {test_file: test_file.to_string(), prod_file: prod_file.to_string()}
    }

    /// Create config assuming default structure
    pub fn default(base_path: &str) -> FileConfig {
        FileConfig {
            test_file: format!("{}/{}", base_path, TEST_INPUT_DATA_FILE),
            prod_file: format!("{}/{}", base_path, INPUT_DATA_FILE)
        }
    }
}
