use std::fmt;

use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};

use colored::{ColoredString, Colorize};

///
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,

    /// The color assigned to wild cards before the color is chosen.
    Undecided,
}

impl Color {
    /// Returns an array containing all colors except Undecided.
    pub fn all() -> [Self; 4] {
        [Color::Red, Color::Blue, Color::Green, Color::Yellow]
    }

    /// Returns a random color excluding Undecided.
    pub fn random() -> Self {
        let colors = Self::all();
        let i = SmallRng::from_rng(thread_rng())
            .unwrap()
            .gen_range(0..colors.len());
        colors[i]
    }

    pub fn to_num(&self) -> u32 {
        match self {
            Self::Red => 0,
            Self::Yellow => 1,
            Self::Green => 2,
            Self::Blue => 3,
            Self::Undecided => 4
        }
    }

    pub fn from_num(n: u32) -> Self {
        match n {
            0 => Self::Red,
            1 => Self::Yellow,
            2 => Self::Green,
            3 => Self::Blue,
            4 => Self::Undecided,
            _ => panic!("can not convert num {} to Color", n)
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Red => write!(f, "{}", "Red".red()),
            Self::Blue => write!(f, "{}", "Blue".blue()),
            Self::Green => write!(f, "{}", "Green".green()),
            Self::Yellow => write!(f, "{}", "Yellow".yellow()),
            Self::Undecided => write!(f, "{}", "UNDECIDED".strikethrough()),
        }
    }
}
