use crate::{
    decorate,
    parser::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
};

pub struct Env;

impl Parser for Env {
    fn regexs(&self) -> Vec<ColorerRegex> {
        vec![
            ColorerRegex::new(r"^\w+(?=\=)", decorate!(Decoration::YellowFgBright), None),
            ColorerRegex::new(r"(?<=\=).*", decorate!(Decoration::CyanFgBright), None),
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
    fn env() {
        let input = vec![
            "DISPLAY=:0",
            "XDG_VTNR=2",
            "LOGNAME=droppo",
            "LS_COLORS=rs96mrs=0:di=01;34:ln=01;36:",
        ];

        let correct_output = vec![
            format!(
                "{yellow}DISPLAY{default}={cyan}:0{default}",
                yellow = decorate!(Decoration::YellowFgBright),
                cyan = decorate!(Decoration::CyanFgBright),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{yellow}XDG_VTNR{default}={cyan}2{default}",
                yellow = decorate!(Decoration::YellowFgBright),
                cyan = decorate!(Decoration::CyanFgBright),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{yellow}LOGNAME{default}={cyan}droppo{default}",
                yellow = decorate!(Decoration::YellowFgBright),
                cyan = decorate!(Decoration::CyanFgBright),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{yellow}LS_COLORS{default}={cyan}rs96mrs=0:di=01;34:ln=01;36:{default}",
                yellow = decorate!(Decoration::YellowFgBright),
                cyan = decorate!(Decoration::CyanFgBright),
                default = decorate!(Decoration::Default)
            ),
        ];

        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &init_parser("env").unwrap())
            );
        }
    }
}
