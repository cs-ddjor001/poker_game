use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

const SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

impl Suit {
    pub fn values() -> &'static [Self] {
        &SUITS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardValue {
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

const CARD_VALUES: [CardValue; 13] = [
    CardValue::Two,
    CardValue::Three,
    CardValue::Four,
    CardValue::Five,
    CardValue::Six,
    CardValue::Seven,
    CardValue::Eight,
    CardValue::Nine,
    CardValue::Ten,
    CardValue::Jack,
    CardValue::Queen,
    CardValue::King,
    CardValue::Ace,
];

impl CardValue {
    pub fn values() -> &'static [Self] {
        &CARD_VALUES
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    value: CardValue,
    suit: Suit,
}

impl Card {
    pub fn new(value: CardValue, suit: Suit) -> Self {
        Self { value, suit }
    }

    pub fn get_value(&self) -> CardValue {
        self.value
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let deck = Suit::values()
            .iter()
            .flat_map(|&suit| {
                CardValue::values()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(CardValue::Ace, Suit::Spades);
        assert_that!(card.value, equal_to(CardValue::Ace));
        assert_that!(card.suit, equal_to(Suit::Spades));
        let card2 = Card::new(CardValue::King, Suit::Hearts);
        assert_that!(card2.value, equal_to(CardValue::King));
        assert_that!(card2.suit, equal_to(Suit::Hearts));
        assert_that!(card, not(equal_to(card2)));
    }

    #[test]
    fn test_deck_size() {
        let deck = Deck::new();
        assert_that!(deck.deck.len(), equal_to(52));
    }
}
