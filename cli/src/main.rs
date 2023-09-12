mod cli;
pub mod commands;
pub mod utils;

fn main() {
    let cmds = commands::init();
    let args = cli::ArgParse::new();

    match cmds.is_valid(args.get_command()) {
        Some(cmd) => {
            (cmd.get_run())(args);
        }
        None => {
            throw!("Command not found: {}", args.get_command());
        }
    }
}
