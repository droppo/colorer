use std::{
    convert::TryInto,
    env,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
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
        if let Some(p) = parser::parser::init_parser(&command) {
            let mut arguments = vec![];
            args.iter().skip(2).for_each(|arg| arguments.push(arg));

            let child = Command::new(command)
                .args(&arguments)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                // commands are filtered during init phase,
                // unwrap is considered secure here
                .unwrap();

            let child_pid = child.id().clone();
            ctrlc::set_handler(move || {
                kill(Pid::from_raw(child_pid.try_into().unwrap()), SIGINT).unwrap();
            })
            .unwrap(); // TODO manage unwrap

            let stdout_child = child.stdout.unwrap(); // TODO
            let stderr_child = child.stderr.unwrap(); // TODO

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

            stderr_th.join().unwrap(); // TODO
            stdout_th.join().unwrap(); // TODO
        } else {
            eprintln!("command {} not supported", command);
        }
    } else {
        eprintln!("failed to start colorer: at least one argument is required");
    }
}

fn main() {
    CliArgs::parse_args();
    run_colorer();
}
