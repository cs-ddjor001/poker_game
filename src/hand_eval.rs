use crate::deck::{Card, CardValue, Suit};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub enum Tier {
    HighCard(CardValue),
    OnePair(CardValue),
    TwoPair(CardValue, CardValue),
    ThreeOfAKind(CardValue),
    Straight(CardValue, CardValue, CardValue, CardValue, CardValue),
    Flush(Suit),
    FullHouse(CardValue, CardValue),
    FourOfAKind(CardValue),
    StraightFlush(Card, Card, Card, Card, Card),
    RoyalFlush(Card, Card, Card, Card, Card),
}

impl Tier {
    fn based_on(hand: [Card; 5]) -> Tier {
        let mut card_counts = HashMap::new();
        let mut suit_counts = HashMap::new();

        let low_ace_straight: [CardValue; 5] = [
            CardValue::Two,
            CardValue::Three,
            CardValue::Four,
            CardValue::Five,
            CardValue::Ace,
        ];

        for card in hand.iter() {
            *card_counts.entry(card.get_value()).or_insert(0) += 1;
            *suit_counts.entry(card.get_suit()).or_insert(0) += 1;
        }

        let mut pairs: Vec<CardValue> = Vec::with_capacity(2);
        let mut trips: Option<CardValue> = None;
        let mut quads: Option<CardValue> = None;

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
            let mut card_values: Vec<CardValue> = card_counts.keys().cloned().collect();
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
                Card::new(CardValue::Ten, suits[0]),
                Card::new(CardValue::Jack, suits[0]),
                Card::new(CardValue::Queen, suits[0]),
                Card::new(CardValue::King, suits[0]),
                Card::new(CardValue::Ace, suits[0]),
            ];

            let low_ace_straight_flush: [Card; 5] = [
                Card::new(CardValue::Two, suits[0]),
                Card::new(CardValue::Three, suits[0]),
                Card::new(CardValue::Four, suits[0]),
                Card::new(CardValue::Five, suits[0]),
                Card::new(CardValue::Ace, suits[0]),
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
            let mut cards: Vec<CardValue> = card_counts.keys().cloned().collect();
            cards.sort();

            if cards == low_ace_straight {
                return Tier::Straight(
                    CardValue::Ace,
                    CardValue::Two,
                    CardValue::Three,
                    CardValue::Four,
                    CardValue::Five,
                );
            }
            if cards[4] as u8 - cards[0] as u8 == 4 {
                return Tier::Straight(cards[0], cards[1], cards[2], cards[3], cards[4]);
            }
        }

        if suit_counts.len() == 1 {
            let suits: Vec<Suit> = suit_counts.keys().cloned().collect();

            return Tier::Flush(suits[0]);
        }

        let mut card_values: Vec<CardValue> = card_counts.keys().cloned().collect();
        card_values.sort();
        return Tier::HighCard(card_values[card_values.len() - 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_high_card() {
        let ace_high: Tier = Tier::based_on([
            Card::new(CardValue::Ace, Suit::Clubs),
            Card::new(CardValue::Jack, Suit::Spades),
            Card::new(CardValue::Four, Suit::Diamonds),
            Card::new(CardValue::Eight, Suit::Diamonds),
            Card::new(CardValue::King, Suit::Hearts),
        ]);

        let ten_high: Tier = Tier::based_on([
            Card::new(CardValue::Two, Suit::Clubs),
            Card::new(CardValue::Five, Suit::Hearts),
            Card::new(CardValue::Ten, Suit::Diamonds),
            Card::new(CardValue::Three, Suit::Diamonds),
            Card::new(CardValue::Seven, Suit::Hearts),
        ]);

        let jack_high: Tier = Tier::based_on([
            Card::new(CardValue::Two, Suit::Clubs),
            Card::new(CardValue::Five, Suit::Hearts),
            Card::new(CardValue::Ten, Suit::Diamonds),
            Card::new(CardValue::Three, Suit::Diamonds),
            Card::new(CardValue::Jack, Suit::Hearts),
        ]);

        let king_high: Tier = Tier::based_on([
            Card::new(CardValue::Nine, Suit::Clubs),
            Card::new(CardValue::Seven, Suit::Hearts),
            Card::new(CardValue::Jack, Suit::Diamonds),
            Card::new(CardValue::King, Suit::Diamonds),
            Card::new(CardValue::Four, Suit::Hearts),
        ]);

        assert_that!(ace_high, is(equal_to(Tier::HighCard(CardValue::Ace))));
        assert_that!(ten_high, is(equal_to(Tier::HighCard(CardValue::Ten))));
        assert_that!(jack_high, is(equal_to(Tier::HighCard(CardValue::Jack))));
        assert_that!(king_high, is(equal_to(Tier::HighCard(CardValue::King))));
    }

    #[test]
    fn test_one_pair() {
        let ace_pair: Tier = Tier::based_on([
            Card::new(CardValue::Ace, Suit::Clubs),
            Card::new(CardValue::Ace, Suit::Spades),
            Card::new(CardValue::Four, Suit::Diamonds),
            Card::new(CardValue::Eight, Suit::Diamonds),
            Card::new(CardValue::King, Suit::Hearts),
        ]);

        let ten_pair: Tier = Tier::based_on([
            Card::new(CardValue::Two, Suit::Clubs),
            Card::new(CardValue::Five, Suit::Hearts),
            Card::new(CardValue::Ten, Suit::Diamonds),
            Card::new(CardValue::Three, Suit::Diamonds),
            Card::new(CardValue::Ten, Suit::Hearts),
        ]);

        assert_that!(ace_pair, is(equal_to(Tier::OnePair(CardValue::Ace))));
        assert_that!(ten_pair, is(equal_to(Tier::OnePair(CardValue::Ten))));
    }

    #[test]
    fn test_two_pair() {
        let ace_ten_two_pair: Tier = Tier::based_on([
            Card::new(CardValue::Ace, Suit::Clubs),
            Card::new(CardValue::Ace, Suit::Spades),
            Card::new(CardValue::Ten, Suit::Diamonds),
            Card::new(CardValue::Ten, Suit::Clubs),
            Card::new(CardValue::King, Suit::Hearts),
        ]);

        assert_that!(
            ace_ten_two_pair,
            is(equal_to(Tier::TwoPair(CardValue::Ten, CardValue::Ace)))
        );
    }

    #[test]
    fn test_three_of_a_kind() {
        let three_aces: Tier = Tier::based_on([
            Card::new(CardValue::Ace, Suit::Clubs),
            Card::new(CardValue::Ace, Suit::Spades),
            Card::new(CardValue::Ace, Suit::Hearts),
            Card::new(CardValue::Eight, Suit::Diamonds),
            Card::new(CardValue::King, Suit::Hearts),
        ]);

        assert_that!(three_aces, is(equal_to(Tier::ThreeOfAKind(CardValue::Ace))));
    }

    #[test]
    fn test_straight() {
        let straight: Tier = Tier::based_on([
            Card::new(CardValue::Six, Suit::Clubs),
            Card::new(CardValue::Seven, Suit::Spades),
            Card::new(CardValue::Ten, Suit::Hearts),
            Card::new(CardValue::Eight, Suit::Diamonds),
            Card::new(CardValue::Nine, Suit::Hearts),
        ]);

        let straight2: Tier = Tier::based_on([
            Card::new(CardValue::Ten, Suit::Clubs),
            Card::new(CardValue::Queen, Suit::Spades),
            Card::new(CardValue::Ace, Suit::Hearts),
            Card::new(CardValue::Jack, Suit::Diamonds),
            Card::new(CardValue::King, Suit::Hearts),
        ]);

        assert_that!(
            straight,
            equal_to(Tier::Straight(
                CardValue::Six,
                CardValue::Seven,
                CardValue::Eight,
                CardValue::Nine,
                CardValue::Ten
            ))
        );

        assert_that!(
            straight2,
            equal_to(Tier::Straight(
                CardValue::Ten,
                CardValue::Jack,
                CardValue::Queen,
                CardValue::King,
                CardValue::Ace
            ))
        );
    }

    #[test]
    fn test_ace_low_straight() {
        let ace_low_straight: Tier = Tier::based_on([
            Card::new(CardValue::Two, Suit::Clubs),
            Card::new(CardValue::Ace, Suit::Spades),
            Card::new(CardValue::Five, Suit::Hearts),
            Card::new(CardValue::Four, Suit::Diamonds),
            Card::new(CardValue::Three, Suit::Hearts),
        ]);

        assert_that!(
            ace_low_straight,
            equal_to(Tier::Straight(
                CardValue::Ace,
                CardValue::Two,
                CardValue::Three,
                CardValue::Four,
                CardValue::Five
            ))
        );
    }

    #[test]
    fn test_flush() {
        let flush1: Tier = Tier::based_on([
            Card::new(CardValue::Two, Suit::Hearts),
            Card::new(CardValue::King, Suit::Hearts),
            Card::new(CardValue::Nine, Suit::Hearts),
            Card::new(CardValue::Five, Suit::Hearts),
            Card::new(CardValue::Jack, Suit::Hearts),
        ]);

        let flush2: Tier = Tier::based_on([
            Card::new(CardValue::Three, Suit::Diamonds),
            Card::new(CardValue::King, Suit::Diamonds),
            Card::new(CardValue::Eight, Suit::Diamonds),
            Card::new(CardValue::Five, Suit::Diamonds),
            Card::new(CardValue::Jack, Suit::Diamonds),
        ]);

        let flush3: Tier = Tier::based_on([
            Card::new(CardValue::Ace, Suit::Spades),
            Card::new(CardValue::Four, Suit::Spades),
            Card::new(CardValue::Jack, Suit::Spades),
            Card::new(CardValue::Nine, Suit::Spades),
            Card::new(CardValue::Queen, Suit::Spades),
        ]);

        let flush4: Tier = Tier::based_on([
            Card::new(CardValue::Two, Suit::Clubs),
            Card::new(CardValue::Three, Suit::Clubs),
            Card::new(CardValue::Nine, Suit::Clubs),
            Card::new(CardValue::Ten, Suit::Clubs),
            Card::new(CardValue::Five, Suit::Clubs),
        ]);

        assert_that!(flush1, equal_to(Tier::Flush(Suit::Hearts)));
        assert_that!(flush2, equal_to(Tier::Flush(Suit::Diamonds)));
        assert_that!(flush3, equal_to(Tier::Flush(Suit::Spades)));
        assert_that!(flush4, equal_to(Tier::Flush(Suit::Clubs)));
    }

    #[test]
    fn test_full_house() {
        let full_house: Tier = Tier::based_on([
            Card::new(CardValue::Jack, Suit::Hearts),
            Card::new(CardValue::Nine, Suit::Spades),
            Card::new(CardValue::Nine, Suit::Clubs),
            Card::new(CardValue::Jack, Suit::Clubs),
            Card::new(CardValue::Nine, Suit::Diamonds),
        ]);

        assert_that!(
            full_house,
            equal_to(Tier::FullHouse(CardValue::Nine, CardValue::Jack))
        );
    }

    #[test]
    fn test_four_of_a_kind() {
        let four_aces: Tier = Tier::based_on([
            Card::new(CardValue::Ace, Suit::Clubs),
            Card::new(CardValue::Ace, Suit::Spades),
            Card::new(CardValue::Ace, Suit::Hearts),
            Card::new(CardValue::Ace, Suit::Diamonds),
            Card::new(CardValue::King, Suit::Hearts),
        ]);

        assert_that!(four_aces, is(equal_to(Tier::FourOfAKind(CardValue::Ace))));
    }

    #[test]
    fn test_straight_flush() {
        let straight_flush = Tier::based_on([
            Card::new(CardValue::Five, Suit::Clubs),
            Card::new(CardValue::Seven, Suit::Clubs),
            Card::new(CardValue::Nine, Suit::Clubs),
            Card::new(CardValue::Six, Suit::Clubs),
            Card::new(CardValue::Eight, Suit::Clubs),
        ]);

        assert_that!(
            straight_flush,
            is(equal_to(Tier::StraightFlush(
                Card::new(CardValue::Five, Suit::Clubs),
                Card::new(CardValue::Six, Suit::Clubs),
                Card::new(CardValue::Seven, Suit::Clubs),
                Card::new(CardValue::Eight, Suit::Clubs),
                Card::new(CardValue::Nine, Suit::Clubs),
            )))
        );
    }

    #[test]
    fn test_low_ace_straight_flush() {
        let straight_flush = Tier::based_on([
            Card::new(CardValue::Five, Suit::Diamonds),
            Card::new(CardValue::Two, Suit::Diamonds),
            Card::new(CardValue::Ace, Suit::Diamonds),
            Card::new(CardValue::Four, Suit::Diamonds),
            Card::new(CardValue::Three, Suit::Diamonds),
        ]);

        assert_that!(
            straight_flush,
            is(equal_to(Tier::StraightFlush(
                Card::new(CardValue::Two, Suit::Diamonds),
                Card::new(CardValue::Three, Suit::Diamonds),
                Card::new(CardValue::Four, Suit::Diamonds),
                Card::new(CardValue::Five, Suit::Diamonds),
                Card::new(CardValue::Ace, Suit::Diamonds),
            )))
        );
    }

    #[test]
    fn test_royal_flush() {
        let royal_flush = Tier::based_on([
            Card::new(CardValue::Ace, Suit::Clubs),
            Card::new(CardValue::King, Suit::Clubs),
            Card::new(CardValue::Queen, Suit::Clubs),
            Card::new(CardValue::Jack, Suit::Clubs),
            Card::new(CardValue::Ten, Suit::Clubs),
        ]);

        assert_that!(
            royal_flush,
            is(equal_to(Tier::RoyalFlush(
                Card::new(CardValue::Ten, Suit::Clubs),
                Card::new(CardValue::Jack, Suit::Clubs),
                Card::new(CardValue::Queen, Suit::Clubs),
                Card::new(CardValue::King, Suit::Clubs),
                Card::new(CardValue::Ace, Suit::Clubs),
            )))
        );
    }

    #[test]
    fn test_any_hand() {
        let hand_1 = Tier::based_on([
            Card::new(CardValue::Three, Suit::Diamonds),
            Card::new(CardValue::Ace, Suit::Diamonds),
            Card::new(CardValue::Three, Suit::Spades),
            Card::new(CardValue::Three, Suit::Clubs),
            Card::new(CardValue::Three, Suit::Hearts),
        ]);

        let hand_2 = Tier::based_on([
            Card::new(CardValue::Jack, Suit::Diamonds),
            Card::new(CardValue::Queen, Suit::Diamonds),
            Card::new(CardValue::Jack, Suit::Spades),
            Card::new(CardValue::Jack, Suit::Clubs),
            Card::new(CardValue::Queen, Suit::Hearts),
        ]);

        let hand_3 = Tier::based_on([
            Card::new(CardValue::Three, Suit::Diamonds),
            Card::new(CardValue::Seven, Suit::Diamonds),
            Card::new(CardValue::Six, Suit::Spades),
            Card::new(CardValue::Four, Suit::Clubs),
            Card::new(CardValue::Five, Suit::Hearts),
        ]);

        let hand_4 = Tier::based_on([
            Card::new(CardValue::Three, Suit::Spades),
            Card::new(CardValue::Seven, Suit::Spades),
            Card::new(CardValue::Six, Suit::Spades),
            Card::new(CardValue::Four, Suit::Spades),
            Card::new(CardValue::Five, Suit::Spades),
        ]);

        let hand_5 = Tier::based_on([
            Card::new(CardValue::Three, Suit::Spades),
            Card::new(CardValue::Three, Suit::Hearts),
            Card::new(CardValue::Jack, Suit::Spades),
            Card::new(CardValue::Jack, Suit::Diamonds),
            Card::new(CardValue::Three, Suit::Clubs),
        ]);

        let hand_6 = Tier::based_on([
            Card::new(CardValue::Three, Suit::Spades),
            Card::new(CardValue::Three, Suit::Hearts),
            Card::new(CardValue::Ace, Suit::Spades),
            Card::new(CardValue::King, Suit::Diamonds),
            Card::new(CardValue::Ten, Suit::Clubs),
        ]);

        let hand_7 = Tier::based_on([
            Card::new(CardValue::Three, Suit::Spades),
            Card::new(CardValue::Three, Suit::Hearts),
            Card::new(CardValue::Ace, Suit::Spades),
            Card::new(CardValue::King, Suit::Diamonds),
            Card::new(CardValue::King, Suit::Clubs),
        ]);

        let hand_8 = Tier::based_on([
            Card::new(CardValue::Three, Suit::Spades),
            Card::new(CardValue::Queen, Suit::Hearts),
            Card::new(CardValue::Ten, Suit::Spades),
            Card::new(CardValue::Seven, Suit::Diamonds),
            Card::new(CardValue::Six, Suit::Clubs),
        ]);

        let hand_9 = Tier::based_on([
            Card::new(CardValue::Three, Suit::Clubs),
            Card::new(CardValue::Queen, Suit::Clubs),
            Card::new(CardValue::Ten, Suit::Clubs),
            Card::new(CardValue::Seven, Suit::Clubs),
            Card::new(CardValue::Six, Suit::Clubs),
        ]);

        let hand_10 = Tier::based_on([
            Card::new(CardValue::Seven, Suit::Hearts),
            Card::new(CardValue::Queen, Suit::Clubs),
            Card::new(CardValue::Seven, Suit::Spades),
            Card::new(CardValue::Seven, Suit::Clubs),
            Card::new(CardValue::Six, Suit::Diamonds),
        ]);

        let hand_11 = Tier::based_on([
            Card::new(CardValue::Ten, Suit::Hearts),
            Card::new(CardValue::Jack, Suit::Hearts),
            Card::new(CardValue::Queen, Suit::Hearts),
            Card::new(CardValue::King, Suit::Hearts),
            Card::new(CardValue::Ace, Suit::Hearts),
        ]);

        assert_that!(hand_1, equal_to(Tier::FourOfAKind(CardValue::Three)));
        assert_that!(
            hand_2,
            equal_to(Tier::FullHouse(CardValue::Jack, CardValue::Queen))
        );
        assert_that!(
            hand_3,
            equal_to(Tier::Straight(
                CardValue::Three,
                CardValue::Four,
                CardValue::Five,
                CardValue::Six,
                CardValue::Seven
            ))
        );

        assert_that!(
            hand_4,
            equal_to(Tier::StraightFlush(
                Card::new(CardValue::Three, Suit::Spades),
                Card::new(CardValue::Four, Suit::Spades),
                Card::new(CardValue::Five, Suit::Spades),
                Card::new(CardValue::Six, Suit::Spades),
                Card::new(CardValue::Seven, Suit::Spades)
            ))
        );

        assert_that!(hand_5, equal_to(Tier::FullHouse(CardValue::Three, CardValue::Jack)));
        assert_that!(hand_6, equal_to(Tier::OnePair(CardValue::Three)));
        assert_that!(hand_7, equal_to(Tier::TwoPair(CardValue::Three, CardValue::King)));
        assert_that!(hand_8, equal_to(Tier::HighCard(CardValue::Queen)));
        assert_that!(hand_9, equal_to(Tier::Flush(Suit::Clubs)));
        assert_that!(hand_10, equal_to(Tier::ThreeOfAKind(CardValue::Seven)));
        assert_that!(
            hand_11,
            is(equal_to(Tier::RoyalFlush(
                Card::new(CardValue::Ten, Suit::Hearts),
                Card::new(CardValue::Jack, Suit::Hearts),
                Card::new(CardValue::Queen, Suit::Hearts),
                Card::new(CardValue::King, Suit::Hearts),
                Card::new(CardValue::Ace, Suit::Hearts),
            )))
        );
    }
}
