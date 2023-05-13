use crate::card::Card;

pub struct Pile {
    stack: Vec<Card>,
}

impl Pile {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Returns the head and tail of the pile.
    /// This does not mutate the pile.
    fn head_tail(&self) -> (Card, Vec<Card>) {
        let head = self.stack[0];
        let tail = &self.stack[1..];
        return (head, tail.to_vec());
    }

    pub fn reduce_to_top(&mut self) -> Vec<Card> {
        let (head, tail) = self.head_tail();
        self.stack = vec![head];
        return tail;
    }

    pub fn add(&mut self, card: Card) {
        self.stack.push(card)
    }

    pub fn top(&self) -> &Card {
        return self.stack.last().unwrap();
    }
}
