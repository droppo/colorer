use std::{env, ffi::OsString, process::exit};

use pico_args::Arguments;

const HELP: (&str, &str) = (
    "\
colorer (clrr) - ",
    "\n
Colorer is a simple command line utility useful to add color to commands that do
not have it by default.

Supported commands: df, dig, docker, env, free, last, lastb, ls, lsns, nmap,
nslookup, ping.

USAGE:
    clrr [FLAGS] ...

FLAGS
    --help      Print this help.
    --version   Print version information and exit.
",
);

#[derive(Debug)]
pub struct CliArgs {
    pub args: Vec<OsString>,
}

impl CliArgs {
    pub fn parse_args() -> CliArgs {
        let mut args = Arguments::from_env();

        if args.contains("--help") {
            println!("{}{}{}", HELP.0, env!("CARGO_PKG_VERSION"), HELP.1);
            exit(0);
        }

        if args.contains("--version") {
            println!("colorer (clrr) {}", env!("CARGO_PKG_VERSION"));
            exit(0);
        }

        CliArgs {
            args: args.finish(),
        }
    }
}
