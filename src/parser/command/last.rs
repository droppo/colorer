use crate::{
    decorate,
    parser::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
};

pub struct Last;

impl Parser for Last {
    fn regexs(&self) -> Vec<ColorerRegex> {
        vec![
            // ip
            ColorerRegex::new(
                r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}",
                decorate!(Decoration::GreenFgBright),
                None,
            ),
            // root
            ColorerRegex::new(r"^root", decorate!(Decoration::RedFgBright), None),
            // reboot
            ColorerRegex::new(r"^reboot", decorate!(Decoration::YellowFgBright), None),
            // duration
            ColorerRegex::new(
                r"(?<=(\()).*(?=\))",
                decorate!(Decoration::YellowFgBright),
                Some(vec![
                    (r"00:0[0-9]", decorate!(Decoration::GreenFgBright)),
                    (
                        r"00:[1-5][0-9]",
                        decorate!(
                            Decoration::BlackFgBright,
                            Decoration::Bold,
                            Decoration::YellowFgBright
                        ),
                    ),
                    (
                        r"\d+:\d+",
                        decorate!(Decoration::Bold, Decoration::RedFgBright),
                    ),
                ]),
            ),
            //
            ColorerRegex::new(
                r"still\slogged\sin|still\srunning|down|crash",
                decorate!(Decoration::CyanFgBright),
                Some(vec![(r"crash", decorate!(Decoration::RedBgBright))]),
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
    fn last() {
        let input = vec![
            "ubuntu   pts/0        192.168.0.117    Thu Aug 19 12:16   still logged in",
            "ubuntu   pts/0        192.168.0.117    Wed Aug 18 10:50 - 10:56  (00:05)",
            "reboot   system boot  5.4.0-1041-raspi Thu Jan  1 00:00   still running",
            "reboot   system boot  5.4.0-1028-raspi Thu Jan  1 00:00 - 18:27 (18792+18:27)",
            "root     pts/0        192.168.0.117    Tue Aug 17 17:38 - 17:39  (00:00)",
            "ubuntu   :0           :0               Thu Sep 24 16:35 - down   (02:07)",
            "ubuntu   :0           :0               Fri Jan 29 15:41 - crash  (00:43)",
        ];

        let correct_output = vec![
            format!("ubuntu   pts/0        {green}192.168.0.117{default}    Thu Aug 19 12:16   {cyan}still logged in{default}",
                    green = decorate!(Decoration::GreenFgBright),
                    cyan = decorate!(Decoration::CyanFgBright),
                    default = decorate!(Decoration::Default),
            ),
            format!("ubuntu   pts/0        {green}192.168.0.117{default}    Wed Aug 18 10:50 - 10:56  ({green}00:05{default})",
                    green = decorate!(Decoration::GreenFgBright),
                    default = decorate!(Decoration::Default),
            ),
            format!("{yellow}reboot{default}   system boot  5.4.0-1041-raspi Thu Jan  1 00:00   {cyan}still running{default}",
                    yellow = decorate!(Decoration::YellowFgBright),
                    cyan = decorate!(Decoration::CyanFgBright),
                    default = decorate!(Decoration::Default),
            ),
            format!(
                "{yellow}reboot{default}   system boot  5.4.0-1028-raspi Thu Jan  1 00:00 - 18:27 ({yellow}18792+18:27{default})",
                yellow= decorate!(Decoration::YellowFgBright),
                default = decorate!(Decoration::Default),
            ),
            format!("{red}root{default}     pts/0        {green}192.168.0.117{default}    Tue Aug 17 17:38 - 17:39  ({green}00:00{default})",
                    green = decorate!(Decoration::GreenFgBright),
                    red= decorate!(Decoration::RedFgBright),
                    default = decorate!(Decoration::Default),
            ),
            format!("ubuntu   :0           :0               Thu Sep 24 16:35 - {cyan}down{default}   ({bold}{red}02:07{default})",
                    cyan = decorate!(Decoration::CyanFgBright),
                    red = decorate!(Decoration::RedFgBright),
                    bold = decorate!(Decoration::Bold),
                    default = decorate!(Decoration::Default),
            ),
            format!("ubuntu   :0           :0               Fri Jan 29 15:41 - {red}crash{default}  ({black}{bold}{yellow}00:43{default})",
                    red = decorate!(Decoration::RedBgBright),
                    yellow = decorate!(Decoration::YellowFgBright),
                    bold = decorate!(Decoration::Bold),
                    black = decorate!(Decoration::BlackFgBright),
                    default = decorate!(Decoration::Default),
            ),
        ];

        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &init_parser("last").unwrap())
            );
        }
    }
}
