use crate::{
    core::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
    decorate,
};

pub struct Free;

impl Parser for Free {
    fn regexs(&self) -> Vec<crate::core::parser::ColorerRegex> {
        vec![
            // Mem
            ColorerRegex::new(r"\bMem\b", decorate!(Decoration::BlueFg), None),
            // Swap
            ColorerRegex::new(r"\bSwap\b", decorate!(Decoration::MagentaFg), None),
            // total
            ColorerRegex::new(
                r"(?<=(\b(Mem|Swap)\b\S\s+))\S+",
                decorate!(Decoration::GreenFg),
                None,
            ),
            // used
            ColorerRegex::new(
                r"(?<=(\b(Mem|Swap)\b(\S+\s+){2}))\S+",
                decorate!(Decoration::GreenFg),
                Some(vec![(r"\b0(\w)?", decorate!(Decoration::GreenBg))]),
            ),
            // free
            ColorerRegex::new(
                r"(?<=(\b(Mem|Swap)\b(\S+\s+){3}))\S+",
                decorate!(Decoration::GreenFg),
                Some(vec![(r"\b0(\w)?", decorate!(Decoration::RedBg))]),
            ),
            // shared
            ColorerRegex::new(
                r"(?<=(\b(Mem|Swap)\b(\S+\s+){4}))\S+",
                decorate!(Decoration::GreenFg),
                None,
            ),
            // buff/chache
            ColorerRegex::new(
                r"(?<=(\b(Mem|Swap)\b(\S+\s+){5}))\S+",
                decorate!(Decoration::GreenFg),
                None,
            ),
            // when -w is used
            ColorerRegex::new(
                r"(?<=(\b(Mem|Swap)\b(\S+\s+){6}))\S+(?=\s+\w+$)",
                decorate!(Decoration::GreenFg),
                None,
            ),
            // available
            ColorerRegex::new(
                r"(?<=(\b(Mem|Swap)\b(\S+\s+){6,7}))\S+$",
                decorate!(Decoration::CyanFg),
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
    fn free() {
        let input =
            "              total        used        free      shared  buff/cache   available
Mem:           15Gi       1,5Gi       9,4Gi       216Mi       4,6Gi        13Gi
Swap:          15Gi          0B        15Gi
"
            .to_string();
        let correct_output = format!("              total        used        free      shared  buff/cache   available
{blue}Mem{reset}:           {green}15Gi{reset}       {green}1,5Gi{reset}       {green}9,4Gi{reset}       {green}216Mi{reset}       {green}4,6Gi{reset}        {cyan}13Gi{reset}
{magenta}Swap{reset}:          {green}15Gi{reset}          {green_bg}0B{reset}        {green}15Gi{reset}
",
                                     blue = decorate!(Decoration::BlueFg),
                                     green = decorate!(Decoration::GreenFg),
                                     green_bg = decorate!(Decoration::GreenBg),
                                     cyan = decorate!(Decoration::CyanFg),
                                     magenta = decorate!(Decoration::MagentaFg),
                                     reset = decorate!(Decoration::Default)
        );

        let args: Vec<String> = vec![];
        assert_eq!(
            correct_output,
            reader_handler(input, &init_parser("free", &args).unwrap())
        );
    }

    #[test]
    fn free_mw() {
        let input =
            "              total        used        free      shared  buff/cache   available
Mem:       16296660     4051436     7117784      436768     5127440    11474608
Swap:      16645116    16645116           0
"
            .to_string();
        let correct_output = format!("              total        used        free      shared  buff/cache   available
{blue}Mem{reset}:       {green}16296660{reset}     {green}4051436{reset}     {green}7117784{reset}      {green}436768{reset}     {green}5127440{reset}    {cyan}11474608{reset}
{magenta}Swap{reset}:      {green}16645116{reset}    {green}16645116{reset}           {red_bg}0{reset}
",
                                     blue = decorate!(Decoration::BlueFg),
                                     green = decorate!(Decoration::GreenFg),
                                     red_bg = decorate!(Decoration::RedBg),
                                     cyan = decorate!(Decoration::CyanFg),
                                     magenta = decorate!(Decoration::MagentaFg),
                                     reset = decorate!(Decoration::Default),
        );

        let args: Vec<String> = vec![];
        assert_eq!(
            correct_output,
            reader_handler(input, &init_parser("free", &args).unwrap())
        );
    }
}
