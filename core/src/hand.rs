use std::{
    iter::Enumerate,
    slice::{self, Iter},
    vec,
};

use crate::{card::Card, rules::PlusStacking};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Constructs a new, empty hand
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    /// Returns every index of a card that stacks on the `target` card
    pub fn all_stackable_indices(&self, target: &Card) -> Vec<usize> {
        self.cards
            .iter()
            .enumerate()
            .filter_map(|(i, card)| card.stacks_on(target).then(|| (i)))
            .collect()
    }

    /// Returns every index of a card that plus stacks on the `target` card
    pub fn all_plus_stackable(&self, target: &Card, rules: &PlusStacking) -> Vec<usize> {
        self.enumerate()
            .filter_map(|(i, card)| card.plus_stacks_on(target, &rules).then(|| i))
            .collect()
    }

    fn enumerate(&self) -> Enumerate<Iter<'_, Card>> {
        self.cards.iter().enumerate()
    }

    /// Adds a card to this hand
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card)
    }

    /// Adds many cards to this hand
    pub fn add_many(&mut self, mut cards: Vec<Card>) {
        self.cards.append(&mut cards)
    }

    /// Indicates whether the current hand has no cards
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Removes the card at the given index using swap and pop (O(1))
    pub fn remove(&mut self, i: usize) -> Card {
        self.cards.swap_remove(i)
    }

    /// Returns a reference to the card at the given index
    pub fn get(&self, i: usize) -> &Card {
        &self.cards[i]
    }
}

impl IntoIterator for Hand {
    type Item = Card;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'a> IntoIterator for &'a Hand {
    type Item = &'a Card;
    type IntoIter = slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}
