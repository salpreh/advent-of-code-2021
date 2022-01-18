mod submarine;

use std::fs::File;
use std::io::{BufReader, prelude::*, Error, ErrorKind};
use std::env;

const INPUT_DATA_FILE: &str = "./resources/input_data.txt";
const TEST_INPUT_DATA_FILE: &str = "./resources/min_input_data.txt";

fn main() {
    let env_arg: Option<String> = env::args().nth(1);
    let commands = load_commands_data(get_input_file_path(env_arg));

    let mut submarine = submarine::AdvancedSubmarine::new();
    for command in commands {
        submarine.process_command(command);
    }

    println!("Final submarine position: {}x, {}y", submarine.getPosition(), submarine.getDepth());
    println!("Depth x position: {}", submarine.getPosition() * submarine.getDepth());
}

fn load_commands_data(path: &str) -> Vec<submarine::Command> {
    let data = load_data(path);
    let mut commands: Vec<submarine::Command> = Vec::new();
    for command_cod in data.iter() {
        let mut command_data = command_cod.split(" ");
        let parse_value_f = |cd: &str| {cd.parse().unwrap()};
        let  command = match command_data.nth(0).expect("Unable to split command data") {
            "forward" => Ok(submarine::Command::FORWARD(
                command_data.nth(0).map(parse_value_f).expect("msg")
            )),
            "up" => Ok(submarine::Command::UP(
                command_data.nth(0).map(parse_value_f).expect("msg")
            )),
            "down" => Ok(submarine::Command::DOWN(
                command_data.nth(0).map(parse_value_f).expect("msg")
            )),
            unk => Err(Error::new(ErrorKind::InvalidData, format!("Unknown movement: {}", unk)))
        };

        commands.push(command.unwrap());
    }

    commands
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