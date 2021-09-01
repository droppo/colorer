use std::{
    convert::TryInto,
    env,
    io::{BufRead, BufReader},
    process::{self, Command, Stdio},
    thread,
};

use cli_args::CliArgs;
use nix::{
    sys::signal::{kill, Signal::SIGINT},
    unistd::Pid,
};

mod cli_args;
mod parser;

fn run_colorer() {
    let args: Vec<String> = env::args().collect();
    if let Some(command) = args.get(1) {
        if let Some(p) = parser::parser::init_parser(command) {
            let mut arguments = vec![];
            args.iter().skip(2).for_each(|arg| arguments.push(arg));

            let child = match Command::new(command)
                .args(&arguments)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(c) => c,
                Err(_) => {
                    eprintln!("clrr: command not found: {}", args.get(1).unwrap());
                    process::exit(1);
                }
            };

            let child_pid = child.id();
            let res = ctrlc::set_handler(move || {
                kill(Pid::from_raw(child_pid.try_into().unwrap()), SIGINT).unwrap();
            });

            if res.is_err() {
                eprintln!("clrr: failed to set handler");
                process::exit(1);
            }

            let stdout_child = match child.stdout {
                Some(stdout) => stdout,
                None => {
                    eprintln!("clrr: failed to hook stdout");
                    process::exit(1)
                }
            };

            let stderr_child = match child.stderr {
                Some(stderr) => stderr,
                None => {
                    eprintln!("clrr: failed to hook stderr");
                    process::exit(1);
                }
            };

            let p_out = p.clone();
            let p_err = p.clone();

            let stdout_th = thread::spawn(move || {
                BufReader::new(stdout_child)
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| {
                        println!("{}", parser::parser::reader_handler(line, &p_out));
                    });
            });

            let stderr_th = thread::spawn(move || {
                BufReader::new(stderr_child)
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| {
                        eprintln!("{}", parser::parser::reader_handler(line, &p_err));
                    });
            });

            if stderr_th.join().is_err() || stdout_th.join().is_err() {
                eprintln!("clrr: can't join thread");
                process::exit(1);
            }
        } else {
            eprintln!("clrr: command {} not supported", command);
        }
    } else {
        eprintln!("failed to start colorer: at least one argument is required");
    }
}

fn main() {
    CliArgs::parse_args();
    run_colorer();
}
