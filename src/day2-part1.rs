use std::fs;
use std::result::Result;

#[derive(Debug)]
struct Command {
    direction: String,
    distance: i32,
}

fn parse_command(input: String) -> Result<Command, std::num::ParseIntError>{
    let split: Vec<&str> = input.split(' ').collect();
    let cmd: Command;

    match split[1].parse() {
        Ok(value) => {
            cmd = Command{
                direction: split[0].to_string(),
                distance: value,
            };
        },
        Err(e) => return Err(e),
    }
    Ok(cmd)
}

fn read_command_file(filename: String) -> Vec<Command> {
    let input = fs::read(filename).expect("Failed to read file");
    let depths = String::from_utf8(input).unwrap();
    let split: Vec<&str> = depths.split('\n').filter(|s| !s.is_empty()).collect();
    let mut output: Vec<Command> = Vec::new();

    for input in split {
        match parse_command(input.to_string()) {
            Ok(value) => {
                output.push(value)
            },
            Err(e) => panic!("Failed to parse command: {}", e),
        }
    }

    output
}

fn eval_commands(vec: Vec<Command>) -> i32 {
    let mut horizontal_pos: i32 = 0;
    let mut depth: i32 = 0;

    for cmd in vec {
        match cmd.direction.as_str() {
            "forward" => { horizontal_pos += cmd.distance },
            "up" =>  { depth -= cmd.distance },
            "down" => { depth += cmd.distance },
            _ => {},
        }
    }

    horizontal_pos * depth
}

fn main(){
    let commands = read_command_file("data/commands.txt".to_string());
    let final_pos: i32 = eval_commands(commands);
    println!("Final position: {}", final_pos);
}
