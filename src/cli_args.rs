// use std::{env, ffi::OsString, process::exit};

// use pico_args::Arguments;

// const HELP: (&str, &str) = (
//     "\
// colorer",
//     "\n
// Colorer is a simple command line utility useful to add color to commands that do
// not have it by default.

// USAGE:
//     colorer [OPTIONS] command [ARGS]...

// FLAGS:
//     --help      Print this help and exit.
//     --version   Print version information and exit.
// ",
// );

// #[derive(Debug)]
// pub struct CliArgs {
//     pub args: Vec<OsString>,
// }

// impl CliArgs {
//     pub fn parse_args() -> CliArgs {
//         let mut args = Arguments::from_env();

//         if args.contains("--help") {
//             println!("{}{}{}", HELP.0, env!("CARGO_PKG_VERSION"), HELP.1);
//             exit(0);
//         }

//         if args.contains("--version") {
//             println!("colorer {}", env!("CARGO_PKG_VERSION"));
//             exit(0);
//         }

//         CliArgs {
//             args: args.finish(),
//         }
//     }

//     pub fn print_usage() {
//         println!("usage: colorer [OPTIONS] command ...")
//     }
// }

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CliArgs {
    // #[structopt(long = "file", short = "file")]
    // color_file: Option<String>
    pub command: Vec<String>,
}
