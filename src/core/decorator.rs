// TODO add type checking
#[macro_export]
macro_rules! decorate {
    ($($decoration:expr),*) => {
        {
            let mut temp_string = String::new();
            $(
                temp_string.push_str(format!("\u{1b}[{}m", $decoration as i32).as_str());
            )*
            temp_string
        }
    };
}

pub enum Decoration {
    Default = 0,
    Bold = 1,
    Faint = 2,
    Italicized = 3,
    Underlined = 4,
    Blink = 5,
    BlackFg = 30,
    RedFg = 31,
    GreenFg = 32,
    YellowFg = 33,
    BlueFg = 34,
    MagentaFg = 35,
    CyanFg = 36,
    WhiteFg = 37,
    BlackBg = 40,
    RedBg = 41,
    GreenBg = 42,
    YellowBg = 43,
    BlueBg = 44,
    MagentaBg = 45,
    CyanBg = 46,
    WhiteBg = 47,
    BlackFgBright = 90,
    RedFgBright = 91,
    GreenFgBright = 92,
    YellowFgBright = 93,
    BlueFgBright = 94,
    MagentaFgBright = 95,
    CyanFgBright = 96,
    WhiteFgBright = 97,
    BlackBgBright = 100,
    RedBgBright = 101,
    GreenBgBright = 102,
    YellowBgBright = 103,
    BlueBgBright = 104,
    MagentaBgBright = 105,
    CyanBgBright = 106,
    WhiteBgBright = 107,
}
