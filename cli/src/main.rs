mod cli;
pub mod commands;
pub mod connection;
pub mod utils;

fn main() {
    let cmds = commands::init();
    let args = cli::ArgParse::new();

    let conn = connection::Connection::new("127.0.0.1:8080");

    match cmds.is_valid(args.get_command()) {
        Some(cmd) => {
            (cmd.get_run())(args, conn);
        }
        None => {
            error!("Command not found: {} (try 'help')", args.get_command());
        }
    }
}
