//! Structures and enums representing colors in the Todoist API (<https://developer.todoist.com/guides/#colors>)
//!
//! These colors are currently being used in [labels](crate::model::label::PersonalLabel)
//! and [projects](crate::model::project::Project)

use fmt::Display;
use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

/// An enum of the colors used in the Todoist API
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Color {
    #[serde(rename = "berry_red")]
    BerryRed,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "olive_green")]
    OliveGreen,
    #[serde(rename = "lime_green")]
    LimeGreen,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "mint_green")]
    MintGreen,
    #[serde(rename = "teal")]
    Teal,
    #[serde(rename = "sky_blue")]
    SkyBlue,
    #[serde(rename = "light_blue")]
    LightBlue,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "grape")]
    Grape,
    #[serde(rename = "violet")]
    Violet,
    #[serde(rename = "lavender")]
    Lavender,
    #[serde(rename = "magenta")]
    Magenta,
    #[serde(rename = "salmon")]
    Salmon,
    #[serde(rename = "charcoal")]
    Charcoal,
    #[serde(rename = "grey")]
    Grey,
    #[serde(rename = "taupe")]
    Taupe,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Color {
    //noinspection SpellCheckingInspection
    /// Get the hexadecimal form of the [Color]
    pub fn get_hex(&self) -> ColorHex {
        match self {
            Color::BerryRed => ColorHex {
                hex: String::from("#B8256F"),
            },
            Color::Red => ColorHex {
                hex: String::from("#DB4035"),
            },
            Color::Orange => ColorHex {
                hex: String::from("#FF9933"),
            },
            Color::Yellow => ColorHex {
                hex: String::from("#FAD000"),
            },
            Color::OliveGreen => ColorHex {
                hex: String::from("#AFB83B"),
            },
            Color::LimeGreen => ColorHex {
                hex: String::from("#7EBC00"),
            },
            Color::Green => ColorHex {
                hex: String::from("#299438"),
            },
            Color::MintGreen => ColorHex {
                hex: String::from("#6ACCBC"),
            },
            Color::Teal => ColorHex {
                hex: String::from("#158FAD"),
            },
            Color::SkyBlue => ColorHex {
                hex: String::from("#14AAF5"),
            },
            Color::LightBlue => ColorHex {
                hex: String::from("#96C3EB"),
            },
            Color::Blue => ColorHex {
                hex: String::from("#4073FF"),
            },
            Color::Grape => ColorHex {
                hex: String::from("#884DFF"),
            },
            Color::Violet => ColorHex {
                hex: String::from("#AF38EB"),
            },
            Color::Lavender => ColorHex {
                hex: String::from("#EB96EB"),
            },
            Color::Magenta => ColorHex {
                hex: String::from("#E05194"),
            },
            Color::Salmon => ColorHex {
                hex: String::from("#FF8D85"),
            },
            Color::Charcoal => ColorHex {
                hex: String::from("#808080"),
            },
            Color::Grey => ColorHex {
                hex: String::from("#B8B8B8"),
            },
            Color::Taupe => ColorHex {
                hex: String::from("#CCAC93"),
            },
        }
    }

    /// Get the Todoist API ID of the [Color]
    pub fn get_id(&self) -> u8 {
        match self {
            Color::BerryRed => 30,
            Color::Red => 31,
            Color::Orange => 32,
            Color::Yellow => 33,
            Color::OliveGreen => 34,
            Color::LimeGreen => 35,
            Color::Green => 36,
            Color::MintGreen => 37,
            Color::Teal => 38,
            Color::SkyBlue => 39,
            Color::LightBlue => 40,
            Color::Blue => 41,
            Color::Grape => 42,
            Color::Violet => 43,
            Color::Lavender => 44,
            Color::Magenta => 45,
            Color::Salmon => 46,
            Color::Charcoal => 47,
            Color::Grey => 48,
            Color::Taupe => 49,
        }
    }
}

/// The hexadecimal value of a color
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColorHex {
    /// The hexadecimal [String] value of a color
    pub hex: String,
}

impl Display for ColorHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hex)
    }
}

impl From<Color> for ColorHex {
    fn from(value: Color) -> Self {
        value.get_hex()
    }
}
