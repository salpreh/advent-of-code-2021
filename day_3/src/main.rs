mod diagnostic_parse;

use common::{FileConfig, get_input_file_path, load_data};

use diagnostic_parse::{DiagnosticReport, parse_report_line};

fn main() {
    let config = FileConfig::default("./day_3");
    let report_data = load_data(get_input_file_path(&config));

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