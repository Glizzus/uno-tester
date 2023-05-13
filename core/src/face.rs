use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Face {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Skip,
    Reverse,
    PlusTwo,
    Wild,
    PlusFour,
}

impl Face {
    pub fn is_plus(&self) -> bool {
        match self {
            Face::PlusTwo | Face::PlusFour => true,
            _ => false,
        }
    }

    /// Returns all faces that appear twice for each color
    /// in a standard Uno deck
    pub fn double_faces() -> [Self; 12] {
        [
            Face::One,
            Face::Two,
            Face::Three,
            Face::Four,
            Face::Five,
            Face::Six,
            Face::Seven,
            Face::Eight,
            Face::Nine,
            Face::Skip,
            Face::Reverse,
            Face::PlusTwo,
        ]
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            Self::Zero => "0",
            Self::One => "1",
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Skip => "🛇",
            Self::Reverse => "⤤ ⤦ ",
            Self::Wild => "Wild",
            Self::PlusTwo => "+2",
            Self::PlusFour => "+4",
        };
        write!(f, "{}", res)
    }
}
