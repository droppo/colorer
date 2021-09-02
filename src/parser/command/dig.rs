use crate::{
    decorate,
    parser::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
};

pub struct Dig;

impl Parser for Dig {
    fn regexs(&self) -> Vec<crate::parser::parser::ColorerRegex> {
        vec![
            // comment
            ColorerRegex::new(r"^;.*", decorate!(Decoration::BlackFgBright), None),
            // ip
            ColorerRegex::new(
                r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$",
                decorate!(Decoration::GreenFgBright),
                None,
            ),
            // answer
            ColorerRegex::new(
                r"^[^;](\S+\.){1,}\S+",
                decorate!(Decoration::BlueFgBright),
                None,
            ),
            // cname
            ColorerRegex::new(
                r"(?<=CNAME\s+).*(?=\.)",
                decorate!(Decoration::MagentaFgBright),
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
    fn dig() {
        let input = vec![
            "; <<>> DiG 9.11.5-P4-5.1+deb10u5-Debian <<>> duckduckgo.com",
            ";; global options: +cmd",
            ";; Got answer:",
            ";; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 11262",
            ";; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 1",
            ";; OPT PSEUDOSECTION:",
            "; EDNS: version: 0, flags:; udp: 512",
            ";; QUESTION SECTION:",
            ";duckduckgo.com.			IN	A",
            ";; ANSWER SECTION:",
            "duckduckgo.com.		200	IN	A	40.114.177.156",
            ";; Query time: 86 msec",
            ";; SERVER: 192.168.0.90#53(192.168.0.90)",
            ";; WHEN: mer ago 18 16:30:01 CEST 2021",
            ";; MSG SIZE  rcvd: 59",
        ];

        let correct_output = vec![
            format!("{black}; <<>> DiG 9.11.5-P4-5.1+deb10u5-Debian <<>> duckduckgo.com{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; global options: +cmd{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; Got answer:{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 11262{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 1{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; OPT PSEUDOSECTION:{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black}; EDNS: version: 0, flags:; udp: 512{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; QUESTION SECTION:{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};duckduckgo.com.			IN	A{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; ANSWER SECTION:{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{blue}duckduckgo.com.{default}		200	IN	A	{green}40.114.177.156{default}",
                    green = decorate!(Decoration::GreenFgBright),
                    blue = decorate!(Decoration::BlueFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; Query time: 86 msec{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; SERVER: 192.168.0.90#53(192.168.0.90){default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; WHEN: mer ago 18 16:30:01 CEST 2021{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
            format!("{black};; MSG SIZE  rcvd: 59{default}",
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default)),
        ];

        let args: Vec<String> = vec![];
        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &init_parser("dig", &args).unwrap())
            );
        }
    }
}
