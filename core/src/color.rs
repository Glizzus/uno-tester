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
