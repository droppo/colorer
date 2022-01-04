use crate::core::{
    decorator::Decoration,
    parser::{ColorerRegex, Command},
};

pub struct Ls;

impl Ls {
    pub fn regexs() -> Command {
        // TODO find a better regex for dates... this seems pretty expensive
        let v = vec![
            // date
            // ColorerRegex::new(r"(\w{3}|\d{1,2})\s+(\w{3}|\d{1,2})\s+(\d{4}|\d{1,2}(\:|\.)\d{2})", decorate!(Decoration::Underlined), None),
            // root user
            ColorerRegex::new(r"(?<=\s)root(?=\s)", vec![Decoration::RedFgBright], None),
            // size
            ColorerRegex::new(
                r"\S+(?=(\s+(\w{3}|\d{1,2})\s+(\w{3}|\d{1,2})\s+(\d{4}|\d{1,2}[:.]\d{2})))",
                vec![Decoration::Default], // TODO None insead of Default
                Some(vec![
                    (
                        r"(\S+[KB])|\S{1}([.,]\d+)?M|(\d{1,7})".to_string(),
                        vec![Decoration::GreenFgBright],
                    ),
                    (
                        r"(\S+G)|(\d{10,})".to_string(),
                        vec![Decoration::RedFgBright, Decoration::Bold],
                    ),
                    (
                        r"(\d{2,}([.,]\d+)?M)|(\d{8,})".to_string(),
                        vec![Decoration::YellowFgBright, Decoration::Bold],
                    ),
                ]),
            ),
            // directory
            ColorerRegex::new(
                r"[bcdlnps](?=([-rwxst]{3}){3})",
                vec![Decoration::Bold],
                None,
            ),
            // user permission
            ColorerRegex::new(
                r"(?<=(^[-bcdlnps]{1}))[-rwxst]{3}",
                vec![Decoration::YellowFgBright],
                None,
            ),
            // group permission
            ColorerRegex::new(
                r"(?<=(^[-bcdlnps]{1}\S{3}))[-rwxst]{3}",
                vec![Decoration::RedFgBright],
                None,
            ),
            // other permission
            ColorerRegex::new(
                r"(?<=(^[-bcdlnps]{1}\S{6}))[-rwxst]{3}",
                vec![Decoration::GreenFgBright],
                None,
            ),
            // hidden files and directories
            ColorerRegex::new(
                r"((?<=\s)\.\w+\S+|\S+\/)$",
                vec![Decoration::BlueFgBright],
                Some(vec![
                    (
                        r"\S+\/".to_string(),
                        vec![Decoration::BlueFgBright, Decoration::Underlined],
                    ),
                    (
                        r"(?<=\s)\.\w+\S+".to_string(),
                        vec![Decoration::BlueFgBright],
                    ),
                ]),
            ), // directories /(?<=(^[d].*\w{3}\s\d{1,2}\s\S+\s)).*/g
            // executables
            ColorerRegex::new(
                r"(\S+\s+->\s+)?\S+\*",
                vec![Decoration::GreenFgBright, Decoration::Bold],
                None,
            ),
        ];

        Command::new(None, None, Some(v))
    }
}
/*
#[cfg(test)]
mod tests {
    use crate::core::parser::{init_parser, reader_handler};

    use super::*;

    #[test]
    fn ll() {
        let input = vec![
            "totale 44K",
            "drwxr-xr-x  5 droppo droppo 4,0K ago 15 23:46 ./",
            "drwxr-xr-x 60 root  root  4,0K ago 13 22:20 ../",
            "-rw-r--r--  1 droppo droppo  11K ago 13 15:12 Cargo.lock",
            "-rw-r--r--  1 droppo droppo  497 ago 13 15:12 Cargo.toml",
            "drwxr-xr-x  8 droppo droppo 4,0K ago 15 23:53 .git/",
            "-rw-r--r--  1 droppo droppo    8 ago 13 15:12 .gitignore",
            "-rw-r--r--  1 droppo droppo  997 ago 14 14:49 README.md",
            "drwxr-xr-x  3 droppo droppo 4,0K ago 13 15:12 src/",
            "drwxr-xr-x  3 droppo droppo 4,0K ago 15 23:46 target/",
            "drwxr-xr-x  7 droppo droppo 4,0K gen 21  2021 .themes/",
            "-rw-r--r--  1 droppo droppo 2,4G lug  9 12:32 pop-os_21.04_amd64_intel_6.iso",
            "-rw-r--r-- 1 droppo droppo 2537684992 lug  9 12:32 pop-os_21.04_amd64_intel_6.iso",
            "-rw-r--r--  1 droppo droppo  28M dic  2  2020 backup.zip",
            "-rw-r--r-- 1 droppo droppo 29252908 dic  2  2020 backup.zip",
            "lrwxrwxrwx   1 root root         20 ago  1  2019 vi -> /etc/alternatives/vi*",
        ];

        let correct_output = vec![
            format!("totale 44K"),
            format!(
                "{bold}d{default}{yellow}rwx{default}{red}r-x{default}{green}r-x{default}  5 droppo droppo {green}4,0K{default} ago 15 23:46 {blue}{underlined}./{default}",
                green = decorate!(Decoration::GreenFgBright),
                bold = decorate!(Decoration::Bold),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                blue = decorate!(Decoration::BlueFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{bold}d{default}{yellow}rwx{default}{red}r-x{default}{green}r-x{default} 60 {red}root{default}  {red}root{default}  {green}4,0K{default} ago 13 22:20 {blue}{underlined}../{default}",
                red = decorate!(Decoration::RedFgBright),
                green = decorate!(Decoration::GreenFgBright),
                bold = decorate!(Decoration::Bold),
                yellow = decorate!(Decoration::YellowFgBright),
                blue = decorate!(Decoration::BlueFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "-{yellow}rw-{default}{red}r--{default}{green}r--{default}  1 droppo droppo  {green}11K{default} ago 13 15:12 Cargo.lock",
                green = decorate!(Decoration::GreenFgBright),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "-{yellow}rw-{default}{red}r--{default}{green}r--{default}  1 droppo droppo  {green}497{default} ago 13 15:12 Cargo.toml",
                green = decorate!(Decoration::GreenFgBright),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{bold}d{default}{yellow}rwx{default}{red}r-x{default}{green}r-x{default}  8 droppo droppo {green}4,0K{default} ago 15 23:53 {blue}{underlined}.git/{default}",
                green = decorate!(Decoration::GreenFgBright),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                bold = decorate!(Decoration::Bold),
                blue = decorate!(Decoration::BlueFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "-{yellow}rw-{default}{red}r--{default}{green}r--{default}  1 droppo droppo    {green}8{default} ago 13 15:12 {blue}.gitignore{default}",
                green = decorate!(Decoration::GreenFgBright),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                blue = decorate!(Decoration::BlueFgBright),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "-{yellow}rw-{default}{red}r--{default}{green}r--{default}  1 droppo droppo  {green}997{default} ago 14 14:49 README.md",
                green = decorate!(Decoration::GreenFgBright),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{bold}d{default}{yellow}rwx{default}{red}r-x{default}{green}r-x{default}  3 droppo droppo {green}4,0K{default} ago 13 15:12 {blue}{underlined}src/{default}",
                green = decorate!(Decoration::GreenFgBright),
                bold = decorate!(Decoration::Bold),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                blue = decorate!(Decoration::BlueFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{bold}d{default}{yellow}rwx{default}{red}r-x{default}{green}r-x{default}  3 droppo droppo {green}4,0K{default} ago 15 23:46 {blue}{underlined}target/{default}",
                green = decorate!(Decoration::GreenFgBright),
                bold = decorate!(Decoration::Bold),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                blue = decorate!(Decoration::BlueFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{bold}d{default}{yellow}rwx{default}{red}r-x{default}{green}r-x{default}  7 droppo droppo {green}4,0K{default} gen 21  2021 {blue}{underlined}.themes/{default}",
                green = decorate!(Decoration::GreenFgBright),
                bold = decorate!(Decoration::Bold),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                blue = decorate!(Decoration::BlueFgBright),
                underlined = decorate!(Decoration::Underlined),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "-{yellow}rw-{default}{red}r--{default}{green}r--{default}  1 droppo droppo {red}{bold}2,4G{default} lug  9 12:32 pop-os_21.04_amd64_intel_6.iso",
                green = decorate!(Decoration::GreenFgBright),
                bold = decorate!(Decoration::Bold),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "-{yellow}rw-{default}{red}r--{default}{green}r--{default} 1 droppo droppo {red}{bold}2537684992{default} lug  9 12:32 pop-os_21.04_amd64_intel_6.iso",
                green = decorate!(Decoration::GreenFgBright),
                bold = decorate!(Decoration::Bold),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                default = decorate!(Decoration::Default)

            ),
            format!(
                "-{yellow}rw-{default}{red}r--{default}{green}r--{default}  1 droppo droppo  {yellow}{bold}28M{default} dic  2  2020 backup.zip",
                green = decorate!(Decoration::GreenFgBright),
                bold = decorate!(Decoration::Bold),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "-{yellow}rw-{default}{red}r--{default}{green}r--{default} 1 droppo droppo {yellow}{bold}29252908{default} dic  2  2020 backup.zip",
                green = decorate!(Decoration::GreenFgBright),
                bold = decorate!(Decoration::Bold),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                default = decorate!(Decoration::Default)
            ),
            format!(
                "{bold}l{default}{yellow}rwx{default}{red}rwx{default}{green}rwx{default}   1 {red}root{default} {red}root{default}         {green}20{default} ago  1  2019 {green}{bold}vi -> /etc/alternatives/vi*{default}",
                green = decorate!(Decoration::GreenFgBright),
                bold = decorate!(Decoration::Bold),
                yellow = decorate!(Decoration::YellowFgBright),
                red = decorate!(Decoration::RedFgBright),
                default = decorate!(Decoration::Default)
            )
        ];

        let args = vec!["clrr".to_string(), "ls".to_string(), "-l".to_string()];
        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &init_parser("ls", &args).unwrap())
            );
        }
    }
}
*/
