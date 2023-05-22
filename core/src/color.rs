use std::fmt;

use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};

use colored::{ColoredString, Colorize};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum ColorSuite {
    Red,
    Yellow,
    Green,
    Blue,
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

    fn colorize<T>(&self) -> fn(T) -> ColoredString
    where
        T: Sized + Colorize,
    {
        match self {
            Self::Red => Colorize::red,
            Self::Yellow => Colorize::yellow,
            Self::Green => Colorize::green,
            Self::Blue => Colorize::blue,
        }
    }
}

impl fmt::Display for ColorSuite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Self::Red => "Red",
            Self::Blue => "Blue",
            Self::Green => "Green",
            Self::Yellow => "Yellow",
        };
        let colored = self.colorize()(str);
        write!(f, "{colored}")
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Color {
    Standard(ColorSuite),
    Wild(Option<ColorSuite>),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Standard(color) => write!(f, "{color}"),
            Self::Wild(None) => write!(f, "Wild"),
            Self::Wild(Some(c)) => {
                write!(f, "Wild {c}")
            }
        }
    }
}
