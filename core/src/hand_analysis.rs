use crate::{color::Color, hand::Hand};
/*
pub struct CardHistogram {
    hist: [u32; 4],
}

impl CardHistogram {
    fn new(hand: &Hand) -> Self {
        let mut hist = [0; 4];
        for card in hand {
            let i = card.color.to_num();
            hist[i as usize] += 1;
        }
        Self { hist }
    }

    /// Returns the color with the max amount of occurences in this histogram
    pub fn max(&self) -> (Color, u32) {
        let (i, val) = self
            .hist
            .iter()
            .enumerate()
            .max_by_key(|&(_, val)| val)
            .unwrap();
        (Color::from_num(self.hist[i]), *val)
    }
}

pub struct HandAnalysis<'a> {
    hand: &'a Hand,

    histogram: Option<CardHistogram>,
    histogram_built: bool,
}

impl<'a> HandAnalysis<'a> {
    pub fn new(hand: &'a Hand) -> Self {
        Self {
            hand,
            histogram: None,
            histogram_built: false,
        }
    }

    /// Returns a histogram from this HandAnalysis
    pub fn histogram(&mut self) -> &CardHistogram {
        if self.histogram.is_none() {
            let hist = CardHistogram::new(self.hand);
            self.histogram = Some(hist);
        }
        self.histogram.as_ref().unwrap()
    }
}
*/
