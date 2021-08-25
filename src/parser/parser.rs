use std::{env, sync::Arc};

use crate::decorate;

use super::{
    command::{
        df::Df, dig::Dig, docker::Docker, env::Env, free::Free, last::Last, ls::Ls, lsns::Lsns,
        nmap::Nmap, nslookup::Nslookup, ping::Ping,
    },
    decorator::Decoration,
};

#[derive(Debug)]
pub struct ColorerRegex {
    regex: &'static str,
    default_decorator: String,
    optional_decorator: Option<Vec<(&'static str, String)>>,
}

impl ColorerRegex {
    pub fn new(
        regex: &'static str,
        default_decorator: String,
        optional_decorator: Option<Vec<(&'static str, String)>>,
    ) -> ColorerRegex {
        ColorerRegex {
            regex,
            default_decorator,
            optional_decorator,
        }
    }
}

pub trait Parser: Sync + Send {
    fn regexs(&self) -> Vec<ColorerRegex>;
}

pub fn init_parser(command: &str) -> Option<Arc<dyn Parser + Sync + Send>> {
    match command {
        "ping" => Some(Arc::new(Ping)),
        "nmap" => Some(Arc::new(Nmap)),
        "docker" => {
            let subcommand = match env::args().collect::<Vec<String>>().get(2) {
                Some(sub) => Some(sub.to_owned()),
                None => None,
            };
            Some(Arc::new(Docker { subcommand }))
        }
        "df" => Some(Arc::new(Df)),
        "free" => Some(Arc::new(Free)),
        "ls" => Some(Arc::new(Ls)),
        "nslookup" => Some(Arc::new(Nslookup)),
        "dig" => Some(Arc::new(Dig)),
        "last" | "lastb" => Some(Arc::new(Last)),
        "env" => Some(Arc::new(Env)),
        "lsns" => Some(Arc::new(Lsns)),
        _ => None,
    }
}

pub fn reader_handler(line: String, parser: &Arc<dyn Parser + Sync + Send>) -> String {
    // TODO check a better way to avoid two cycles
    let mut positions = vec![];
    let mut colored_line = line.to_string();

    // 1: find all the positions
    // NOTE a variable is used to meet borrow checker requirements
    let regexs = parser.regexs();
    regexs.iter().for_each(|r| {
        onig::Regex::new(r.regex)
            .unwrap()
            .find_iter(&line)
            .for_each(|position| {
                positions.push((position, r));
            })
    });

    // 2: replace
    positions.sort_by_key(|e| e.0);
    positions.into_iter().rev().for_each(|p| {
        let part = &line[p.0 .0..p.0 .1];
        let decorator = match &p.1.optional_decorator {
            Some(decorators) => {
                let mut decorator = &p.1.default_decorator;
                for d in decorators.iter() {
                    if onig::Regex::new(d.0).unwrap().is_match(part) {
                        decorator = &d.1;
                        break;
                    }
                }
                decorator
            }
            None => &p.1.default_decorator,
        };
        colored_line.replace_range(
            p.0 .0..p.0 .1,
            format!("{}{}{}", decorator, part, decorate!(Decoration::Default)).as_str(),
        )
    });

    format!("{}", colored_line)
}
