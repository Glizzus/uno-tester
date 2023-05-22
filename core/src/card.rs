use core::panic;
use std::fmt;

use colored::Colorize;

use crate::color::{Color, ColorSuite};
use crate::face::Face;
use crate::rules::PlusStacking;

/// Represents an Uno Card
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Card {
    pub face: Face,
    pub color: Color,
}

impl Card {
    fn new(face: Face, color: Color) -> Self {
        Card { face, color }
    }

    /// Constructs a new wild card with the given face.
    pub fn new_wild(face: Face) -> Self {
        Self::new(face, Color::Wild(None))
    }

    /// Constructs a new card with the given color and face.
    pub fn new_colored(face: Face, color: ColorSuite) -> Self {
        Self::new(face, Color::Standard(color))
    }

    /// Returns whether this card stacks on another card.
    pub fn stacks_on(&self, other: &Card) -> bool {
        match self.color {
            // Wildcards always stack
            Color::Wild(None) => true,
            Color::Standard(self_color) | Color::Wild(Some(self_color)) => match other.color {
                Color::Wild(None) => panic!("attempt to stack on unassigned wild card"),
                Color::Wild(Some(other_color)) | Color::Standard(other_color) => {
                    self_color == other_color || self.face == other.face
                }
            },
        }
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
        match self.face {
            Face::Zero
            | Face::One
            | Face::Two
            | Face::Three
            | Face::Four
            | Face::Six
            | Face::Seven
            | Face::Eight
            | Face::Nine => true,
            _ => false,
        }
    }

    pub fn is_special(&self) -> bool {
        !self.is_num()
    }

    pub fn assign_color(&mut self, color: ColorSuite) {
        match self.color {
            Color::Wild(None) => self.color = Color::Wild(Some(color)),
            _ => panic!("Attempted to assign color to unassigned wild card {}", self),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.color, self.face)
    }
}
