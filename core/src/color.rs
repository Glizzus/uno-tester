use std::fmt;

use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};

use colored::Colorize;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum ColorSuite {
    Red,
    Yellow,
    Green,
    Blue
}

impl ColorSuite {
    /// Returns an array containing all colors except Undecided.
    pub fn all() -> [Self; 4] {
        [Self::Red, Self::Blue, Self::Green, Self::Yellow]
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

impl fmt::Display for ColorSuite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Red => write!(f, "{}", "Red".red()),
            Self::Blue => write!(f, "{}", "Blue".blue()),
            Self::Green => write!(f, "{}", "Green".green()),
            Self::Yellow => write!(f, "{}", "Yellow".yellow())
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Color {
    Standard(ColorSuite),
    Wild(Option<ColorSuite>)
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Standard(color) => write!(f, "{color}"),
            Self::Wild(option) => {
                write!(f, "{}{}{}{}", "W".red(), "i".blue(), "l".green(), "d".yellow());
                match option {
                    None => write!(f, ""),
                    Some(color) => write!(f, "{color}")
                }
            }
        }
    }
}
