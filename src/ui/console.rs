use crate::domain::deck::Deck;
use crate::domain::round::{Round, RoundOutcome};

pub fn play_round(bet: i32, deck: &mut Deck) -> i32 {
    let mut round = Round::new(deck);

    if round.player_hand.is_blackjack() {
        println!(
            "You got a blackjack! You win £{}",
            RoundOutcome::PlayerBlackjack.winnings(bet)
        );
        return RoundOutcome::PlayerBlackjack.winnings(bet);
    }

    while !round.player_hand.is_bust() {
        println!();
        println!("Your hand ({:?}):", round.player_hand.value());
        for card in round.player_hand.cards() {
            println!("  {}", card.display());
        }
        println!();
        println!(
            "Dealer shows: {}",
            round
                .dealer_hand
                .first_card()
                .map(|c| c.display())
                .unwrap_or_default()
        );
        println!("Hit or stand? (h/s)");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        if choice.trim().to_lowercase() == "h" {
            round.player_hit(deck);
            println!(
                "You drew: {}",
                round.player_hand.cards().last().unwrap().display()
            );
        } else {
            break;
        }
    }

    if round.player_hand.is_bust() {
        let outcome = round.outcome();
        println!("You busted! You lose £{}", bet);
        return outcome.winnings(bet);
    }

    println!("Your final hand: {:?}", round.player_hand.value());
    round.resolve_dealer(deck);

    for card in round.dealer_hand.cards() {
        println!("Dealer drew: {}", card.display());
    }
    println!("Dealer's hand: {:?}", round.dealer_hand.value());

    let outcome = round.outcome();
    match &outcome {
        RoundOutcome::DealerBust => println!("Dealer busted! You win £{}", bet),
        RoundOutcome::PlayerWins => println!("You win! You win £{}", bet),
        RoundOutcome::DealerWins => println!("Dealer wins! You lose £{}", bet),
        RoundOutcome::Tie => println!("It's a tie! You get your bet back."),
        _ => {}
    }

    outcome.winnings(bet)
}
