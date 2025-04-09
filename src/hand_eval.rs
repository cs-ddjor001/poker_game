use crate::deck::{Card, Rank, Suit};
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy)]
pub enum Tier {
    HighCard(Rank),
    OnePair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank, Rank, Rank, Rank, Rank),
    Flush(Suit),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank),
    StraightFlush(Card, Card, Card, Card, Card),
    RoyalFlush(Card, Card, Card, Card, Card),
}

impl Tier {
    pub fn evaluate_hand(hand: [Card; 5]) -> Tier {
        let mut card_counts = HashMap::new();
        let mut suit_counts = HashMap::new();

        let low_ace_straight: [Rank; 5] =
            [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace];

        for card in hand.iter() {
            *card_counts.entry(card.get_value()).or_insert(0) += 1;
            *suit_counts.entry(card.get_suit()).or_insert(0) += 1;
        }

        let mut pairs: Vec<Rank> = Vec::with_capacity(2);
        let mut trips: Option<Rank> = None;
        let mut quads: Option<Rank> = None;

        for (&card_value, &count) in &card_counts {
            match count {
                2 => pairs.push(card_value),
                3 => trips = Some(card_value),
                4 => quads = Some(card_value),
                _ => (),
            }
        }

        if pairs.len() == 1 {
            if let (Some(three), Some(two)) = (trips, pairs.first()) {
                return Tier::FullHouse(three, *two);
            }
            return Tier::OnePair(pairs[0]);
        }

        if pairs.len() == 2 {
            pairs.sort();
            return Tier::TwoPair(pairs[0], pairs[1]);
        }

        if let Some(set) = trips {
            return Tier::ThreeOfAKind(set);
        }

        if let Some(quad) = quads {
            return Tier::FourOfAKind(quad);
        }

        if card_counts.len() == 5 && suit_counts.len() == 1 {
            let suits: Vec<Suit> = suit_counts.keys().cloned().collect();
            let mut card_values: Vec<Rank> = card_counts.keys().cloned().collect();
            card_values.sort();

            let mut cards: Vec<Card> = vec![
                Card::new(card_values[0], suits[0]),
                Card::new(card_values[1], suits[0]),
                Card::new(card_values[2], suits[0]),
                Card::new(card_values[3], suits[0]),
                Card::new(card_values[4], suits[0]),
            ];

            cards.sort();

            let royal_flush: [Card; 5] = [
                Card::new(Rank::Ten, suits[0]),
                Card::new(Rank::Jack, suits[0]),
                Card::new(Rank::Queen, suits[0]),
                Card::new(Rank::King, suits[0]),
                Card::new(Rank::Ace, suits[0]),
            ];

            let low_ace_straight_flush: [Card; 5] = [
                Card::new(Rank::Two, suits[0]),
                Card::new(Rank::Three, suits[0]),
                Card::new(Rank::Four, suits[0]),
                Card::new(Rank::Five, suits[0]),
                Card::new(Rank::Ace, suits[0]),
            ];

            if cards == low_ace_straight_flush {
                return Tier::StraightFlush(cards[0], cards[1], cards[2], cards[3], cards[4]);
            }

            if cards == royal_flush {
                return Tier::RoyalFlush(cards[0], cards[1], cards[2], cards[3], cards[4]);
            }

            if cards[4].get_value() as u8 - cards[0].get_value() as u8 == 4 {
                return Tier::StraightFlush(cards[0], cards[1], cards[2], cards[3], cards[4]);
            }
        }

        if card_counts.len() == 5 {
            let mut cards: Vec<Rank> = card_counts.keys().cloned().collect();
            cards.sort();

            if cards == low_ace_straight {
                return Tier::Straight(Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five);
            }
            if cards[4] as u8 - cards[0] as u8 == 4 {
                return Tier::Straight(cards[0], cards[1], cards[2], cards[3], cards[4]);
            }
        }

        if suit_counts.len() == 1 {
            let suits: Vec<Suit> = suit_counts.keys().cloned().collect();

            return Tier::Flush(suits[0]);
        }

        let mut card_values: Vec<Rank> = card_counts.keys().cloned().collect();
        card_values.sort();
        return Tier::HighCard(card_values[card_values.len() - 1]);
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tier::HighCard(rank) => write!(f, "High Card({})", rank),
            Tier::OnePair(rank) => write!(f, "One Pair({})", rank),
            Tier::TwoPair(high, low) => write!(f, "Two Pair({}, {})", high, low),
            Tier::ThreeOfAKind(rank) => write!(f, "Three of a Kind({})", rank),
            Tier::Straight(a, b, c, d, e) => {
                write!(f, "Straight({}, {}, {}, {}, {})", a, b, c, d, e)
            }
            Tier::Flush(suit) => write!(f, "Flush in {}", suit),
            Tier::FullHouse(three, pair) => write!(f, "Full House({}, {})", three, pair),
            Tier::FourOfAKind(rank) => write!(f, "Four of a Kind({})", rank),
            Tier::StraightFlush(a, b, c, d, e) => {
                write!(f, "Straight Flush({}, {}, {}, {}, {})", a, b, c, d, e)
            }
            Tier::RoyalFlush(a, b, c, d, e) => {
                write!(f, "Royal Flush({}, {}, {}, {}, {})", a, b, c, d, e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_high_card() {
        let ace_high: Tier = Tier::evaluate_hand([
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Hearts),
        ]);

        let ten_high: Tier = Tier::evaluate_hand([
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Hearts),
        ]);

        let jack_high: Tier = Tier::evaluate_hand([
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Hearts),
        ]);

        let king_high: Tier = Tier::evaluate_hand([
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Hearts),
        ]);

        assert_that!(ace_high, is(equal_to(Tier::HighCard(Rank::Ace))));
        assert_that!(ten_high, is(equal_to(Tier::HighCard(Rank::Ten))));
        assert_that!(jack_high, is(equal_to(Tier::HighCard(Rank::Jack))));
        assert_that!(king_high, is(equal_to(Tier::HighCard(Rank::King))));
    }

    #[test]
    fn test_one_pair() {
        let ace_pair: Tier = Tier::evaluate_hand([
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Hearts),
        ]);

        let ten_pair: Tier = Tier::evaluate_hand([
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Ten, Suit::Hearts),
        ]);

        assert_that!(ace_pair, is(equal_to(Tier::OnePair(Rank::Ace))));
        assert_that!(ten_pair, is(equal_to(Tier::OnePair(Rank::Ten))));
    }

    #[test]
    fn test_two_pair() {
        let ace_ten_two_pair: Tier = Tier::evaluate_hand([
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
        ]);

        assert_that!(
            ace_ten_two_pair,
            is(equal_to(Tier::TwoPair(Rank::Ten, Rank::Ace)))
        );
    }

    #[test]
    fn test_three_of_a_kind() {
        let three_aces: Tier = Tier::evaluate_hand([
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Hearts),
        ]);

        assert_that!(three_aces, is(equal_to(Tier::ThreeOfAKind(Rank::Ace))));
    }

    #[test]
    fn test_straight() {
        let straight: Tier = Tier::evaluate_hand([
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Nine, Suit::Hearts),
        ]);

        let straight2: Tier = Tier::evaluate_hand([
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::King, Suit::Hearts),
        ]);

        assert_that!(
            straight,
            equal_to(Tier::Straight(
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten
            ))
        );

        assert_that!(
            straight2,
            equal_to(Tier::Straight(
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace
            ))
        );
    }

    #[test]
    fn test_ace_low_straight() {
        let ace_low_straight: Tier = Tier::evaluate_hand([
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Hearts),
        ]);

        assert_that!(
            ace_low_straight,
            equal_to(Tier::Straight(
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five
            ))
        );
    }

    #[test]
    fn test_flush() {
        let flush1: Tier = Tier::evaluate_hand([
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
        ]);

        let flush2: Tier = Tier::evaluate_hand([
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Diamonds),
        ]);

        let flush3: Tier = Tier::evaluate_hand([
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
        ]);

        let flush4: Tier = Tier::evaluate_hand([
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
        ]);

        assert_that!(flush1, equal_to(Tier::Flush(Suit::Hearts)));
        assert_that!(flush2, equal_to(Tier::Flush(Suit::Diamonds)));
        assert_that!(flush3, equal_to(Tier::Flush(Suit::Spades)));
        assert_that!(flush4, equal_to(Tier::Flush(Suit::Clubs)));
    }

    #[test]
    fn test_full_house() {
        let full_house: Tier = Tier::evaluate_hand([
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Diamonds),
        ]);

        assert_that!(
            full_house,
            equal_to(Tier::FullHouse(Rank::Nine, Rank::Jack))
        );
    }

    #[test]
    fn test_four_of_a_kind() {
        let four_aces: Tier = Tier::evaluate_hand([
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::King, Suit::Hearts),
        ]);

        assert_that!(four_aces, is(equal_to(Tier::FourOfAKind(Rank::Ace))));
    }

    #[test]
    fn test_straight_flush() {
        let straight_flush = Tier::evaluate_hand([
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ]);

        assert_that!(
            straight_flush,
            is(equal_to(Tier::StraightFlush(
                Card::new(Rank::Five, Suit::Clubs),
                Card::new(Rank::Six, Suit::Clubs),
                Card::new(Rank::Seven, Suit::Clubs),
                Card::new(Rank::Eight, Suit::Clubs),
                Card::new(Rank::Nine, Suit::Clubs),
            )))
        );
    }

    #[test]
    fn test_low_ace_straight_flush() {
        let straight_flush = Tier::evaluate_hand([
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
        ]);

        assert_that!(
            straight_flush,
            is(equal_to(Tier::StraightFlush(
                Card::new(Rank::Two, Suit::Diamonds),
                Card::new(Rank::Three, Suit::Diamonds),
                Card::new(Rank::Four, Suit::Diamonds),
                Card::new(Rank::Five, Suit::Diamonds),
                Card::new(Rank::Ace, Suit::Diamonds),
            )))
        );
    }

    #[test]
    fn test_royal_flush() {
        let royal_flush = Tier::evaluate_hand([
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
        ]);

        assert_that!(
            royal_flush,
            is(equal_to(Tier::RoyalFlush(
                Card::new(Rank::Ten, Suit::Clubs),
                Card::new(Rank::Jack, Suit::Clubs),
                Card::new(Rank::Queen, Suit::Clubs),
                Card::new(Rank::King, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Clubs),
            )))
        );
    }

    #[test]
    fn test_any_hand() {
        let hand_1 = Tier::evaluate_hand([
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Three, Suit::Hearts),
        ]);

        let hand_2 = Tier::evaluate_hand([
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Hearts),
        ]);

        let hand_3 = Tier::evaluate_hand([
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
        ]);

        let hand_4 = Tier::evaluate_hand([
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
        ]);

        let hand_5 = Tier::evaluate_hand([
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Clubs),
        ]);

        let hand_6 = Tier::evaluate_hand([
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Ten, Suit::Clubs),
        ]);

        let hand_7 = Tier::evaluate_hand([
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::King, Suit::Clubs),
        ]);

        let hand_8 = Tier::evaluate_hand([
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
        ]);

        let hand_9 = Tier::evaluate_hand([
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
        ]);

        let hand_10 = Tier::evaluate_hand([
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
        ]);

        let hand_11 = Tier::evaluate_hand([
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Hearts),
        ]);

        let hand_12 = Tier::evaluate_hand([
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
        ]);

        let hand_13 = Tier::evaluate_hand([
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Clubs),
        ]);

        let hand_14 = Tier::evaluate_hand([
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Diamonds),
        ]);

        assert_that!(hand_1, equal_to(Tier::FourOfAKind(Rank::Three)));
        assert_that!(hand_2, equal_to(Tier::FullHouse(Rank::Jack, Rank::Queen)));
        assert_that!(
            hand_3,
            equal_to(Tier::Straight(
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven
            ))
        );

        assert_that!(
            hand_4,
            equal_to(Tier::StraightFlush(
                Card::new(Rank::Three, Suit::Spades),
                Card::new(Rank::Four, Suit::Spades),
                Card::new(Rank::Five, Suit::Spades),
                Card::new(Rank::Six, Suit::Spades),
                Card::new(Rank::Seven, Suit::Spades)
            ))
        );

        assert_that!(hand_5, equal_to(Tier::FullHouse(Rank::Three, Rank::Jack)));
        assert_that!(hand_6, equal_to(Tier::OnePair(Rank::Three)));
        assert_that!(hand_7, equal_to(Tier::TwoPair(Rank::Three, Rank::King)));
        assert_that!(hand_8, equal_to(Tier::HighCard(Rank::Queen)));
        assert_that!(hand_9, equal_to(Tier::Flush(Suit::Clubs)));
        assert_that!(hand_10, equal_to(Tier::ThreeOfAKind(Rank::Seven)));
        assert_that!(
            hand_11,
            is(equal_to(Tier::RoyalFlush(
                Card::new(Rank::Ten, Suit::Hearts),
                Card::new(Rank::Jack, Suit::Hearts),
                Card::new(Rank::Queen, Suit::Hearts),
                Card::new(Rank::King, Suit::Hearts),
                Card::new(Rank::Ace, Suit::Hearts),
            )))
        );

        assert_that!(hand_12, equal_to(Tier::OnePair(Rank::Queen)));
        assert_that!(hand_13, equal_to(Tier::OnePair(Rank::Seven)));
        assert_that!(hand_14, equal_to(Tier::HighCard(Rank::Ace)));
    }
}
