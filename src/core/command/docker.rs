use crate::{
    core::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
    decorate,
};

#[derive(Debug)]
pub struct Docker {
    pub subcommand: Option<String>,
}

impl Parser for Docker {
    fn regexs(&self) -> Vec<crate::core::parser::ColorerRegex> {
        // let args: Vec<OsString> = CliArgs::parse_args().args;
        let regex_vector = if let Some(subcommand) = &self.subcommand {
            match subcommand.as_str() {
                "ps" | "container" => {
                    vec![
                        // image
                        ColorerRegex::new(
                            r#"(?<=(^(\S+)\s+))(\S+){3,}"#,
                            decorate!(Decoration::MagentaFgBright),
                            None,
                        ),
                        // created
                        ColorerRegex::new(
                            r#"(?<=(^\S+\s+\S+\s+\".*\"\s+))(\d|\w+\s\w+|\d+)(\s\w+){1,3}"#,
                            decorate!(Decoration::BlueFgBright),
                            None,
                        ),
                        // status
                        ColorerRegex::new(
                            r#"(\w+\s\d+\s\w+|\w+\s\(\d+\)\s\d+(\s\w+){2})"#,
                            decorate!(Decoration::GreenFgBright),
                            Some(vec![(
                                r#"\w+\s\(\d+\)\s\d+\s\w+\s\w+"#,
                                decorate!(Decoration::RedBgBright),
                            )]),
                        ),
                        // named status e.g. healthy, unhealthy
                        // FIXME unhealthy not working correctly
                        // ColorerRegex::new(
                        //     r#"\bunhealthy\b"#,
                        //     decorate!(Decoration::ReBg),
                        //     Some(vec![(r"\b(unhealthy)\b", decorate!(Decoration::ReBg))]),
                        // ),
                        // internal ports
                        ColorerRegex::new(
                            r#"(?<=(\s|>))\d+/\w+"#,
                            decorate!(Decoration::YellowFgBright),
                            None,
                        ),
                        // external
                        ColorerRegex::new(
                            r#"((\d{1,3}.){3}\d{1,3}|::):\d+"#,
                            decorate!(Decoration::GreenFgBright),
                            None,
                        ),
                        // name
                        ColorerRegex::new(
                            r#"\S+[^NAMES\n]($|\n)"#,
                            decorate!(Decoration::CyanFgBright),
                            None,
                        ),
                    ]
                }
                "search" => {
                    vec![
                        // name
                        ColorerRegex::new(
                            r"(^[^NAME]\w+(?=\s)|(?<=(^\w+\/))\S+)",
                            decorate!(Decoration::YellowFgBright, Decoration::Bold),
                            None,
                        ),
                        // Description
                        // ColorerRegex::new(r"", decorate!(Decoration::CyanFgBright), None),
                        // stars
                        ColorerRegex::new(
                            r"(?<=\s)\d+(?=\s)",
                            decorate!(Decoration::GreenFgBright),
                            Some(vec![
                                (r"\d{1}", decorate!(Decoration::MagentaFgBright)),
                                (r"\d{2}", decorate!(Decoration::YellowFgBright)),
                                (r"\d{3,}", decorate!(Decoration::GreenFgBright)),
                            ]),
                        ),
                        // official/automate
                        ColorerRegex::new(
                            r"(?<=\[)OK(?=\])",
                            decorate!(Decoration::GreenFgBright),
                            None,
                        ),
                        // underline
                        ColorerRegex::new(
                            r"(?<=^[^NAME]\S+\s)\s+(?=\s)",
                            decorate!(Decoration::BlackFgBright, Decoration::Underlined),
                            None,
                        ),
                    ]
                }
                "network" => {
                    vec![
                        // name
                        ColorerRegex::new(
                            r"(?<=(^[^NAME]\S+\s+))\S+",
                            decorate!(Decoration::MagentaFgBright),
                            Some(vec![(r"none", decorate!(Decoration::BlackFgBright))]),
                        ),
                        // driver
                        ColorerRegex::new(
                            r"(?<=(^[^NAME](\S+\s+){2}))\S+",
                            decorate!(Decoration::CyanFgBright),
                            Some(vec![
                                (r"host", decorate!(Decoration::CyanFgBright)),
                                (r"bridge", decorate!(Decoration::BlueFgBright)),
                                (r"null", decorate!(Decoration::BlackFgBright)),
                            ]),
                        ),
                    ]
                }
                _ => {
                    vec![]
                }
            }
        } else {
            vec![]
        };

        regex_vector
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        core::{
            command::docker::Docker,
            decorator::Decoration,
            parser::{reader_handler, Parser},
        },
        decorate,
    };

    #[test]
    fn docker_ps() {
        let input = vec![
            r#"12c3dedd07be   kunde21/gitea-arm   "/usr/bin/entrypoint…"   5 weeks ago   Up 5 days  3000/tcp, 0.0.0.0:222->22/tcp, :::222->22/tcp   gitea"#.to_string(),
            r#"a26d8ced06fd   eclipse-mosquitto   "/docker-entrypoint.…"   5 hours ago   Restarting (13) 47 seconds ago   message-broker"#.to_string()
        ];

        let correct_output = vec![
            format!(
                r#"12c3dedd07be   {magenta}kunde21/gitea-arm{reset}   "/usr/bin/entrypoint…"   {blue}5 weeks ago{reset}   {green}Up 5 days{reset}  {yellow}3000/tcp{reset}, {green}0.0.0.0:222{reset}->{yellow}22/tcp{reset}, {green}:::222{reset}->{yellow}22/tcp{reset}   {cyan}gitea{reset}"#,
                magenta = decorate!(Decoration::MagentaFgBright),
                blue = decorate!(Decoration::BlueFgBright),
                green = decorate!(Decoration::GreenFgBright),
                yellow = decorate!(Decoration::YellowFgBright),
                cyan = decorate!(Decoration::CyanFgBright),
                reset = decorate!(Decoration::Default)
            ),
            format!(
                r#"a26d8ced06fd   {magenta}eclipse-mosquitto{default}   "/docker-entrypoint.…"   {blue}5 hours ago{default}   {red}Restarting (13) 47 seconds ago{default}   {cyan}message-broker{default}"#,
                magenta = decorate!(Decoration::MagentaFgBright),
                blue = decorate!(Decoration::BlueFgBright),
                red = decorate!(Decoration::RedBgBright),
                cyan = decorate!(Decoration::CyanFgBright),
                default = decorate!(Decoration::Default)
            ),
        ];

        fn test_init() -> Arc<dyn Parser + Sync + Send> {
            Arc::new(Docker {
                subcommand: Some("ps".to_owned()),
            })
        }

        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &test_init())
            );
        }
    }

    #[test]
    fn docker_search() {
        let input = vec![
            "NAME                                    DESCRIPTION                                     STARS               OFFICIAL            AUTOMATED",
            "postgres                                 The PostgreSQL object-relational database sy…   9804                [OK]",
            "sameersbn/postgresql                                                                      159                                     [OK]",
            "paintedfox/postgresql                     A docker image for running Postgresql.          77                                      [OK]",
            "centos/postgresql-96-centos7              PostgreSQL is an advanced Object-Relational …   45 ",
            "centos/postgresql-95-centos7              PostgreSQL is an advanced Object-Relational …   6  "
        ];

        let correct_output = vec![
            format!("NAME                                    DESCRIPTION                                     STARS               OFFICIAL            AUTOMATED",),
            format!("{yellow}{bold}postgres{default} {black}{underline}                               {default} The PostgreSQL object-relational database sy…   {green}9804{default}                [{green}OK{default}]",
                    yellow = decorate!(Decoration::YellowFgBright),
                    bold = decorate!(Decoration::Bold),
                    black = decorate!(Decoration::BlackFgBright),
                    underline = decorate!(Decoration::Underlined),
                    green = decorate!(Decoration::GreenFgBright),
                    default = decorate!(Decoration::Default)
            ),
            format!("sameersbn/{yellow}{bold}postgresql{default} {black}{underline}                                                                    {default} {green}159{default}                                     [{green}OK{default}]",
                    yellow = decorate!(Decoration::YellowFgBright),
                    bold = decorate!(Decoration::Bold),
                    black = decorate!(Decoration::BlackFgBright),
                    underline = decorate!(Decoration::Underlined),
                    green = decorate!(Decoration::GreenFgBright),
                    default = decorate!(Decoration::Default)),
            format!("paintedfox/{yellow}{bold}postgresql{default} {black}{underline}                   {default} A docker image for running Postgresql.          {yellow}77{default}                                      [{green}OK{default}]",
                    yellow = decorate!(Decoration::YellowFgBright),
                    bold = decorate!(Decoration::Bold),
                    black = decorate!(Decoration::BlackFgBright),
                    underline = decorate!(Decoration::Underlined),
                    green = decorate!(Decoration::GreenFgBright),
                    default = decorate!(Decoration::Default)),
            format!("centos/{yellow}{bold}postgresql-96-centos7{default} {black}{underline}            {default} PostgreSQL is an advanced Object-Relational …   {yellow}45{default} ",
                    yellow = decorate!(Decoration::YellowFgBright),
                    bold = decorate!(Decoration::Bold),
                    black = decorate!(Decoration::BlackFgBright),
                    underline = decorate!(Decoration::Underlined),
                    default = decorate!(Decoration::Default)),
            format!("centos/{yellow}{bold}postgresql-95-centos7{default} {black}{underline}            {default} PostgreSQL is an advanced Object-Relational …   {magenta}6{default}  ",
                    yellow = decorate!(Decoration::YellowFgBright),
                    bold = decorate!(Decoration::Bold),
                    black = decorate!(Decoration::BlackFgBright),
                    underline = decorate!(Decoration::Underlined),
                    magenta = decorate!(Decoration::MagentaFgBright),
                    default = decorate!(Decoration::Default)),
        ];

        fn test_init() -> Arc<dyn Parser + Sync + Send> {
            Arc::new(Docker {
                subcommand: Some("search".to_owned()),
            })
        }

        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &test_init())
            );
        }
    }

    #[test]
    fn docker_network() {
        let input = vec![
            "NETWORK ID     NAME                      DRIVER    SCOPE",
            "ee17b8ac62ab   bridge                    bridge    local",
            "c31b98b076a0   host                      host      local",
            "9aa4c404d9dc   none                      null      local",
            "0cb5c7d7e926   broker-test_default       bridge    local",
        ];

        let correct_output = vec![
            format!("NETWORK ID     NAME                      DRIVER    SCOPE"),
            format!("ee17b8ac62ab   {magenta}bridge{default}                    {blue}bridge{default}    local", magenta = decorate!(Decoration::MagentaFgBright), default = decorate!(Decoration::Default), blue = decorate!(Decoration::BlueFgBright)),
            format!("c31b98b076a0   {magenta}host{default}                      {cyan}host{default}      local", magenta = decorate!(Decoration::MagentaFgBright), default = decorate!(Decoration::Default), cyan = decorate!(Decoration::CyanFgBright)),
            format!("9aa4c404d9dc   {black}none{default}                      {black}null{default}      local", default = decorate!(Decoration::Default), black = decorate!(Decoration::BlackFgBright)),
            format!("0cb5c7d7e926   {magenta}broker-test_default{default}       {blue}bridge{default}    local", magenta = decorate!(Decoration::MagentaFgBright), default = decorate!(Decoration::Default), blue = decorate!(Decoration::BlueFgBright)),
        ];

        fn test_init() -> Arc<dyn Parser + Sync + Send> {
            Arc::new(Docker {
                subcommand: Some("network".to_owned()),
            })
        }

        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &test_init())
            );
        }
    }
}
