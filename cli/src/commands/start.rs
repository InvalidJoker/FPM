use crate::{_color_output, cli::ArgParse, connection::Connection, info};

pub fn run(args: ArgParse, mut connection: Connection) {
    info!("Args: {:?}", args);
    connection.send("Hello from the client");
}
