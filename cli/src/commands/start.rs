use crate::{_color_output, cli::ArgParse, info, utils::connection::Connection};

pub fn run(args: ArgParse, mut connection: Connection) {
    info!("Args: {:?}", args);
    connection.send("Hello from the client");
}
