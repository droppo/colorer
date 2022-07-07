use std::{
    convert::TryInto,
    io::{BufRead, BufReader},
    process::{self, Command, Stdio},
    thread,
};

use cli_args::CliArgs;
use nix::{
    sys::signal::{kill, Signal::SIGINT},
    unistd::Pid,
};
use structopt::StructOpt;

mod cli_args;
mod core;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CliArgs::from_args();

    if let Some(command) = cli.command.get(0) {
        let mut arguments = vec![];

        cli.command
            .iter()
            .skip(1)
            .for_each(|arg| arguments.push(arg));

        if let Some(p) = core::parser::init_parser(command, &cli.command)? {
            let child = Command::new(command)
                .args(&arguments)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

            let child_pid = child.id();
            ctrlc::set_handler(move || {
                kill(Pid::from_raw(child_pid.try_into().unwrap()), SIGINT).unwrap();
            })?;

            let stdout_child = child.stdout.unwrap_or_else(|| {
                eprint!("failed to hook stdout");
                process::exit(1);
            });

            let stderr_child = child.stderr.unwrap_or_else(|| {
                eprint!("failed to hook stdout");
                process::exit(1);
            });

            let p_out = p.clone();
            let p_err = p;

            let stdout_th = thread::spawn(move || {
                BufReader::new(stdout_child)
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| {
                        println!("{}", core::parser::reader_handler(line, &p_out));
                    });
            });

            let stderr_th = thread::spawn(move || {
                BufReader::new(stderr_child)
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| {
                        eprintln!("{}", core::parser::reader_handler(line, &p_err));
                    });
            });

            if stderr_th.join().is_err() || stdout_th.join().is_err() {
                eprintln!("can't join thread");
                process::exit(1);
            }
        } else {
            Command::new(command).args(&arguments).spawn()?.wait()?;
        }
    } else {
        eprintln!("At least one argument is required");
        println!("run clrr --help for help");
        process::exit(1);
    }

    Ok(())
}

fn main() {
    CliArgs::from_args();
    if let Err(err) = run() {
        eprint!("{}", err);
        process::exit(1);
    }
}
