use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    author = "droppo - github.com/droppo",
    name = "clrr - colorer",
    about = "Simple command line utility to highlight the output of commands with custom color schemas.",
    rename_all = "kebab-case"
)]
pub struct CliArgs {
    #[structopt(long, short)]
    pub file: Option<PathBuf>,
    pub command: Vec<String>,
}
