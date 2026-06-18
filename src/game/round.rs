use super::deck::Deck;
use super::hand::Hand;

pub fn play_round(bet: i32, deck: &mut Deck) -> i32 {
    let mut player_hand = Hand::new();
    let mut dealer_hand = Hand::new();
    player_hand.hit(deck.deal().unwrap());
    dealer_hand.hit(deck.deal().unwrap());
    player_hand.hit(deck.deal().unwrap());
    dealer_hand.hit(deck.deal().unwrap());

    if player_hand.is_blackjack() {
        println!("You got a blackjack! You win £{}", bet);
        return (bet as f64 * 2.5) as i32;
    }

    while !player_hand.is_bust() {
        println!();
        println!("Your hand: {:?}", player_hand.value());
        println!(
            "Your hand: {:?}",
            player_hand
                .cards()
                .iter()
                .map(|card| card.display())
                .collect::<Vec<String>>()
        );
        println!();
        println!(
            "Dealer's hand: {:?}",
            dealer_hand
                .first_card()
                .map(|card| card.display())
                .unwrap_or("Unknown".to_string())
        );
        println!("Do you want to hit or stand? (h/s)");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        if choice.trim().to_lowercase() == "h" {
            player_hand.hit(deck.deal().unwrap());
            println!(
                "You drew: {:?}",
                player_hand.cards().last().unwrap().display()
            );
        } else {
            break;
        }
    }

    if player_hand.is_bust() {
        println!("You busted! You lose £{}", bet);
        -bet
    } else {
        println!("Your final hand: {:?}", player_hand.value());
        println!("Dealer's hand: {:?}", dealer_hand.value());
        while dealer_hand.value() < 17 {
            dealer_hand.hit(deck.deal().unwrap());
            println!(
                "Dealer drew: {:?}",
                dealer_hand.cards().last().unwrap().display()
            );
            println!("Dealer's hand: {:?}", dealer_hand.value());
        }

        if dealer_hand.is_bust() {
            println!("Dealer busted! You win £{}", bet);
            bet
        } else if player_hand.value() > dealer_hand.value() {
            println!("You win! You win £{}", bet);
            bet
        } else if player_hand.value() < dealer_hand.value() {
            println!("Dealer wins! You lose £{}", bet);
            -bet
        } else {
            println!("It's a tie! You get your bet back.");
            0
        }
    }
}
