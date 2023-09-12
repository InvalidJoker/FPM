use crate::{_color_output, cli::ArgParse, connection::Connection, info};

pub fn run(args: ArgParse, _: Connection) {
    info!("Args: {:?}", args);
}
