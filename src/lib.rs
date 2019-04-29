use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::str::Split;

pub fn print_shell_prompt() {
    print!("-> ");
    io::stdout().flush().unwrap();
}

pub fn read_commands() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer
}

pub fn split_commands(result: &String) -> Split<&str> {
    result.split(";")
}

pub fn execute_commands(commands: Split<&str>) {
    for raw_command in commands {
        let command = parse_command(raw_command);
        let output = Command::new(command[0])
            .env("PATH", "/bin")
            .args(&command[1..])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();
        if output.is_err() {
            let error = output.unwrap_err();
            print!("{}\n", error);
        }
    }
}

pub fn parse_command(result: &str) -> Vec<&str> {
    let parsed_result: Vec<&str> = result.split_whitespace().collect();
    parsed_result
}
