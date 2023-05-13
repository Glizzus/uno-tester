use std::fmt;

use crate::{card::Card, hand::Hand, rules::Rules, strategy::Strategy};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub hand: Hand,
    strategy: Strategy,
}

pub struct PlayResult {
    pub card: Option<Card>,
    pub was_last_card: bool,
}

impl PlayResult {
    pub fn new(card: Option<Card>, was_last_card: bool) -> Self {
        Self {
            card,
            was_last_card,
        }
    }
}

impl Player {
    pub fn new(id: u32, name: String, strategy: Strategy) -> Self {
        Self {
            id,
            hand: Hand::new(),
            strategy,
            name,
        }
    }

    pub fn play(&mut self, card: &Card, rules: &Rules, plus_stacking: bool) -> PlayResult {
        self.strategy
            .run(card, &mut self.hand, rules, plus_stacking)
    }

    pub fn proclaim_victory(&self) {
        println!(
            "{} has won using the {} strategy",
            self.name, self.strategy.name
        )
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) {}", self.id, self.name)
    }
}
