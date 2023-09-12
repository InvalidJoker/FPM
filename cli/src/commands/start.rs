use crate::{_color_output, cli::ArgParse, info, connection::Connection};

pub fn run(args: ArgParse, connection: Connection) {
    info!("Args: {:?}", args);
}
