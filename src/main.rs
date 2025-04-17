use itertools::Itertools;
use poker_game::card::{Card, Deck};
use poker_game::hand_eval::Tier;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BestHand {
    pub tier: Tier,
    pub cards: [Card; 5],
}

impl Ord for BestHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.tier.cmp(&other.tier) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
        }
    }
}

impl PartialOrd for BestHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let player1_hand = deck.deal(2).expect("Not enough cards for Player 1");
    let player2_hand = deck.deal(2).expect("Not enough cards for Player 2");

    let community_cards = deck.deal(5).expect("Not enough cards for community cards");

    println!(
        "Player 1's cards: [{}]",
        player1_hand
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();
    println!(
        "Player 2's cards: [{}]",
        player2_hand
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();
    println!(
        "Community cards: [{}]",
        community_cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();
    println!("Evaluating best hands...\n");

    let player1_best_hand = evaluate_best_hand(&player1_hand, &community_cards);
    let player1_best_tier = player1_best_hand.tier;
    let player1_best_hand_cards = player1_best_hand.cards;

    let player2_best_hand = evaluate_best_hand(&player2_hand, &community_cards);
    let player2_best_tier = player2_best_hand.tier;
    let player2_best_hand_cards = player2_best_hand.cards;

    println!(
        "Player 1's best hand: {} with [{}]",
        player1_best_tier,
        player1_best_hand_cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();
    println!(
        "Player 2's best hand: {} with [{}]",
        player2_best_tier,
        player2_best_hand_cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("\nComparing hands...\n");

    match player1_best_tier.cmp(&player2_best_tier) {
        std::cmp::Ordering::Greater => {
            println!(
                "Player 1 wins with {} [{}]!",
                player1_best_tier,
                player1_best_hand_cards
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        std::cmp::Ordering::Less => {
            println!(
                "Player 2 wins with {} [{}]!",
                player2_best_tier,
                player2_best_hand_cards
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        std::cmp::Ordering::Equal => match player1_best_hand_cards.cmp(&player2_best_hand_cards) {
            std::cmp::Ordering::Greater => {
                println!(
                    "Player 1 wins with {} [{}] (better kickers)!",
                    player1_best_tier,
                    player1_best_hand_cards
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            std::cmp::Ordering::Less => {
                println!(
                    "Player 2 wins with {} [{}] (better kickers)!",
                    player2_best_tier,
                    player2_best_hand_cards
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            std::cmp::Ordering::Equal => {
                println!(
                    "It's a complete tie! Both hands: [{}]",
                    player1_best_hand_cards
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        },
    }
}

pub fn evaluate_best_hand(player_cards: &[Card], community_cards: &[Card]) -> BestHand {
    let all_cards = [player_cards, community_cards].concat();
    let mut best_hand: Option<BestHand> = None;

    for combo in all_cards.iter().cloned().combinations(5) {
        let hand_array: [Card; 5] = combo.try_into().unwrap();
        let tier = Tier::evaluate_hand(hand_array);

        let current = BestHand {
            tier,
            cards: hand_array,
        };

        if best_hand.as_ref().map_or(true, |best| current > *best) {
            best_hand = Some(current);
        }
    }

    best_hand.expect("There should always be a best hand")
}
