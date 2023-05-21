use crate::card::Card;

struct CardSequence<'a> {
    cards: Vec<&'a Card>,
}

impl<'a> CardSequence<'a> {
    pub fn special(&self) -> Self {
        let mut specials: Vec<&Card> = Vec::new();
        for card in &self.cards {
            if card.is_special() {
                specials.push(card);
            }
        }
        Self { cards: specials }
    }

    pub fn match_face(&self, other: &Card) -> Self {
        let mut matches = Vec::new();
        for &card in &self.cards {
            if card.face == other.face {
                matches.push(card)
            }
        }
        Self { cards: matches }
    }

    pub fn match_color(&self, other: &Card) -> Self {
        let mut matches = Vec::new();
        for &card in &self.cards {
            if card.color == other.color {
                matches.push(card)
            }
        }
        Self { cards: matches }
    }
}
