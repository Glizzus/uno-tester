use crate::{
    card::Card,
    hand::Hand,
    player::PlayResult,
    rules::Rules, algorithms::naive,
};

use std::{
    hash::{self, Hash},
    ptr,
};

#[derive(Clone)]

pub struct Strategy {
    pub name: String,
    algorithm: fn(&Card, &mut Hand, &Rules, bool) -> PlayResult,
}

impl Strategy {
    pub fn naive() -> Self {
        Self {
            name: "Naive".to_owned(),
            algorithm: naive,
        }
    }

    pub fn get(name: String) -> Self {
        match name.to_lowercase().as_str() {
            "naive" => Self::naive(),
            _ => panic!("unknown strategy {}", name)
        }
    }

    pub fn run(
        &self,
        card: &Card,
        hand: &mut Hand,
        rules: &Rules,
        plus_stacking: bool,
    ) -> PlayResult {
        (self.algorithm)(card, hand, rules, plus_stacking)
    }
}

impl Eq for Strategy {}

impl PartialEq for Strategy {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.algorithm as *const (), other.algorithm as *const ())
    }
}

impl Hash for Strategy {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        ptr::hash(self.algorithm as *const (), state);
    }
}
