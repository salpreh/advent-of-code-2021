mod diagnostic_parse;

use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::env;

use diagnostic_parse::{DiagnosticReport, parse_report_line};

const INPUT_DATA_FILE: &str = "./day_3/resources/input_data.txt";
const TEST_INPUT_DATA_FILE: &str = "./day_3/resources/min_input_data.txt";

fn main() {
    let env_arg: Option<String> = env::args().nth(1);
    let report_data = load_data(get_input_file_path(env_arg));

    let diagnostic_report = DiagnosticReport::from_data(report_data);

    println!(
        "Gamma: {}\nEpsilon: {}\nPower consumption: {}\n",
        diagnostic_report.get_gamma_rate(),
        diagnostic_report.get_epsilon_rate(),
        diagnostic_report.get_power_consumption()
    );

    println!(
        "O2: {}\nCO2 {}\nLife support: {}\n",
        diagnostic_report.get_oxigen_rate(),
        diagnostic_report.get_co2_rate(),
        diagnostic_report.get_life_support_rate()
    );
}

#[deprecated()]
fn load_report_data(path: &str) -> Vec<u32> {
    load_data(path).into_iter()
        .map(|i| parse_report_line(&i))
        .collect()
}

fn load_data(path: &str) -> Vec<String> {
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

fn get_input_file_path(env: Option<String>) -> &'static str {
    match env {
        Some(s) => if s == "test" {TEST_INPUT_DATA_FILE} else {INPUT_DATA_FILE},
        _ => INPUT_DATA_FILE
    }
}
