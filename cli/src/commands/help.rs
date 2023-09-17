use crate::{
    _color_output,
    cli::ArgParse,
    utils::connection::Connection,
    info,
    error,
};

pub fn run(args: ArgParse, _: Connection) {
    // get all commands
    let cmds = crate::commands::init();

    // if no command was specified, print help

    if args.get_command().is_empty() {
        // list all commands
        info!("Available commands:");
        for cmd in &cmds.commands {
            info!("  {} - {}", cmd.get_name(), cmd.get_description());
        }
        return;
    }

    // if command was specified, print help for that command
    if let Some(cmd) = cmds.is_valid(args.get_command()) {
        info!("Help for command '{}':", cmd.get_name());
        info!("  Description: {}", cmd.get_description());
        info!("  Aliases: {}", cmd.get_aliases().join(", "));
        return;
    }

    // if command was specified but not found, print error
    error!("Command not found: {} (try 'help')", args.get_command());
}
