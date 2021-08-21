use crate::{
    decorate,
    parser::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
};

pub struct Nslookup;

impl Parser for Nslookup {
    fn regexs(&self) -> Vec<crate::parser::parser::ColorerRegex> {
        vec![
            // values
            ColorerRegex::new(
                r"Server|Address|Non-authoritative\s+answer|Name|canonical\s+name",
                decorate!(Decoration::Underlined),
                None,
            ),
            // ip
            ColorerRegex::new(
                r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}|([a-z0-9]+\:(\:\S+)?){2,}",
                decorate!(Decoration::GreenFgBright),
                None,
            ),
            // port
            ColorerRegex::new(r"(?<=#)\d+", decorate!(Decoration::YellowFgBright), None),
            // canonical name
            ColorerRegex::new(
                r"(?<=(canonical\s+name\s+=\s+)).*(?=\.)",
                decorate!(Decoration::MagentaFgBright),
                None,
            ),
            // name
            ColorerRegex::new(
                r"(?<=(Name:\s+))\S+",
                decorate!(Decoration::BlueFgBright),
                None,
            ),
            // non auth answers uri
            ColorerRegex::new(r"^(\S+\.){1,}\S+", decorate!(Decoration::RedFgBright), None),
            // not found
            ColorerRegex::new(
                r"\*\*(\s+\S+){1,}",
                decorate!(Decoration::WhiteFgBright, Decoration::RedBgBright),
                None,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        decorate,
        parser::{
            decorator::Decoration,
            parser::{init_parser, reader_handler},
        },
    };

    #[test]
    fn nslookup() {
        let input = vec![
            "Server:		192.168.0.90",
            "Address:	192.168.0.90#53",
            "Non-authoritative answer:",
            "www.duckduckgo.com	canonical name = duckduckgo.com.",
            "Name:	duckduckgo.com",
            "Address: 40.114.177.156",
            "Name:	www.google.it",
            "Address: 2a00:1450:4002:800::2003",
            "tp.47cf2c8c9-frontier.amazon.com	canonical name = d3ag4hukkh62yn.cloudfront.net.",
            "** server can't find aaa.bbb: NXDOMAIN",
        ];

        let correct_output = vec![
            format!(
                "{underlined}Server{default}:		{green}192.168.0.90{default}",
                green = decorate!(Decoration::GreenFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{underlined}Address{default}:	{green}192.168.0.90{default}#{yellow}53{default}",
                green = decorate!(Decoration::GreenFgBright),
                yellow = decorate!(Decoration::YellowFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{underlined}Non-authoritative answer{default}:",
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{red}www.duckduckgo.com{default}	{underlined}canonical name{default} = {magenta}duckduckgo.com{default}.",
                red = decorate!(Decoration::RedFgBright),
                magenta = decorate!(Decoration::MagentaFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{underlined}Name{default}:	{blue}duckduckgo.com{default}",
                blue = decorate!(Decoration::BlueFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{underlined}Address{default}: {green}40.114.177.156{default}",
                green = decorate!(Decoration::GreenFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{underlined}Name{default}:	{blue}www.google.it{default}",
                blue = decorate!(Decoration::BlueFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{underlined}Address{default}: {green}2a00:1450:4002:800::2003{default}",
                green = decorate!(Decoration::GreenFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{red}tp.47cf2c8c9-frontier.amazon.com{default}	{underlined}canonical name{default} = {magenta}d3ag4hukkh62yn.cloudfront.net{default}.",
                red = decorate!(Decoration::RedFgBright),
                magenta = decorate!(Decoration::MagentaFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{white}{red_bg}** server can't find aaa.bbb: NXDOMAIN{default}",
                white = decorate!(Decoration::WhiteFgBright),
                red_bg = decorate!(Decoration::RedBgBright),
                default = decorate!(Decoration::Default)
            )
        ];

        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &init_parser("nslookup").unwrap())
            );
        }
    }
}
