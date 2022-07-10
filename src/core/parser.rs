use std::{collections::HashMap, fs::File, io::Read, path::Path, sync::Arc};

use serde_derive::{Deserialize, Serialize};

use super::decorator::{decorate, Decoration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    disabling_flags: Option<Vec<String>>,
    subcommand: Option<HashMap<String, Vec<ColorerRegex>>>,
    command: Option<Vec<ColorerRegex>>,
}

impl Command {
    pub fn _new(
        disabling_flags: Option<Vec<String>>,
        subcommand: Option<HashMap<String, Vec<ColorerRegex>>>,
        command: Option<Vec<ColorerRegex>>,
    ) -> Self {
        Self {
            disabling_flags,
            subcommand,
            command,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorerRegex {
    regex: String,
    default_decorator: Vec<Decoration>,
    optional_decorators: Option<Vec<(String, Vec<Decoration>)>>,
}

impl ColorerRegex {
    pub fn _new(
        regex: &str,
        default_decorator: Vec<Decoration>,
        optional_decorators: Option<Vec<(String, Vec<Decoration>)>>,
    ) -> Self {
        Self {
            regex: regex.to_string(),
            default_decorator,
            optional_decorators,
        }
    }
}

pub fn init_parser(
    command: &str,
    args: &[String],
) -> Result<Option<Arc<Vec<ColorerRegex>>>, Box<dyn std::error::Error>> {
    let config = Path::new("/etc").join("colorer");
    if config.exists() {
        let config_path = config.join(format!("{}.toml", command));

        if config_path.exists() {
            let mut config_file = File::open(config_path)?;
            let mut configs = String::new();
            config_file.read_to_string(&mut configs)?;
            let pattern: Command = toml::from_str(&configs)?;

            // TODO check if the loaded configs contains any disabling flag
            if let Some(values) = pattern.command {
                return Ok(Some(Arc::new(values)));
            } else if let Some(values) = pattern.subcommand {
                if let Some(subcommand) = args.get(1) {
                    if let Some(colors) = values.get(subcommand) {
                        return Ok(Some(Arc::new(colors.to_vec())));
                    }
                }
            }
        }
    }

    Ok(None)
}

pub fn reader_handler(line: String, parser: &Arc<Vec<ColorerRegex>>) -> String {
    let mut positions = vec![];
    let mut colored_line = line.to_string();

    // 1: find all the positions
    parser.iter().for_each(|r| {
        if let Ok(re) = onig::Regex::new(&r.regex) {
            re.find_iter(&line).for_each(|position| {
                positions.push((position, r));
            })
        }
    });

    // 2: replace
    positions.sort_by_key(|e| e.0);
    positions.into_iter().rev().for_each(|p| {
        let part = &line[p.0 .0..p.0 .1];
        let decorator = match &p.1.optional_decorators {
            Some(decorators) => {
                let mut decorator = &p.1.default_decorator;
                for d in decorators.iter() {
                    if let Ok(re) = onig::Regex::new(&d.0.to_string()) {
                        if re.is_match(part) {
                            decorator = &d.1;
                            break;
                        }
                    }
                }
                decorator
            }
            None => &p.1.default_decorator,
        };
        colored_line.replace_range(
            p.0 .0..p.0 .1,
            format!("{}{}{}", decorate(decorator), part, Decoration::Default).as_str(),
        )
    });

    colored_line
}
