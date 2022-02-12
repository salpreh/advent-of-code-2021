use std::io::{Error, ErrorKind};

use common::{FileConfig, get_input_file_path, load_data};

mod submarine;

const INPUT_DATA_FILE: &str = "./day_2/resources/input_data.txt";
const TEST_INPUT_DATA_FILE: &str = ".day_2/resources/min_input_data.txt";

fn main() {
    let config = FileConfig::new(INPUT_DATA_FILE, TEST_INPUT_DATA_FILE);
    let commands = load_commands_data(get_input_file_path(config));

    let mut submarine = submarine::AdvancedSubmarine::new();
    for command in commands {
        submarine.process_command(command);
    }

    println!("Final submarine position: {}x, {}y", submarine.get_position(), submarine.get_depth());
    println!("Depth x position: {}", submarine.get_position() * submarine.get_depth());
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