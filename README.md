# Texas Hold'em Poker Game in Rust

## Overview

This project is a command-line implementation of **Texas Hold'em Poker** written in **Rust**, featuring core game mechanics such as:

- Card dealing and community card reveal
- Hand evaluation using standard poker rules
- Accurate winner determination and tie-breaking using kickers
- Expressive debug output showing best hands and evaluation steps

The project was designed to deepen my understanding of Rust, especially around enums, pattern matching, and idiomatic code architecture. It’s currently a two-player simulation, but the logic is extensible for multiplayer or AI opponents.

---

## Dependencies

Listed in `Cargo.toml`:

- [`rand`](https://crates.io/crates/rand) – For shuffling the deck
- [`itertools`](https://crates.io/crates/itertools) – For generating 5-card combinations
- ['hamcrest2'](https://crates.io/crates/hamcrest2) - For testing the implementation code

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

Thanks for checking out the project! This was a fun and challenging way to explore Rust’s strengths while building something game-related and logic-heavy.

Feel free to fork, test, or build upon this code!

---
