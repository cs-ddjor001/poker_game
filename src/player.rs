use crate::card::{Card, Rank, Suit};
use std::fmt;

pub struct Player<'a> {
    pub name: &'a str,
    pub chips: u32,
    pub hand: Vec<Card>,
    pub is_playing: bool,
    pub is_small_blind: bool,
    pub is_big_blind: bool,
}

impl<'a> Player<'a> {
    pub fn new(name: &'a str, chips: u32) -> Self {
        Self {
            name,
            chips,
            hand: Vec::new(),
            is_playing: true,
            is_small_blind: false,
            is_big_blind: false,
        }
    }

    pub fn receive_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn raise(&mut self, amount: u32) {
        if self.chips >= amount {
            self.chips -= amount;
        }
    }

    pub fn fold(&mut self) {
        self.is_playing = false;
    }

    pub fn small_blind(&mut self) {
        self.is_small_blind = true;
        self.raise(25);
    }

    pub fn big_blind(&mut self) {
        self.is_big_blind = true;
        self.raise(50);
    }

    pub fn clear_hand(&mut self) {
        self.hand.clear();
    }

    pub fn get_hand_value(&self) -> Vec<Rank> {
        self.hand.iter().map(|card| card.get_rank()).collect()
    }

    pub fn get_hand_suits(&self) -> Vec<Suit> {
        self.hand.iter().map(|card| card.get_suit()).collect()
    }

    pub fn is_busted(&self) -> bool {
        self.chips == 0
    }
}

impl fmt::Display for Player<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (Chips: {})", self.name, self.chips)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Rank;

    #[test]
    fn test_player_new() {
        let player = Player::new("Dusan", 100);
        assert_eq!(player.name, "Dusan");
        assert_eq!(player.chips, 100);
        assert!(player.is_playing);
        assert!(!player.is_small_blind);
        assert!(!player.is_big_blind);
        assert_eq!(player.hand.len(), 0);
    }

    #[test]
    fn test_receive_card() {
        let mut player = Player::new("Dusan", 100);
        let card = Card::new(Rank::Ace, Suit::Spades);
        player.receive_card(card);
        assert_eq!(player.hand.len(), 1);
        assert_eq!(player.hand[0], card);
    }

    #[test]
    fn test_raise() {
        let mut player = Player::new("Dusan", 100);
        player.raise(30);
        assert_eq!(player.chips, 70);
    }

    #[test]
    fn test_fold() {
        let mut player = Player::new("Dusan", 100);
        player.fold();
        assert!(!player.is_playing);
    }

    #[test]
    fn test_small_blind() {
        let mut player = Player::new("Dusan", 100);
        player.small_blind();
        assert!(player.is_small_blind);
        assert_eq!(player.chips, 75);
    }

    #[test]
    fn test_big_blind() {
        let mut player = Player::new("Dusan", 100);
        player.big_blind();
        assert!(player.is_big_blind);
        assert_eq!(player.chips, 50);
    }

    #[test]
    fn test_clear_hand() {
        let mut player = Player::new("Dusan", 100);
        let card1 = Card::new(Rank::Ace, Suit::Spades);
        let card2 = Card::new(Rank::King, Suit::Hearts);
        player.receive_card(card1);
        player.receive_card(card2);
        assert_eq!(player.hand.len(), 2);
        player.clear_hand();
        assert_eq!(player.hand.len(), 0);
    }

    #[test]
    fn test_get_hand_value() {
        let mut player = Player::new("Dusan", 100);
        let card1 = Card::new(Rank::Ace, Suit::Spades);
        let card2 = Card::new(Rank::King, Suit::Hearts);
        player.receive_card(card1);
        player.receive_card(card2);
        let hand_values = player.get_hand_value();
        assert_eq!(hand_values, vec![Rank::Ace, Rank::King]);
    }

    #[test]
    fn test_is_busted() {
        let mut player = Player::new("Dusan", 100);
        assert!(!player.is_busted());
        player.raise(100);
        assert!(player.is_busted());
    }
}
