use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

const SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

impl Suit {
    pub fn values() -> &'static [Self] {
        &SUITS
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

const CARD_VALUES: [Rank; 13] = [
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];

impl Rank {
    pub fn values() -> &'static [Self] {
        &CARD_VALUES
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    value: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(value: Rank, suit: Suit) -> Self {
        Self { value, suit }
    }

    pub fn get_value(&self) -> Rank {
        self.value
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let deck = Suit::values()
            .iter()
            .flat_map(|&suit| {
                Rank::values()
                    .iter()
                    .map(move |&value| Card::new(value, suit))
            })
            .collect();

        Self { deck }
    }

    pub fn shuffle(&mut self) -> &mut Self {
        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
        self
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.deck.pop()
    }

    pub fn remaining_cards(&self) -> usize {
        self.deck.len()
    }

    pub fn deal(&mut self, count: usize) -> Result<Vec<Card>, &'static str> {
        if count > self.deck.len() {
            Err("Not enough cards in the deck")
        } else {
            Ok(self.deck.split_off(self.deck.len() - count))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        assert_that!(card.value, equal_to(Rank::Ace));
        assert_that!(card.suit, equal_to(Suit::Spades));
        let card2 = Card::new(Rank::King, Suit::Hearts);
        assert_that!(card2.value, equal_to(Rank::King));
        assert_that!(card2.suit, equal_to(Suit::Hearts));
        assert_that!(card, not(equal_to(card2)));
    }

    #[test]
    fn test_deck_size() {
        let deck = Deck::new();
        assert_that!(deck.deck.len(), equal_to(52));
    }
}
