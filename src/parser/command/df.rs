use crate::{
    decorate,
    parser::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
};

pub struct Df;

impl Parser for Df {
    fn regexs(&self) -> Vec<ColorerRegex> {
        vec![
            // file system
            ColorerRegex::new(
                r"^[^A-Z](\w+(\s\w+)?|\/\w+)\S+",
                decorate!(Decoration::YellowFg),
                Some(vec![
                    (r"~\/\S+", decorate!(Decoration::YellowFg)),
                    (
                        r"^(tmp|map\s)\S+", // check \s after map
                        decorate!(Decoration::WhiteFg, Decoration::BlackBg),
                    ),
                    (r"^[a-zA-Z]\S+", decorate!(Decoration::Default)),
                ]),
            ),
            // available
            ColorerRegex::new(
                r"(?<=(\b(\d+((\.|\,)\d+)?(\w+)?\s+){2}))\d+((\.|\,)\d+)?([BKMGT](i)?)?(?=(\s+(\d+%|-)))",
                decorate!(Decoration::BlueFg),
                Some(vec![("0", decorate!(Decoration::RedFg))]),
            ),
            // usage
            ColorerRegex::new(
                r"(?<=(\b(\d+((\.|\,)\d+)?(\w+)?\s+){3}))\d+%",
                decorate!(Decoration::GreenFg),
                Some(vec![
                    (r"([5-7][0-9])%", decorate!(Decoration::YellowFg)),
                    (r"([8][0-9]|[9][0-6])%", decorate!(Decoration::MagentaFg)),
                    (r"([9][7-9]|100)%", decorate!(Decoration::RedBg)),
                ]),
            ),
            // mounted
            ColorerRegex::new(r"(?<=(%|\-)\s+)\/.*", decorate!(Decoration::CyanFg), None),
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
    fn df() {
        let input = vec![
            "File system    1K-blocchi     Usati Disponib. Uso% Montato su",
            "udev              8126756         0   8126756   0% /dev",
            "tmpfs             1629664      9852   1619812   1% /run",
            "/dev/nvme0n1p2  462768976 268470844 170720996  62% /",
            "/dev/loop1         168832    168832         0 100% /snap/gnome-3-28-1804/161",
            "/dev/nvme0n1p1          0         0         0    - /boot/efi",
        ];

        let correct_output = vec![
            format!("File system    1K-blocchi     Usati Disponib. Uso% Montato su"),
            format!("{reset}udev{reset}              8126756         0   {blue}8126756{reset}   {green}0%{reset} {cyan}/dev{reset}",
                    blue = decorate!(Decoration::BlueFg),
                    green = decorate!(Decoration::GreenFg),
                    cyan = decorate!(Decoration::CyanFg),
                    reset = decorate!(Decoration::Default)
            ),
            format!("{white}{black_bg}tmpfs{reset}             1629664      9852   {blue}1619812{reset}   {green}1%{reset} {cyan}/run{reset}",
                    white = decorate!(Decoration::WhiteFg),
                    black_bg = decorate!(Decoration::BlackBg),
                    blue = decorate!(Decoration::BlueFg),
                    green = decorate!(Decoration::GreenFg),
                    cyan = decorate!(Decoration::CyanFg),
                    reset = decorate!(Decoration::Default)
            ),
            format!("{yellow}/dev/nvme0n1p2{reset}  462768976 268470844 {blue}170720996{reset}  {yellow}62%{reset} {cyan}/{reset}",
                    yellow = decorate!(Decoration::YellowFg),
                    blue = decorate!(Decoration::BlueFg),
                    cyan = decorate!(Decoration::CyanFg),
                    reset = decorate!(Decoration::Default)
            ),
            format!("{yellow}/dev/loop1{reset}         168832    168832         {red}0{reset} {red_bg}100%{reset} {cyan}/snap/gnome-3-28-1804/161{reset}",
                    yellow = decorate!(Decoration::YellowFg),
                    red = decorate!(Decoration::RedFg),
                    red_bg = decorate!(Decoration::RedBg),
                    cyan = decorate!(Decoration::CyanFg),
                    reset = decorate!(Decoration::Default)
            ),
            format!("{yellow}/dev/nvme0n1p1{reset}          0         0         {red}0{reset}    - {cyan}/boot/efi{reset}",
                    yellow = decorate!(Decoration::YellowFg),
                    cyan = decorate!(Decoration::CyanFg),
                    red = decorate!(Decoration::RedFg),
                    reset = decorate!(Decoration::Default)
            ),
        ];

        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &init_parser("df").unwrap())
            );
        }
    }

    #[test]
    fn df_h() {
        let input = vec![
            "File system     Dim. Usati Dispon. Uso% Montato su",
            "udev            7,8G     0    7,8G   0% /dev",
            "tmpfs           1,6G  9,7M    1,6G   1% /run",
            "/dev/nvme0n1p2  442G  256G    163G  62% /",
            "tmpfs           7,8G     0    7,8G   0% /sys/fs/cgroup",
            "/dev/loop0       66M   66M       0 100% /snap/gtk-common-themes/1515",
            "/dev/nvme0n1p1     0     0       0    - /boot/efi",
            "map auto_home    0Bi    0Bi    0Bi   100%       0          0  100%   /System/Volumes/Data/home"
        ];

        let correct_output = vec![
            format!("File system     Dim. Usati Dispon. Uso% Montato su"),
            format!("{reset}udev{reset}            7,8G     0    {blue}7,8G{reset}   {green}0%{reset} {cyan}/dev{reset}",
                blue = decorate!(Decoration::BlueFg),
                green = decorate!(Decoration::GreenFg),
                cyan = decorate!(Decoration::CyanFg),
                reset = decorate!(Decoration::Default)
            ),
            format!("{white}{black_bg}tmpfs{reset}           1,6G  9,7M    {blue}1,6G{reset}   {green}1%{reset} {cyan}/run{reset}",
                white = decorate!(Decoration::WhiteFg),
                black_bg = decorate!(Decoration::BlackBg),
                blue = decorate!(Decoration::BlueFg),
                green = decorate!(Decoration::GreenFg),
                cyan = decorate!(Decoration::CyanFg),
                reset = decorate!(Decoration::Default)
            ),
            format!("{yellow}/dev/nvme0n1p2{reset}  442G  256G    {blue}163G{reset}  {yellow}62%{reset} {cyan}/{reset}",
                yellow = decorate!(Decoration::YellowFg),
                blue = decorate!(Decoration::BlueFg),
                cyan = decorate!(Decoration::CyanFg),
                reset = decorate!(Decoration::Default)
            ),
            format!("{white}{black_bg}tmpfs{reset}           7,8G     0    {blue}7,8G{reset}   {green}0%{reset} {cyan}/sys/fs/cgroup{reset}",
                white = decorate!(Decoration::WhiteFg),
                black_bg = decorate!(Decoration::BlackBg),
                blue = decorate!(Decoration::BlueFg),
                green = decorate!(Decoration::GreenFg),
                cyan = decorate!(Decoration::CyanFg),
                reset = decorate!(Decoration::Default)
            ),
            format!("{yellow}/dev/loop0{reset}       66M   66M       {red}0{reset} {red_bg}100%{reset} {cyan}/snap/gtk-common-themes/1515{reset}",
                yellow = decorate!(Decoration::YellowFg),
                cyan = decorate!(Decoration::CyanFg),
                red_bg = decorate!(Decoration::RedBg),
                red = decorate!(Decoration::RedFg),
                reset = decorate!(Decoration::Default)
            ),
            format!("{yellow}/dev/nvme0n1p1{reset}     0     0       {red}0{reset}    - {cyan}/boot/efi{reset}",
                yellow = decorate!(Decoration::YellowFg),
                cyan = decorate!(Decoration::CyanFg),
                red = decorate!(Decoration::RedFg),
                    reset = decorate!(Decoration::Default)
            ),
            format!("{white}{black}map auto_home{reset}    0Bi    0Bi    {blue}0Bi{reset}   {red_bg}100%{reset}       0          0  100%   {cyan}/System/Volumes/Data/home{reset}",
                    white = decorate!(Decoration::WhiteFg),
                    blue = decorate!(Decoration::BlueFg),
                    black = decorate!(Decoration::BlackBg),
                    cyan = decorate!(Decoration::CyanFg),
                    red_bg = decorate!(Decoration::RedBg),
                    reset = decorate!(Decoration::Default)
            )
        ];

        for (index, line) in input.iter().enumerate() {
            assert_eq!(
                correct_output.get(index).unwrap(),
                &reader_handler(line.to_string(), &init_parser("df").unwrap())
            );
        }
    }
}
