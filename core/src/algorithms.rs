use rand::{rngs::SmallRng, seq::SliceRandom, thread_rng, Rng, SeedableRng};

use crate::{card::Card, color::Color, hand::Hand, player::PlayResult, rules::Rules};

fn random(hand: &mut Hand, candidates: Vec<usize>) -> PlayResult {
    // This unwrap is safe right now because we
    let i = *candidates.choose(&mut thread_rng()).unwrap();
    let mut played = hand.remove(i);
    if played.is_wild {
        played.assign_color(Color::random())
    }
    PlayResult::new(Some(played), hand.is_empty())
}


/// The stupidest algorithm
pub fn naive(card: &Card, hand: &mut Hand, rules: &Rules, plus_stack: bool) -> PlayResult {
    let candidates = if plus_stack {
        hand.all_plus_stackable(card, &rules.plus_stacking)
    } else {
        hand.all_stackable_indices(card)
    };
    if candidates.len() == 0 {
        PlayResult::new(None, false)
    } else {
        random(hand, candidates)
    }
}