use core::fmt;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{card::Card, color::{Color, ColorSuite}, face::Face};

#[derive(Debug)]
pub enum DeckError {
    NotEnoughCards(usize, usize),
}

impl Display for DeckError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NotEnoughCards(has, requested) => {
                write!(f, "requested {} cards, deck only has {}", has, requested)
            }
        }
    }
}

impl Error for DeckError {}

/// Represents the Uno deck that cards will be pulled from
pub struct Deck {
    stack: Vec<Card>,

    rng: SmallRng,
}

impl Deck {
    /// Constructs a new standard unshuffled Uno deck
    pub fn new() -> Self {
        let mut stack = Vec::new();
        for color in ColorSuite::all() {
            stack.push(Card::new_colored(Face::Zero, color));
            for face in Face::double_faces() {
                for _ in 0..2 {
                    stack.push(Card::new_colored(face, color));
                }
            }
        }
        for _ in 0..4 {
            for wild_face in [Face::Wild, Face::PlusFour] {
                stack.push(Card::new_wild(wild_face))
            }
        }
        Self {
            stack,
            rng: SmallRng::from_entropy(),
        }
    }

    /// Constructs a new standard shuffled Uno deck
    pub fn new_shuffled() -> Self {
        let mut deck = Self::new();
        deck.shuffle();
        deck
    }

    /// Shuffles the Deck
    pub fn shuffle(&mut self) {
        self.stack.shuffle(&mut self.rng);
    }

    /// Removes a card from the top of the deck and returns it.
    ///
    /// # Returns
    ///
    /// `Ok(Card)` if the deck has a card to remove.
    ///
    /// `Err(DeckError)` if the deck is empty.
    pub fn take(&mut self) -> Result<Card, DeckError> {
        self.stack.pop().ok_or(DeckError::NotEnoughCards(0, 1))
    }

    /// Removes `n` cards from the top of the deck and returns them as a vector.
    /// This uses Vec::split_off, and is not merely `take` called in a loop.
    ///
    /// # Returns
    ///
    /// `Ok(card)` if the deck is able to return `n` cards.
    ///
    /// `Err(DeckError)` if the deck does not have `n` cards to remove.
    pub fn take_many(&mut self, n: usize) -> Result<Vec<Card>, DeckError> {
        let len = self.stack.len();
        if n > len {
            return Err(DeckError::NotEnoughCards(len, n));
        }
        Ok(self.stack.split_off(len - n))
    }

    /// Replaces all cards with the given cards.
    pub fn replace_cards(&mut self, cards: Vec<Card>) {
        self.stack = cards;
    }

    /// Indicates whether the deck contains no cards.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Returns a borrow to the top card is without removing it.
    ///
    /// # Returns
    ///
    /// `Ok(&Card)` if the deck has a card.
    ///
    /// `Err(DeckError)` if the deck does not have a card.
    pub fn peek(&self) -> Result<&Card, DeckError> {
        self.stack.last().ok_or(DeckError::NotEnoughCards(0, 1))
    }

    pub fn re_wild(&mut self) {
        for card in self.stack.iter_mut() {
            match card.color {
                Color::Wild(Some(_)) => card.color = Color::Wild(None),
                _ => {}
            }
        }
    }
}
