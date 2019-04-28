use shell;

fn main() {
    loop {
        shell::print_shell_prompt();
        let commands = shell::read_commands();
        let parsed_commands = shell::split_commands(&commands);
        shell::execute_commands(parsed_commands);
    }
}
