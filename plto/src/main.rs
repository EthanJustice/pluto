// std

// crates
use clap::{App, Arg, SubCommand};
use pluto::PlutoParser;

// local

fn main() {
    let app = App::new("plto")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("validate")
                .about("Validate a given pluto file")
                .arg(Arg::with_name("INPUT").help("The file to validate")),
        )
        .get_matches();
}
