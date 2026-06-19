# blackjack-rust

![Status](https://img.shields.io/badge/Status-In%20Progress-yellow)
![CI](https://github.com/AndyXIP/blackjack-rust/actions/workflows/ci.yml/badge.svg)
![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust&logoColor=white)
![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/License-MIT-yellow)

A command-line Blackjack game written in Rust, following standard casino rules including double down, splitting, soft 17, and correct 3:2 blackjack payouts.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Running](#running)
- [How It Works](#how-it-works)
- [Development](#development)
  - [Running Tests](#running-tests)
  - [CI/CD](#cicd)
- [Future Enhancements](#future-enhancements)
- [License](#license)

---

## Overview

blackjack-rust is a fully-featured terminal Blackjack game built in Rust. It implements standard casino rules and is structured with a clean separation between pure game logic (`domain/`) and user interaction (`ui/`), making the core logic independently testable without any I/O mocking.

---

## Features

- **Standard Blackjack Rules** – Hit, stand, double down, and split
- **Dealer Blackjack Detection** – Round ends immediately if dealer has blackjack
- **Both Blackjack** – Push when both player and dealer have blackjack
- **Correct 3:2 Payout** – Blackjack pays 1.5× the bet
- **Soft 17** – Dealer hits on soft 17 (Ace + 6)
- **Double Down** – Double your bet on any initial two-card hand
- **Splitting** – Split any two equal-value cards into separate hands
- **Deck Safety** – Graceful handling if the deck runs out
- **Persistent Funds** – Track your bankroll across multiple rounds

---

## Tech Stack

- **Language:** Rust (Edition 2024)
- **Toolchain Manager:** [mise](https://mise.jdx.dev)
- **Dependencies:** [`rand`](https://crates.io/crates/rand) for deck shuffling
- **CI/CD:** GitHub Actions

---

## Project Structure

```
blackjack-rust/
├── src/
│   ├── main.rs              # Entry point — game loop and bankroll management
│   ├── lib.rs               # Public re-exports
│   ├── domain/              # Pure game logic — no I/O
│   │   ├── mod.rs
│   │   ├── card.rs          # Card, Suit, Rank types
│   │   ├── deck.rs          # Deck — shuffling and dealing
│   │   ├── hand.rs          # Hand — value calculation, bust, blackjack
│   │   └── round.rs         # Round state, outcomes, and winnings
│   └── ui/
│       ├── mod.rs
│       └── console.rs       # All I/O — prompts, display, driving the round
├── .github/
│   └── workflows/
│       └── ci.yml           # Check, test, fmt, clippy
├── Cargo.toml
├── mise.toml
└── README.md
```

---

## Getting Started

### Prerequisites

- [mise](https://mise.jdx.dev) — manages the Rust toolchain version

Install mise:
```sh
curl https://mise.run | sh
```

### Installation

```sh
git clone https://github.com/AndyXIP/blackjack-rust.git
cd blackjack-rust
mise install
```

### Running

```sh
cargo run
```

---

## How It Works

Each round follows standard casino Blackjack rules:

1. **Deal** — Player and dealer each receive two cards
2. **Blackjack check** — If either has blackjack, the round resolves immediately
3. **Player turn** — Hit, stand, double down, or split (if eligible)
   - Splitting creates two separate hands, each played in turn
   - Doubling takes exactly one more card and locks the hand
4. **Dealer turn** — Dealer reveals hole card and hits until 17+ (hits on soft 17)
5. **Resolution** — Hands are compared and winnings calculated

### Payouts

| Outcome | Payout |
|---|---|
| Blackjack | +1.5× bet (3:2) |
| Win | +1× bet |
| Push / Both blackjack | Bet returned |
| Loss / Bust | −1× bet |
| Double down win | +2× bet |

---

## Development

### Running Tests

```sh
cargo test
```

30 unit tests cover all domain logic — card values, hand totals, ace adjustment, bust detection, blackjack detection, soft 17, split eligibility, double bet calculation, and all round outcomes.

### CI/CD

GitHub Actions runs four checks on every push and pull request to `main`:

| Job | Command |
|---|---|
| Check | `cargo check` |
| Test | `cargo test` |
| Format | `cargo fmt --check` |
| Clippy | `cargo clippy -- -D warnings` |

Branch protection requires all four to pass before merging.

---

## Future Enhancements

- **Insurance** — Side bet when dealer shows an Ace
- **Surrender** — Forfeit half the bet on the initial hand
- **Multi-deck shoe** — Standard 6-deck casino shoe
- **Statistics** — Track win rate, earnings, and session history
- **TUI** — Terminal UI using [ratatui](https://github.com/ratatui-org/ratatui)

---

## License

This project is licensed under the MIT License.
