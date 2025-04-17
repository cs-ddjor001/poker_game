# Texas Hold'em Poker Game in Rust

## Overview

This project is a command-line implementation of **Texas Hold'em Poker** written in **Rust**, featuring core game mechanics such as:

- Card dealing and community card reveal
- Hand evaluation using standard poker rules
- Accurate winner determination and tie-breaking using kickers
- Expressive debug output showing best hands and evaluation steps

The project was designed to deepen my understanding of Rust, especially around enums, pattern matching, and idiomatic code architecture. It’s currently a two-player simulation, but the logic is extensible for multiplayer or AI opponents.

---

## How To Run The Program

The project doesn't expect any command line parameters and no interaction from the user at the moment. 

- Use ``` cargo run ``` to run the program from the command line.
- Use ``` cargo test ``` to run all the tests and display the results.

---

## Sample Program Output 

The project compares the hands between two players and determines a winner, or declares a tie of no winner is possible. At the start of the program, a deck of 52 cards is shuffled, after which 2 cards are dealt to player 1 and player 2 respetively. Following that, 5 community cards are revealed at the table, and the program determines the best possible hand for each player, from their 2 cards, and the 5 community cards. Each player's best hand is displayed to the terminal. Finally, the players hands are compared and a winner is declared, or a tie if the player's hands are the same.

A sample output from running ``` cargo run ```:

```

Player 1's cards: [7♣, Q♦]

Player 2's cards: [J♠, 6♥]

Community cards: [10♥, 4♣, K♠, 8♠, 6♠]

Evaluating best hands...

Player 1's best hand: High Card(K) with [Q♦, 10♥, K♠, 8♠, 7♣]

Player 2's best hand: One Pair(6) with [J♠, 6♥, K♠, 10♥, 6♠]

Comparing hands...

Player 2 wins with One Pair(6) [J♠, 6♥, K♠, 10♥, 6♠]!


```

---

## Dependencies

Listed in `Cargo.toml`:

- [`rand`](https://crates.io/crates/rand) – For shuffling the deck
- [`itertools`](https://crates.io/crates/itertools) – For generating 5-card combinations
- [`hamcrest2`](https://crates.io/crates/hamcrest2) - For testing the implementation code

---

## Changes Made During the Semester

- Refactored `deck.rs` into `card.rs` for clarity
- Built a `Tier` enum to represent all poker hand types
- Implemented `evaluate_best_hand()` to find the best 5-card hand from a 7-card set
- Fixed edge-case bugs in Two Pair and One Pair tie-breaker scenarios
- Added debug printing for best hands used in comparisons
- Implemented `Display` for clean output of cards and hand types

---

## Lessons Learned

- **Hand Evaluation Is Harder Than It Looks**: Managing edge cases like kickers and choosing the best hand among all 21 combinations was a great exercise in logic and testing.
- **Rust’s Type System Is Powerful**: Enums and pattern matching made the hand evaluation logic clean, safe, and expressive.
- **Tests Are Your Friend**: Creating test cases for each poker tier saved me from countless bugs and regressions.
- **Print Debugging Works**: Outputting the selected 5-card hands helped verify hand evaluation visually and catch subtle issues.

---

## What I’d Do with One More Month

If given more time, I'd expand the project in the following ways:

- Add **AI Opponents** with simple betting heuristics
- Build a **graphical interface** using a crate like [`ggez`](https://crates.io/crates/ggez) or [`bevy`](https://bevyengine.org)
- Add **multiplayer support** via TCP/WebSockets
- Implement a **hand history logger** tp keep track of the best hand in the player's history.
- Improve user interaction with input prompts and betting logic

---

This project was hard :(.

---
