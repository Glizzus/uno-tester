use std::{
    collections::{vec_deque, VecDeque},
    thread,
    time::Duration,
};

use crate::{
    card::Card,
    color::{Color, ColorSuite},
    deck::{Deck, DeckError},
    face::Face,
    pile::Pile,
    player::Player,
    reversible_ring::ReversibleRing,
    rules::Rules,
    strategy::Strategy,
};

/// Handles dealing operations using a `Deck`
struct Dealer {
    deck: Deck,
}

impl Dealer {
    /// Constructs a new `Dealer` with the given `Deck`
    pub fn new(deck: Deck) -> Self {
        Self { deck }
    }

    pub fn deal(&mut self, player: &mut Player) -> Result<(), DeckError> {
        let card = self.take()?;
        player.hand.add_card(card);
        Ok(())
    }

    pub fn deal_many(&mut self, player: &mut Player, n: usize) -> Result<(), DeckError> {
        let cards = self.deck.take_many(n)?;
        player.hand.add_many(cards);
        Ok(())
    }

    pub fn reshuffle(&mut self, pile: &mut Pile) {
        let pile_tail = pile.reduce_to_top();
        self.deck.replace_cards(pile_tail);
        self.deck.shuffle();
        self.deck.re_wild()
    }

    pub fn take(&mut self) -> Result<Card, DeckError> {
        self.deck.take()
    }

    pub fn peek(&self) -> Result<&Card, DeckError> {
        self.deck.peek()
    }
}

pub struct Game {
    player_handler: ReversibleRing<Player>,
    dealer: Dealer,
    pile: Pile,
    rules: Rules,

    stack_count: usize,
    verbose: bool,
}

impl Game {
    pub fn new(players: VecDeque<Player>, rules: Rules, verbose: bool) -> Self {
        Self {
            player_handler: ReversibleRing::from_iter(players),
            dealer: Dealer::new(Deck::new_shuffled()),
            pile: Pile::new(),
            rules,
            stack_count: 0,
            verbose,
        }
    }

    pub fn builder() -> GameBuilder {
        GameBuilder::default()
    }

    fn initial_deal(&mut self) {
        const UNO_HAND_SIZE: usize = 7;
        for player in self.player_handler.iter_mut() {
            self.dealer
                .deal_many(player, UNO_HAND_SIZE)
                .unwrap_or_else(|err| panic!("error during initial deal: {}", err))
        }
    }

    fn initialize_pile(&mut self) {
        loop {
            let top = self.dealer.peek().unwrap_or_else(|err| {
                panic!("error pulling card for pile initialization: {}", err)
            });

            // the game must not start on a plus four (https://www.unorules.com/)
            if top.face == Face::PlusFour {
                self.dealer.deck.shuffle();
                continue;
            }

            // We can safely unwrap here because we peeked earlier
            let mut top = self.dealer.take().unwrap();
            if top.face == Face::Wild {
                top.assign_color(ColorSuite::random())
            }
            self.pile.add(top);
            break;
        }
    }

    pub fn play_until_winner(&mut self) -> Player {
        // the game must be initalized
        self.dealer.deck.shuffle();
        self.initial_deal();

        self.initialize_pile();

        // main game loop
        loop {
            let mut player = self.player_handler.next();

            if self.stack_count > 0 {
                let top = self.pile.top();
                let result = player.play(top, &self.rules, true);
                match result.card {
                    None => {
                        //println!("{} has to draw {}", player, self.stack_count);
                        loop {
                            match self.dealer.deal_many(player, self.stack_count) {
                                Ok(()) => break,
                                Err(e) => {
                                    //println!("Deck Empty: Reshuffling");
                                    self.dealer.reshuffle(&mut self.pile);
                                }
                            }
                        }
                        self.stack_count = 0;
                    }
                    Some(c) => {
                        self.pile.add(c);
                        //println!("{} stacked a {}", player, c);
                        self.stack_count += c.plus_stack_value();
                        if result.was_last_card {
                            //player.proclaim_victory();
                            return player.clone();
                        }
                    }
                }
                continue;
            }

            let top = self.pile.top();
            let result = player.play(top, &self.rules, false);
            // handle card logic
            match result.card {
                Some(c) => {
                    self.pile.add(c);
                    //println!("{} played {}", player, c);
                    if result.was_last_card {
                        //player.proclaim_victory();
                        return player.clone();
                    }
                    match c.face {
                        Face::Reverse => self.player_handler.reverse(),
                        Face::Skip => self.player_handler.skip(),
                        Face::PlusTwo | Face::PlusFour => self.stack_count += c.plus_stack_value(),
                        _ => {}
                    }
                }
                None => {
                    //println!("{} couldn't play and had to draw", player);
                    loop {
                        match self.dealer.deal(&mut player) {
                            Ok(()) => break,
                            Err(e) => {
                                //println!("Deck Empty: Reshuffling...");
                                self.dealer.reshuffle(&mut self.pile);
                            }
                        }
                    }
                }
            }
            //thread::sleep(Duration::from_millis(500))
        }
    }
}

#[derive(Default)]
pub struct GameBuilder {
    players: Vec<Player>,
    rules: Option<Rules>,
}
