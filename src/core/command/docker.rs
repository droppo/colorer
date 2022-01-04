use std::collections::HashMap;

use crate::core::{
    decorator::Decoration,
    parser::{ColorerRegex, Command},
};

#[derive(Debug)]
pub struct Docker {
    pub subcommand: Option<String>,
}

impl Docker {
    pub fn regexs() -> Command {
        // let args: Vec<OsString> = CliArgs::parse_args().args;

        let mut hm: HashMap<String, Vec<ColorerRegex>> = HashMap::new();

        let ps = vec![
            // image
            ColorerRegex::new(
                r#"(?<=(^(\S+)\s+))(\S+){3,}"#,
                vec![Decoration::MagentaFgBright],
                None,
            ),
            // created
            ColorerRegex::new(
                r#"(?<=(^\S+\s+\S+\s+\".*\"\s+))(\d|\w+\s\w+|\d+)(\s\w+){1,3}"#,
                vec![Decoration::BlueFgBright],
                None,
            ),
            // status
            ColorerRegex::new(
                r#"(\w+\s(\d|about a)+\s\w+|\w+\s\(\d+\)\s\d+(\s\w+){2})"#,
                vec![Decoration::GreenFgBright],
                Some(vec![(
                    r#"\w+\s\(\d+\)\s\d+\s\w+\s\w+"#.to_string(),
                    vec![Decoration::RedBgBright],
                )]),
            ),
            // named status e.g. healthy, unhealthy
            // FIXME unhealthy not working correctly
            // ColorerRegex::new(
            //     r#"\bunhealthy\b"#,
            //     vec!(Decoration::ReBg),
            //     Some(vec![(r"\b(unhealthy)\b", vec!(Decoration::ReBg))]),
            // ),
            // internal ports
            ColorerRegex::new(
                r#"(?<=(\s|>))\d+/\w+"#,
                vec![Decoration::YellowFgBright],
                None,
            ),
            // external
            ColorerRegex::new(
                r#"((\d{1,3}.){3}\d{1,3}|::):\d+"#,
                vec![Decoration::GreenFgBright],
                None,
            ),
            // name
            ColorerRegex::new(
                r#"\S+[^NAMES\n]($|\n)"#,
                vec![Decoration::CyanFgBright],
                None,
            ),
        ];
        hm.insert("ps".to_string(), ps);

        let container = vec![
            // image
            ColorerRegex::new(
                r#"(?<=(^(\S+)\s+))(\S+){3,}"#,
                vec![Decoration::MagentaFgBright],
                None,
            ),
            // created
            ColorerRegex::new(
                r#"(?<=(^\S+\s+\S+\s+\".*\"\s+))(\d|\w+\s\w+|\d+)(\s\w+){1,3}"#,
                vec![Decoration::BlueFgBright],
                None,
            ),
            // status
            ColorerRegex::new(
                r#"(\w+\s(\d|about a)+\s\w+|\w+\s\(\d+\)\s\d+(\s\w+){2})"#,
                vec![Decoration::GreenFgBright],
                Some(vec![(
                    r#"\w+\s\(\d+\)\s\d+\s\w+\s\w+"#.to_string(),
                    vec![Decoration::RedBgBright],
                )]),
            ),
            // named status e.g. healthy, unhealthy
            // FIXME unhealthy not working correctly
            // ColorerRegex::new(
            //     r#"\bunhealthy\b"#,
            //     vec!(Decoration::ReBg),
            //     Some(vec![(r"\b(unhealthy)\b", vec!(Decoration::ReBg))]),
            // ),
            // internal ports
            ColorerRegex::new(
                r#"(?<=(\s|>))\d+/\w+"#,
                vec![Decoration::YellowFgBright],
                None,
            ),
            // external
            ColorerRegex::new(
                r#"((\d{1,3}.){3}\d{1,3}|::):\d+"#,
                vec![Decoration::GreenFgBright],
                None,
            ),
            // name
            ColorerRegex::new(
                r#"\S+[^NAMES\n]($|\n)"#,
                vec![Decoration::CyanFgBright],
                None,
            ),
        ];
        hm.insert("container".to_string(), container);

        let search = vec![
            // name
            ColorerRegex::new(
                r"(^[^NAME]\w+(?=\s)|(?<=(^\w+\/))\S+)",
                vec![Decoration::YellowFgBright, Decoration::Bold],
                None,
            ),
            // Description
            // ColorerRegex::new(r"", vec!(Decoration::CyanFgBright), None),
            // stars
            ColorerRegex::new(
                r"(?<=\s)\d+(?=\s)",
                vec![Decoration::GreenFgBright],
                Some(vec![
                    (r"\d{1}".to_string(), vec![Decoration::MagentaFgBright]),
                    (r"\d{2}".to_string(), vec![Decoration::YellowFgBright]),
                    (r"\d{3,}".to_string(), vec![Decoration::GreenFgBright]),
                ]),
            ),
            // official/automate
            ColorerRegex::new(r"(?<=\[)OK(?=\])", vec![Decoration::GreenFgBright], None),
            // underline
            ColorerRegex::new(
                r"(?<=^[^NAME]\S+\s)\s+(?=\s)",
                vec![Decoration::BlackFgBright, Decoration::Underlined],
                None,
            ),
        ];
        hm.insert("search".to_string(), search);

        let network = vec![
            // name
            ColorerRegex::new(
                r"(?<=(^[^NAME]\S+\s+))\S+",
                vec![Decoration::MagentaFgBright],
                Some(vec![(r"none".to_string(), vec![Decoration::BlackFgBright])]),
            ),
            // driver
            ColorerRegex::new(
                r"(?<=(^[^NAME](\S+\s+){2}))\S+",
                vec![Decoration::CyanFgBright],
                Some(vec![
                    (r"host".to_string(), vec![Decoration::CyanFgBright]),
                    (r"bridge".to_string(), vec![Decoration::BlueFgBright]),
                    (r"null".to_string(), vec![Decoration::BlackFgBright]),
                ]),
            ),
        ];
        hm.insert("network".to_string(), network);

        let build = vec![
            // step
            ColorerRegex::new(
                r"(?<=(^Step\s+))\d+",
                vec![Decoration::YellowFgBright],
                None,
            ),
            // total steps
            ColorerRegex::new(
                r"(?<=(^Step\s+\d+\/))\d+",
                vec![Decoration::GreenFgBright],
                None,
            ),
            // docker build commands
            ColorerRegex::new(
                r"\b(FROM|RUN|LABEL|CMD|ARG|ADD|COPY|ENV|ENTRYPOINT|LABEL|EXPOSE|WORKDIR|VOLUME|HEALTHCHECK|ONBUILD)\b",
                vec![Decoration::MagentaFg, Decoration::Bold],
                None,
            ),
            // status message
            ColorerRegex::new(r"(?<=^Status:\s+).*", vec![Decoration::Bold], None),
            // arrow infos
            ColorerRegex::new(r"--->.*", vec![Decoration::BlackFgBright], None),
        ];
        hm.insert("build".to_string(), build);

        let stats = vec![
            // name
            ColorerRegex::new(
                r"(?<=^\S+\s+)\S+",
                vec![Decoration::MagentaFgBright],
                // FIXME find a better way
                // the problem is given by the `clear` before CONTAINER
                Some(vec![(r"ID".to_string(), vec![Decoration::Default])]),
            ),
            // cpu % mem %
            ColorerRegex::new(
                r"\b\d+(.\d+)?%",
                vec![Decoration::GreenFgBright], // default 0 to 49.99%
                Some(vec![
                    (
                        r"\b[5-7]\d(.\d+)%".to_string(),
                        vec![Decoration::YellowFgBright],
                    ), // 50 to 79.99%
                    (
                        r"(\b[8]\d|[9][0-6])(.\d+)%".to_string(),
                        vec![Decoration::MagentaBgBright],
                    ), // 80 to 96,99%
                    (
                        r"(\b[9][7-9]|100)(.\d+)%".to_string(),
                        vec![Decoration::RedBgBright],
                    ), // 97 to 100%
                ]),
            ),
            // mem usage, limit, net i/o, block i/o // TODO useful?
            ColorerRegex::new(
                r"\d+(.\d+)?(B|k(i)?B|M(i)?B|G(i)?B)",
                vec![Decoration::RedBgBright],
                Some(vec![
                    (
                        r"\d+(.\d+)?(B|k(i)?B)|[0-4]\d(.\d+)?(M(i)?B)".to_string(),
                        vec![Decoration::GreenFgBright],
                    ),
                    (
                        r"[5-9]\d(\d)?(.\d?)(M(i)?B)".to_string(),
                        vec![Decoration::YellowFgBright],
                    ),
                ]),
            ),
            // pids
            ColorerRegex::new(
                r"\d+$",
                vec![Decoration::RedFgBright],
                Some(vec![
                    (r"[1][0-9]".to_string(), vec![Decoration::YellowFgBright]),
                    (r"[0-9]".to_string(), vec![Decoration::GreenFgBright]),
                ]),
            ),
        ];
        hm.insert("stats".to_string(), stats);

        let pull = vec![
            // pulling/veryfing
            ColorerRegex::new(
                r"(Pulling|Verifying).*",
                vec![Decoration::YellowFgBright],
                None,
            ),
            // waiting
            ColorerRegex::new(r"Waiting", vec![Decoration::BlackFgBright], None),
            // download complete
            ColorerRegex::new(r"Download complete", vec![Decoration::GreenFgBright], None),
            // pull complate
            ColorerRegex::new(r"Pull complete", vec![Decoration::GreenBgBright], None),
        ];
        hm.insert("pull".to_string(), pull);

        Command::new(None, Some(hm), None)
    }
}

/*#[cfg(test)]
mod tests {
    use std::{sync::Arc, vec};

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
            r#"12c3dedd07be   kunde21/gitea-arm   "/usr/bin/entrypoint…"   5 weeks ago   Up about a minute  3000/tcp, 0.0.0.0:222->22/tcp, :::222->22/tcp   gitea"#.to_string(),
            r#"a26d8ced06fd   eclipse-mosquitto   "/docker-entrypoint.…"   5 hours ago   Restarting (13) 47 seconds ago   message-broker"#.to_string()
        ];

        let correct_output = vec![
            format!(
                r#"12c3dedd07be   {magenta}kunde21/gitea-arm{reset}   "/usr/bin/entrypoint…"   {blue}5 weeks ago{reset}   {green}Up about a minute{reset}  {yellow}3000/tcp{reset}, {green}0.0.0.0:222{reset}->{yellow}22/tcp{reset}, {green}:::222{reset}->{yellow}22/tcp{reset}   {cyan}gitea{reset}"#,
                magenta = vec!(Decoration::MagentaFgBright),
                blue = vec!(Decoration::BlueFgBright),
                green = vec!(Decoration::GreenFgBright),
                yellow = vec!(Decoration::YellowFgBright),
                cyan = vec!(Decoration::CyanFgBright),
                reset = vec!(Decoration::Default)
            ),
            format!(
                r#"a26d8ced06fd   {magenta}eclipse-mosquitto{default}   "/docker-entrypoint.…"   {blue}5 hours ago{default}   {red}Restarting (13) 47 seconds ago{default}   {cyan}message-broker{default}"#,
                magenta = vec!(Decoration::MagentaFgBright),
                blue = vec!(Decoration::BlueFgBright),
                red = vec!(Decoration::RedBgBright),
                cyan = vec!(Decoration::CyanFgBright),
                default = vec!(Decoration::Default)
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
                    yellow = vec!(Decoration::YellowFgBright),
                    bold = vec!(Decoration::Bold),
                    black = vec!(Decoration::BlackFgBright),
                    underline = vec!(Decoration::Underlined),
                    green = vec!(Decoration::GreenFgBright),
                    default = vec!(Decoration::Default)
            ),
            format!("sameersbn/{yellow}{bold}postgresql{default} {black}{underline}                                                                    {default} {green}159{default}                                     [{green}OK{default}]",
                    yellow = vec!(Decoration::YellowFgBright),
                    bold = vec!(Decoration::Bold),
                    black = vec!(Decoration::BlackFgBright),
                    underline = vec!(Decoration::Underlined),
                    green = vec!(Decoration::GreenFgBright),
                    default = vec!(Decoration::Default)),
            format!("paintedfox/{yellow}{bold}postgresql{default} {black}{underline}                   {default} A docker image for running Postgresql.          {yellow}77{default}                                      [{green}OK{default}]",
                    yellow = vec!(Decoration::YellowFgBright),
                    bold = vec!(Decoration::Bold),
                    black = vec!(Decoration::BlackFgBright),
                    underline = vec!(Decoration::Underlined),
                    green = vec!(Decoration::GreenFgBright),
                    default = vec!(Decoration::Default)),
            format!("centos/{yellow}{bold}postgresql-96-centos7{default} {black}{underline}            {default} PostgreSQL is an advanced Object-Relational …   {yellow}45{default} ",
                    yellow = vec!(Decoration::YellowFgBright),
                    bold = vec!(Decoration::Bold),
                    black = vec!(Decoration::BlackFgBright),
                    underline = vec!(Decoration::Underlined),
                    default = vec!(Decoration::Default)),
            format!("centos/{yellow}{bold}postgresql-95-centos7{default} {black}{underline}            {default} PostgreSQL is an advanced Object-Relational …   {magenta}6{default}  ",
                    yellow = vec!(Decoration::YellowFgBright),
                    bold = vec!(Decoration::Bold),
                    black = vec!(Decoration::BlackFgBright),
                    underline = vec!(Decoration::Underlined),
                    magenta = vec!(Decoration::MagentaFgBright),
                    default = vec!(Decoration::Default)),
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
            format!("ee17b8ac62ab   {magenta}bridge{default}                    {blue}bridge{default}    local", magenta = vec!(Decoration::MagentaFgBright), default = vec!(Decoration::Default), blue = vec!(Decoration::BlueFgBright)),
            format!("c31b98b076a0   {magenta}host{default}                      {cyan}host{default}      local", magenta = vec!(Decoration::MagentaFgBright), default = vec!(Decoration::Default), cyan = vec!(Decoration::CyanFgBright)),
            format!("9aa4c404d9dc   {black}none{default}                      {black}null{default}      local", default = vec!(Decoration::Default), black = vec!(Decoration::BlackFgBright)),
            format!("0cb5c7d7e926   {magenta}broker-test_default{default}       {blue}bridge{default}    local", magenta = vec!(Decoration::MagentaFgBright), default = vec!(Decoration::Default), blue = vec!(Decoration::BlueFgBright)),
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

    #[test]
    fn docker_build() {
        let input = vec![
            "Status: Downloaded newer image for",
            "Step 1/8 : FROM debian:latest",
            "Step 2/8 : WORKDIR /",
            " ---> Using cache",
        ];

        let correct_output = vec![
            format!(
                "Status: {bold}Downloaded newer image for{default}",
                default = vec!(Decoration::Default),
                bold = vec!(Decoration::Bold)
            ),
            format!(
                "Step {yellow}1{default}/{green}8{default} : {magenta}{bold}FROM{default} debian:latest",
                default = vec!(Decoration::Default),
                yellow = vec!(Decoration::YellowFgBright),
                green = vec!(Decoration::GreenFgBright),
                magenta = vec!(Decoration::MagentaFg),
                bold = vec!(Decoration::Bold)
            ),
            format!(
                "Step {yellow}2{default}/{green}8{default} : {magenta}{bold}WORKDIR{default} /",
                default = vec!(Decoration::Default),
                yellow = vec!(Decoration::YellowFgBright),
                green = vec!(Decoration::GreenFgBright),
                magenta = vec!(Decoration::MagentaFg),
                bold = vec!(Decoration::Bold)
            ),
            format!(
                " {black}---> Using cache{default}",
                default = vec!(Decoration::Default),
                black = vec!(Decoration::BlackFgBright)
            )
        ];

        fn test_init() -> Arc<dyn Parser + Sync + Send> {
            Arc::new(Docker {
                subcommand: Some("build".to_owned()),
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
    fn docker_stats() {
        let input = vec![
            "CONTAINER ID   NAME                          CPU %     MEM USAGE / LIMIT   MEM %     NET I/O           BLOCK I/O         PIDS",
            "e58a7a049d04   nginx                         99.56%     0B / 0B             55.00%     53.2MiB / 27.9MB   0B / 0B           5"
        ];

        let correct_output = vec![
            format!("CONTAINER {default}ID{default}   NAME                          CPU %     MEM USAGE / LIMIT   MEM %     NET I/O           BLOCK I/O         PIDS", default = vec!(Decoration::Default)),
            format!("e58a7a049d04   {magenta}nginx{default}                         {red}99.56%{default}     {green}0B{default} / {green}0B{default}             {yellow}55.00%{default}     {yellow}53.2MiB{default} / {green}27.9MB{default}   {green}0B{default} / {green}0B{default}           {green}5{default}",
                    default = vec!(Decoration::Default),
                    red = vec!(Decoration::RedBgBright),
                    magenta = vec!(Decoration::MagentaFgBright),
                    green = vec!(Decoration::GreenFgBright),
                    yellow = vec!(Decoration::YellowFgBright)
            )
        ];

        fn test_init() -> Arc<dyn Parser + Sync + Send> {
            Arc::new(Docker {
                subcommand: Some("stats".to_owned()),
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
    fn docker_pull() {
        let input = vec![
            "latest: Pulling from library/mysql",
            "07aded7c29c6: Pulling fs layer",
            "635b0b84d686: Waiting",
            "1b7cb4d6fe05: Verifying Checksum",
            "6d24c7242d02: Download complete",
            "07aded7c29c6: Pull complete",
        ];

        let correct_output = vec![
            format!(
                "latest: {yellow}Pulling from library/mysql{default}",
                yellow = vec!(Decoration::YellowFgBright),
                default = vec!(Decoration::Default)
            ),
            format!(
                "07aded7c29c6: {yellow}Pulling fs layer{default}",
                yellow = vec!(Decoration::YellowFgBright),
                default = vec!(Decoration::Default)
            ),
            format!(
                "635b0b84d686: {black}Waiting{default}",
                black = vec!(Decoration::BlackFgBright),
                default = vec!(Decoration::Default)
            ),
            format!(
                "1b7cb4d6fe05: {yellow}Verifying Checksum{default}",
                yellow = vec!(Decoration::YellowFgBright),
                default = vec!(Decoration::Default)
            ),
            format!(
                "6d24c7242d02: {green}Download complete{default}",
                green = vec!(Decoration::GreenFgBright),
                default = vec!(Decoration::Default)
            ),
            format!(
                "07aded7c29c6: {green_fg}Pull complete{default}",
                green_fg = vec!(Decoration::GreenBgBright),
                default = vec!(Decoration::Default)
            ),
        ];

        fn test_init() -> Arc<dyn Parser + Sync + Send> {
            Arc::new(Docker {
                subcommand: Some("pull".to_owned()),
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
*/
