use std::fmt;

use colored::Colorize;

use crate::color::Color;
use crate::face::Face;
use crate::rules::PlusStacking;

/// Represents an Uno Card
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Card {
    pub face: Face,
    pub color: Color,

    /// Indicates whether the card is a wild card.
    ///
    /// This remains true even after the wild card is
    /// assigned a value so that we can remember it was wild.
    pub is_wild: bool,
}

impl Card {
    fn new(face: Face, color: Color, is_wild: bool) -> Self {
        Card {
            face,
            color,
            is_wild,
        }
    }

    /// Constructs a new wild card with the given face.
    pub fn new_wild(face: Face) -> Self {
        Self::new(face, Color::Undecided, true)
    }

    /// Constructs a new card with the given color and face.
    pub fn new_colored(face: Face, color: Color) -> Self {
        Self::new(face, color, false)
    }

    /// Returns whether this card stacks on another card.
    pub fn stacks_on(&self, other: &Card) -> bool {
        self.color == other.color || self.face == other.face || self.is_wild
    }

    /// Returns whether this card plus stacks on another card.
    pub fn plus_stacks_on(&self, other: &Card, rules: &PlusStacking) -> bool {
        if !self.face.is_plus() {
            return false;
        }
        match rules {
            PlusStacking::Banned => false,
            PlusStacking::Liberal => self.face == Face::PlusFour || self.color == other.color,
            // This may be inefficient
            PlusStacking::Conservative => {
                other.face != Face::PlusFour || self.face != Face::PlusTwo
            }
        }
    }

    /// Returns the amount that this card increases a plus stack by.
    pub fn plus_stack_value(&self) -> usize {
        match self.face {
            Face::PlusFour => 4,
            Face::PlusTwo => 2,
            _ => 0,
        }
    }

    /// Indicates whether this card is a simple numeric card.
    pub fn is_num(&self) -> bool {
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
        ]
        .contains(&self.face)
    }

    pub fn is_special(&self) -> bool {
        !self.is_num()
    }

    /// Assigns a color to the given wild card.
    ///
    /// # Panic
    ///
    /// If the given card is not a wild card or its color is not `Undecided`
    pub fn assign_color(&mut self, color: Color) {
        assert!(self.is_wild, "Card is not a wild card!");
        assert!(
            self.color == Color::Undecided,
            "Wild Card does not have undecided color!"
        );
        self.color = color
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let face = format!("{}", self.face);
        let colored = match self.color {
            Color::Red => face.red(),
            Color::Yellow => face.yellow(),
            Color::Blue => face.blue(),
            Color::Green => face.green(),
            Color::Undecided => face.strikethrough(),
        };
        write!(f, "{}", colored)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_assign_wild() {
        let mut wild = Card::new_wild(Face::Wild);
        assert!(
            wild.color == Color::Undecided,
            "Newly created wildcard does not have undecided color"
        );
        wild.assign_color(Color::Blue);
        assert!(wild.color == Color::Blue, "Wild card was not made blue");
        assert!(wild.is_wild, "Wild card is no longer wild");
    }

    #[test]
    fn normal_stack_test() {
        let blue_two = Card::new_colored(Face::Two, Color::Blue);
        let red_two = Card::new_colored(Face::Two, Color::Red);
        let green_three = Card::new_colored(Face::Three, Color::Green);
        let blue_nine = Card::new_colored(Face::Nine, Color::Blue);

        let stack_on_blue_two = |card: &Card| card.stacks_on(&blue_two);

        assert!(
            stack_on_blue_two(&red_two),
            "Can't stack red two on blue two"
        );
        assert!(
            stack_on_blue_two(&blue_nine),
            "Can't stack blue nine on blue two"
        );
        assert!(
            !stack_on_blue_two(&green_three),
            "Stacked green three on blue two"
        );
    }

    #[test]
    fn plus_stack_test() {
        let blue_plus_two = Card::new_colored(Face::PlusTwo, Color::Blue);
        let blue_plus_four = Card::new_colored(Face::PlusFour, Color::Blue);

        let stack_plus_two_on_plus_four =
            |stack_rules: &PlusStacking| blue_plus_two.plus_stacks_on(&blue_plus_four, stack_rules);

        assert!(
            stack_plus_two_on_plus_four(&PlusStacking::Liberal),
            "Unable to stack plus two on plus four liberally"
        );
        assert!(
            !stack_plus_two_on_plus_four(&PlusStacking::Conservative),
            "Stacked plus two on plus four conservatively"
        );
        assert!(
            !stack_plus_two_on_plus_four(&PlusStacking::Banned),
            "Stacked plus two on plus four with stacking banned"
        );
    }
}
