use rand::{seq::SliceRandom, thread_rng};

use crate::{
    card::Card,
    color::{Color, ColorSuite},
    hand::Hand,
    player::PlayResult,
    rules::Rules,
};

fn random(hand: &mut Hand, candidates: Vec<usize>) -> PlayResult {
    // This unwrap is safe right now because we
    let i = *candidates.choose(&mut thread_rng()).unwrap();
    let mut played = hand.remove(i);
    match played.color {
        Color::Wild(None) => played.assign_color(ColorSuite::random()),
        _ => {}
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
