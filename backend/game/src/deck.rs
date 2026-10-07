use rand::seq::SliceRandom;
use super::card::*;
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::with_capacity(64);
        for suit in &[CardSuit::Hearts, CardSuit::Diamonds, CardSuit::Clubs, CardSuit::Spades] {
            for rank in &[
                CardRank::Ace,
                CardRank::Two,
                CardRank::Three,
                CardRank::Four,
                CardRank::Five,
                CardRank::Six,
                CardRank::Seven,
                CardRank::Eight,
                CardRank::Nine,
                CardRank::Ten,
                CardRank::Jack,
                CardRank::Queen,
                CardRank::King,
            ] {
                cards.push(Card {
                    suit: suit.clone(),
                    rank: rank.clone(),
                });
            }
        }
        Deck { cards }
    }
}

impl Deck {
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn reset(&mut self) {
        *self = Deck::default();
    }
}