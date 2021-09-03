use crate::{
    core::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
    decorate,
};

pub struct Lsns;

impl Parser for Lsns {
    fn regexs(&self) -> Vec<ColorerRegex> {
        vec![
            // header
            ColorerRegex::new(
                "NS|TYPE|NPROCS|PID|USER|COMMAND",
                decorate!(Decoration::Underlined),
                None,
            ),
            // ns
            ColorerRegex::new(r"^\w+", decorate!(Decoration::YellowFgBright), None),
            // root
            ColorerRegex::new(r"root", decorate!(Decoration::RedBgBright), None),
            // command
            ColorerRegex::new(
                r"(?<=^(\w+\s+){5}).*",
                decorate!(Decoration::CyanFgBright),
                None,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{
            decorator::Decoration,
            parser::{init_parser, reader_handler},
        },
        decorate,
    };

    #[test]
    fn lsns() {
        let input = vec![
            "        NS TYPE   NPROCS   PID USER  COMMAND",
            "4026531835 cgroup     66  1805 droppo /lib/systemd/systemd --user",
            "4026531835 cgroup     66  1805 root /lib/systemd/systemd --user",
        ];

        let correct_output = vec![
            format!(
                "        {underline}NS{default} {underline}TYPE{default}   {underline}NPROCS{default}   {underline}PID{default} {underline}USER{default}  {underline}COMMAND{default}",
                underline = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)),
            format!(
                "{yellow}4026531835{default} cgroup     66  1805 droppo {cyan}/lib/systemd/systemd --user{default}",
                yellow = decorate!(Decoration::YellowFgBright),
                cyan = decorate!(Decoration::CyanFgBright), default = decorate!(Decoration::Default)),
            format!(
                "{yellow}4026531835{default} cgroup     66  1805 {red}root{default} {cyan}/lib/systemd/systemd --user{default}",
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedBgBright),
                cyan = decorate!(Decoration::CyanFgBright), default = decorate!(Decoration::Default))
        ];

        let args: Vec<String> = vec![];
        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &init_parser("lsns", &args).unwrap())
            )
        }
    }
}
