use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

// TODO add type checking
// #[macro_export]
// macro_rules! decorate {
//     ($($decoration:expr),*) => {
//         {
//             let mut temp_string = String::new();
//             $(
//                 temp_string.push_str(format!("{}", $decoration as i32).as_str());
//             )*
//             temp_string
//         }
//     };
// }

pub fn decorate(decorations: &Vec<Decoration>) -> String {
    let mut decoration = String::new();
    decorations
        .iter()
        .for_each(|d| decoration.push_str(format!("{}", d).as_str()));

    decoration
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl Display for Decoration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Decoration::Default => 0,
            Decoration::Bold => 1,
            Decoration::Faint => 2,
            Decoration::Italicized => 3,
            Decoration::Underlined => 4,
            Decoration::Blink => 5,
            Decoration::BlackFg => 30,
            Decoration::RedFg => 31,
            Decoration::GreenFg => 32,
            Decoration::YellowFg => 33,
            Decoration::BlueFg => 34,
            Decoration::MagentaFg => 35,
            Decoration::CyanFg => 36,
            Decoration::WhiteFg => 37,
            Decoration::BlackBg => 40,
            Decoration::RedBg => 41,
            Decoration::GreenBg => 42,
            Decoration::YellowBg => 43,
            Decoration::BlueBg => 44,
            Decoration::MagentaBg => 45,
            Decoration::CyanBg => 46,
            Decoration::WhiteBg => 47,
            Decoration::BlackFgBright => 90,
            Decoration::RedFgBright => 91,
            Decoration::GreenFgBright => 92,
            Decoration::YellowFgBright => 93,
            Decoration::BlueFgBright => 94,
            Decoration::MagentaFgBright => 95,
            Decoration::CyanFgBright => 96,
            Decoration::WhiteFgBright => 97,
            Decoration::BlackBgBright => 100,
            Decoration::RedBgBright => 101,
            Decoration::GreenBgBright => 102,
            Decoration::YellowBgBright => 103,
            Decoration::BlueBgBright => 104,
            Decoration::MagentaBgBright => 105,
            Decoration::CyanBgBright => 106,
            Decoration::WhiteBgBright => 107,
        };
        write!(f, "\u{1b}[{}m", value)
    }
}
