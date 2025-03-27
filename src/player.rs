use crate::deck::Card;
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

    pub fn recieve_card(&mut self, card: Card) {
        self.hand.push(card)
    }

    pub fn raise(&mut self, amount: u32) {
        self.chips -= amount
    }

    pub fn fold(&mut self) {
        self.is_playing = false
    }

    pub fn small_blind(&mut self) {
        self.is_small_blind = true;
        self.raise(25);
    }

    pub fn big_blind(&mut self) {
        self.is_big_blind = true;
        self.raise(50);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_player_new() {
        let player = Player::new("Dusan", 20);
        assert_that!(player.name, equal_to("Dusan"));
        assert_that!(player.chips, equal_to(20));
        assert_that!(player.is_playing, equal_to(true));
        assert_that!(player.is_small_blind, equal_to(false));
        assert_that!(player.is_big_blind, equal_to(false));
        assert_that!(player.hand.len(), equal_to(0));
    }

    #[test]
    fn test_recieve_card() {
        let mut player = Player::new("Dusan", 20);
        assert_that!(player.name, equal_to("Dusan"));
        assert_that!(player.chips, equal_to(20));
        assert_that!(player.is_playing, equal_to(true));
        assert_that!(player.is_small_blind, equal_to(false));
        assert_that!(player.is_big_blind, equal_to(false));
        assert_that!(player.hand.len(), equal_to(0));
        let card1 = Card::new(crate::deck::Rank::Queen, crate::deck::Suit::Clubs);
        let card2 = Card::new(crate::deck::Rank::Jack, crate::deck::Suit::Hearts);
        player.recieve_card(card1);
        player.recieve_card(card2);
        assert_that!(player.hand.is_empty(), is(false));
        assert_that!(player.hand.len(), equal_to(2));
        assert_that!(player.hand.contains(&card1), is(true));
        assert_that!(player.hand.contains(&card2), is(true));
    }

    #[test]
    fn test_raise() {
        let mut player = Player::new("Dusan", 20);
        assert_that!(player.name, equal_to("Dusan"));
        assert_that!(player.chips, equal_to(20));
        assert_that!(player.is_playing, equal_to(true));
        assert_that!(player.is_small_blind, equal_to(false));
        assert_that!(player.is_big_blind, equal_to(false));
        assert_that!(player.hand.len(), equal_to(0));
        player.raise(10);
        assert_that!(player.chips, equal_to(10));
    }

    #[test]
    fn test_fold() {
        let mut player = Player::new("Dusan", 20);
        assert_that!(player.name, equal_to("Dusan"));
        assert_that!(player.chips, equal_to(20));
        assert_that!(player.is_playing, equal_to(true));
        assert_that!(player.is_small_blind, equal_to(false));
        assert_that!(player.is_big_blind, equal_to(false));
        assert_that!(player.hand.len(), equal_to(0));
        player.fold();
        assert_that!(player.is_playing, is(false));
    }

    #[test]
    fn test_small_blind() {
        let mut player = Player::new("Dusan", 100);
        assert_that!(player.name, equal_to("Dusan"));
        assert_that!(player.chips, equal_to(100));
        assert_that!(player.is_playing, equal_to(true));
        assert_that!(player.is_small_blind, equal_to(false));
        assert_that!(player.is_big_blind, equal_to(false));
        assert_that!(player.hand.len(), equal_to(0));
        player.small_blind();
        assert_that!(player.is_small_blind, equal_to(true));
        assert_that!(player.chips, equal_to(75));
    }

    #[test]
    fn test_big_blind() {
        let mut player = Player::new("Dusan", 100);
        assert_that!(player.name, equal_to("Dusan"));
        assert_that!(player.chips, equal_to(100));
        assert_that!(player.is_playing, equal_to(true));
        assert_that!(player.is_small_blind, equal_to(false));
        assert_that!(player.is_big_blind, equal_to(false));
        assert_that!(player.hand.len(), equal_to(0));
        player.big_blind();
        assert_that!(player.is_big_blind, equal_to(true));
        assert_that!(player.chips, equal_to(50));
    }
}
