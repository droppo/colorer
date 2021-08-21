use crate::{
    decorate,
    parser::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
};

#[derive(Debug)]
pub struct Docker {
    pub subcommand: Option<String>,
}

impl Parser for Docker {
    fn regexs(&self) -> Vec<crate::parser::parser::ColorerRegex> {
        // let args: Vec<OsString> = CliArgs::parse_args().args;
        let regex_vector = if let Some(subcommand) = &self.subcommand {
            match subcommand.as_str() {
                "ps" | "container" => {
                    vec![
                        // image
                        ColorerRegex::new(
                            r#"(?<=(^(\S+)\s+))(\S+){3,}"#,
                            decorate!(Decoration::MagentaFg),
                            None,
                        ),
                        // created
                        ColorerRegex::new(
                            r#"(?<=(^\S+\s+\S+\s+\".*\"\s+))(\d|\w+\s\w+|\d+)(\s\w+){1,3}"#,
                            decorate!(Decoration::BlueFg),
                            None,
                        ),
                        // status
                        ColorerRegex::new(
                            r#"(\w+\s\d+\s\w+|\w+\s\(\d+\)\s\d+(\s\w+){2})"#,
                            decorate!(Decoration::GreenFg),
                            Some(vec![(
                                r#"\w+\s\(\d+\)\s\d+\s\w+\s\w+"#,
                                decorate!(Decoration::RedBg),
                            )]),
                        ),
                        // named status e.g. healthy, unhealthy
                        // FIXME unhealthy not working correctly
                        // ColorerRegex::new(
                        //     r#"\bunhealthy\b"#,
                        //     decorate!(Decoration::ReBg),
                        //     Some(vec![(r"\b(unhealthy)\b", decorate!(Decoration::ReBg))]),
                        // ),
                        // itnernal ports
                        ColorerRegex::new(
                            r#"(?<=(\s|>))\d+/\w+"#,
                            decorate!(Decoration::YellowFg),
                            None,
                        ),
                        // esternal
                        ColorerRegex::new(
                            r#"((\d{1,3}.){3}\d{1,3}|::):\d+"#,
                            decorate!(Decoration::GreenFg),
                            None,
                        ),
                        // name
                        ColorerRegex::new(
                            r#"\S+[^NAMES\n]($|\n)"#,
                            decorate!(Decoration::CyanFg),
                            None,
                        ),
                    ]
                }
                _ => {
                    vec![ColorerRegex::new("", decorate!(Decoration::Default), None)]
                }
            }
        } else {
            vec![ColorerRegex::new("", decorate!(Decoration::Default), None)]
        };

        regex_vector
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        decorate,
        parser::{
            command::docker::Docker,
            decorator::Decoration,
            parser::{reader_handler, Parser},
        },
    };

    #[test]
    fn docker_ps() {
        let input = r#"12c3dedd07be   kunde21/gitea-arm   "/usr/bin/entrypoint…"   5 weeks ago   Up 5 days  3000/tcp, 0.0.0.0:222->22/tcp, :::222->22/tcp   gitea"#.to_string();

        let correct_output = format!(
            r#"12c3dedd07be   {magenta}kunde21/gitea-arm{reset}   "/usr/bin/entrypoint…"   {blue}5 weeks ago{reset}   {green}Up 5 days{reset}  {yellow}3000/tcp{reset}, {green}0.0.0.0:222{reset}->{yellow}22/tcp{reset}, {green}:::222{reset}->{yellow}22/tcp{reset}   {cyan}gitea{reset}"#,
            magenta = decorate!(Decoration::MagentaFg),
            blue = decorate!(Decoration::BlueFg),
            green = decorate!(Decoration::GreenFg),
            yellow = decorate!(Decoration::YellowFg),
            cyan = decorate!(Decoration::CyanFg),
            reset = decorate!(Decoration::Default)
        );

        fn test_init() -> Arc<dyn Parser + Sync + Send> {
            Arc::new(Docker {
                subcommand: Some("ps".to_owned()),
            })
        }

        assert_eq!(correct_output, reader_handler(input, &test_init()));
    }

    // #[test]
    // fn docker_ps_healthy() {
    //     let input = r#"5fb8032dd734   vaultwarden/server:latest   "/usr/bin/dumb-init …"   5 weeks ago   Up 5 days (healthy)   80/tcp, 3012/tcp    vaultwarden"#.to_string();

    //     let correct_output = format!(
    //         r#"5fb8032dd734   {magenta}vaultwarden/server:latest{reset}   "/usr/bin/dumb-init …"   {blue}5 weeks ago{reset}   {green}Up 5 days{reset} ({green_bg}healthy{reset})   {yellow}80/tcp{reset}, {yellow}3012/tcp{reset}    {cyan}vaultwarden{reset}"#,
    //         magenta = decorate!(Decoration::MAGENTA_FG,
    //         blue = decorate!(Decoration::BlueFg),
    //         green = decorate!(Decoration::GreenFg),
    //         green_bg = decorate!(Decoration::GREEN_BG,
    //         yellow = decorate!(Decoration::YellowFg),
    //         cyan = decorate!(Decoration::CyanFg),
    //         reset = decorate!(Decoration::RESET
    //     );

    //     fn test_init() -> Box<dyn Parser> {
    //         Box::new(Docker {
    //             subcommand: Some("ps".to_owned()),
    //         })
    //     }

    //     assert_eq!(correct_output, reader_handler(input, &test_init()));
    // }

    // #[test]
    // fn docker_ps_unhealthy() {
    //     let input = r#"5fb8032dd734   vaultwarden/server:latest   "/usr/bin/dumb-init …"   5 weeks ago   Up 5 days (unhealthy)   80/tcp, 3012/tcp    vaultwarden"#.to_string();

    //     let correct_output = format!(
    //         r#"5fb8032dd734   {magenta}vaultwarden/server:latest{reset}   "/usr/bin/dumb-init …"   {blue}5 weeks ago{reset}   {green}Up 5 days{reset} ({ReBg)}unhealthy{reset})   {yellow}80/tcp{reset}, {yellow}3012/tcp{reset}    {cyan}vaultwarden{reset}"#,
    //         magenta = decorate!(Decoration::MAGENTA_FG,
    //         blue = decorate!(Decoration::BlueFg),
    //         green = decorate!(Decoration::GreenFg),
    //         ReBg) = decorate!(Decoration::ReBg),
    //         yellow = decorate!(Decoration::YellowFg),
    //         cyan = decorate!(Decoration::CyanFg),
    //         reset = decorate!(Decoration::RESET
    //     );

    //     fn test_init() -> Box<dyn Parser> {
    //         Box::new(Docker {
    //             subcommand: Some("ps".to_owned()),
    //         })
    //     }

    //     assert_eq!(correct_output, reader_handler(input, &test_init()));
    // }
}
