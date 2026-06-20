use crate::domain::deck::Deck;
use crate::domain::round::{Round, RoundOutcome};

pub fn play_round(bet: i32, funds: i32, deck: &mut Deck) -> i32 {
    let mut round = match Round::new(deck) {
        Some(r) => r,
        None => {
            println!("Deck ran out of cards.");
            return 0;
        }
    };

    let player_bj = round.player_has_blackjack();
    let dealer_bj = round.dealer_has_blackjack();

    if player_bj || dealer_bj {
        let outcome = match (player_bj, dealer_bj) {
            (true, true) => RoundOutcome::BothBlackjack,
            (true, false) => RoundOutcome::PlayerBlackjack,
            (false, true) => RoundOutcome::DealerBlackjack,
            _ => unreachable!(),
        };
        let winnings = outcome.winnings(bet);
        match &outcome {
            RoundOutcome::PlayerBlackjack => println!("Blackjack! You win £{}", winnings),
            RoundOutcome::DealerBlackjack => println!("Dealer has blackjack! You lose £{}", bet),
            RoundOutcome::BothBlackjack => println!("Both blackjack — push!"),
            _ => unreachable!(),
        }
        return winnings;
    }

    // Player turn — loops through each hand (more than one if split)
    while round.current_hand_index() < round.hand_count() {
        loop {
            let hand_label = if round.hand_count() > 1 {
                format!("Hand {}", round.current_hand_index() + 1)
            } else {
                "Your hand".to_string()
            };

            if round.current_hand().is_bust() {
                println!("{} busts!", hand_label);
                break;
            }

            println!();
            println!("{} ({}):", hand_label, round.current_hand().value());
            for card in round.current_hand().cards() {
                println!("  {}", card.display());
            }
            println!(
                "Dealer shows: {}",
                round
                    .dealer_hand
                    .first_card()
                    .map(|c| c.display())
                    .unwrap_or_default()
            );

            let can_split = round.can_split();
            let can_double = round.can_double() && funds >= bet * 2;
            let prompt = match (can_split, can_double) {
                (true, true) => "Hit, stand, double, or split? (h/s/d/p)",
                (false, true) => "Hit, stand, or double? (h/s/d)",
                (true, false) => "Hit, stand, or split? (h/s/p)",
                (false, false) => "Hit or stand? (h/s)",
            };
            println!("{}", prompt);

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            match input.trim().to_lowercase().as_str() {
                "h" => {
                    if round.player_hit(deck).is_none() {
                        println!("Deck is empty!");
                        break;
                    }
                    println!(
                        "You drew: {}",
                        round.current_hand().cards().last().unwrap().display()
                    );
                }
                "s" => break,
                "d" if can_double => {
                    if round.player_double(deck).is_none() {
                        println!("Deck is empty!");
                        break;
                    }
                    println!(
                        "Doubled! Drew: {} — hand is now {}",
                        round.current_hand().cards().last().unwrap().display(),
                        round.current_hand().value()
                    );
                    break;
                }
                "p" if can_split => {
                    if round.split(deck).is_none() {
                        println!("Deck is empty!");
                        break;
                    }
                    println!("Split! Playing each hand in turn.");
                }
                _ => println!("Invalid choice."),
            }
        }

        round.advance_hand();
    }

    // Reveal dealer and resolve
    let any_alive = (0..round.hand_count()).any(|i| !round.player_hands[i].is_bust());
    if any_alive {
        println!();
        println!(
            "Dealer reveals: {}",
            round
                .dealer_hand
                .cards()
                .iter()
                .map(|c| c.display())
                .collect::<Vec<_>>()
                .join(" | ")
        );

        let initial_count = round.dealer_hand.cards().len();
        if round.resolve_dealer(deck).is_none() {
            println!("Deck ran out during dealer's turn.");
        }
        for card in &round.dealer_hand.cards()[initial_count..] {
            println!("Dealer draws: {}", card.display());
        }
        println!("Dealer stands at: {}", round.dealer_hand.value());
    }

    // Results
    println!();
    for i in 0..round.hand_count() {
        let label = if round.hand_count() > 1 {
            format!("Hand {}", i + 1)
        } else {
            "Result".to_string()
        };
        let effective_bet = if round.is_doubled(i) { bet * 2 } else { bet };
        match round.outcome_for_hand(i) {
            RoundOutcome::PlayerBust => println!("{}: Bust! You lose £{}", label, effective_bet),
            RoundOutcome::DealerBust => {
                println!("{}: Dealer busted! You win £{}", label, effective_bet)
            }
            RoundOutcome::PlayerWins => println!("{}: You win £{}", label, effective_bet),
            RoundOutcome::DealerWins => {
                println!("{}: Dealer wins. You lose £{}", label, effective_bet)
            }
            RoundOutcome::Tie => println!("{}: Push — bet returned.", label),
            _ => {}
        }
    }

    round.total_winnings(bet)
}
