use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn as_str(&self) -> &str {
        match *self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{}", symbol)
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn get_rank(&self) -> Rank {
        self.rank
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in &[
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
            ] {
                cards.push(Card::new(*rank, *suit));
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn remaining_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn deal(&mut self, count: usize) -> Result<Vec<Card>, &'static str> {
        if count > self.cards.len() {
            Err("Not enough cards in the deck")
        } else {
            Ok(self.cards.split_off(self.cards.len() - count))
        }
    }

    pub fn deal_to_players(
        &mut self,
        num_players: usize,
        cards_per_player: usize,
    ) -> Result<Vec<Vec<Card>>, &'static str> {
        if num_players * cards_per_player > self.cards.len() {
            return Err("Not enough cards in the deck");
        }

        let mut hands = Vec::with_capacity(num_players);
        for _ in 0..num_players {
            let hand = self.deal(cards_per_player)?;
            hands.push(hand);
        }

        Ok(hands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        assert_that!(card.rank, equal_to(Rank::Ace));
        assert_that!(card.suit, equal_to(Suit::Spades));
        let card2 = Card::new(Rank::King, Suit::Hearts);
        assert_that!(card2.rank, equal_to(Rank::King));
        assert_that!(card2.suit, equal_to(Suit::Hearts));
        assert_that!(card, not(equal_to(card2)));
    }

    #[test]
    fn test_deck_size() {
        let deck = Deck::new();
        assert_that!(deck.cards.len(), equal_to(52));
    }

    #[test]
    fn test_deal_to_players() {
        let mut deck = Deck::new();
        deck.shuffle();
        let hands = deck.deal_to_players(4, 5).unwrap();

        assert_that!(hands.len(), equal_to(4));
        assert_that!(hands[0].len(), equal_to(5));
    }
}
