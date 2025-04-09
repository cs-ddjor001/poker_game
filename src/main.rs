use itertools::Itertools;
use poker_game::deck::Suit;
use poker_game::deck::{Card, Deck};
use poker_game::hand_eval::Tier;

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
    println!("");
    println!(
        "Player 2's cards: [{}]",
        player2_hand
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("");
    println!(
        "Community cards: [{}]",
        community_cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("");
    println!("Evaluating best hands...");
    println!("");

    let player1_best_hand = evaluate_best_hand(&player1_hand, &community_cards);
    let player2_best_hand = evaluate_best_hand(&player2_hand, &community_cards);

    println!("Player 1's best hand: {}", player1_best_hand);
    println!("");
    println!("Player 2's best hand: {}", player2_best_hand);
    println!("");
    println!("Comparing hands...");
    println!("");

    match player1_best_hand.cmp(&player2_best_hand) {
        std::cmp::Ordering::Greater => {
            println!("Player 1 wins with {}!", player1_best_hand);
        }
        std::cmp::Ordering::Less => {
            println!("Player 2 wins with {}!", player2_best_hand);
        }
        std::cmp::Ordering::Equal => {
            let player1_high_card = get_high_card(&player1_best_hand);
            let player2_high_card = get_high_card(&player2_best_hand);

            match player1_high_card.cmp(&player2_high_card) {
                std::cmp::Ordering::Greater => {
                    println!(
                        "Player 1 wins with {} (high card: {})!",
                        player1_best_hand, player1_high_card
                    );
                }
                std::cmp::Ordering::Less => {
                    println!(
                        "Player 2 wins with {} (high card: {})!",
                        player2_best_hand, player2_high_card
                    );
                }
                std::cmp::Ordering::Equal => {
                    println!("It's a complete tie!");
                }
            }
        }
    }
}

fn evaluate_best_hand(player_cards: &[Card], community_cards: &[Card]) -> Tier {
    let mut best_tier = None;

    let all_cards = [player_cards, community_cards].concat();

    let combinations = all_cards.iter().cloned().combinations(5);

    for combo in combinations {
        let hand_array: [Card; 5] = combo.try_into().expect("Combination should have 5 cards");
        let tier = Tier::evaluate_hand(hand_array);

        if best_tier.is_none() || tier > best_tier.unwrap() {
            best_tier = Some(tier);
        }
    }

    best_tier.expect("There should always be a best hand")
}

fn get_high_card(tier: &Tier) -> Card {
    match tier {
        Tier::HighCard(rank) => Card::new(*rank, Suit::Spades),
        Tier::OnePair(rank) => Card::new(*rank, Suit::Spades),
        Tier::TwoPair(high, _) => Card::new(*high, Suit::Spades),
        Tier::ThreeOfAKind(rank) => Card::new(*rank, Suit::Spades),
        Tier::Straight(high, ..) => Card::new(*high, Suit::Spades),
        Tier::Flush(_) => panic!("Flush tie-breaking logic not implemented"),
        Tier::FullHouse(three, _) => Card::new(*three, Suit::Spades),
        Tier::FourOfAKind(rank) => Card::new(*rank, Suit::Spades),
        Tier::StraightFlush(high, ..) => Card::new(high.get_value(), Suit::Spades),
        Tier::RoyalFlush(_, _, _, _, high) => *high,
    }
}
