use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    author = "droppo - github.com/droppo",
    name = "clrr - colorer",
    about = "Simple command line utility to highlight the output of commands with custom color schemas."
)]
pub struct CliArgs {
    pub command: Vec<String>,
}
